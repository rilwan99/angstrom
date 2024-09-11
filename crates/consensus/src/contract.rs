use std::{borrow::Borrow, collections::HashMap};

use angstrom_types::{
    consensus::Proposal,
    contract_payloads::{
        angstrom::{AngstromBundle, OrderQuantities, Pair, PoolUpdate, TopOfBlockOrder, UserOrder},
        tob::Asset
    },
    matching::SqrtPriceX96,
    orders::{NetAmmOrder, OrderFillState, OrderOutcome},
    sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData}
};
use matching_engine::{
    cfmm::uniswap::{tob::calculate_reward, MarketSnapshot},
    MatchingManager
};
use reth_primitives::Address;
use tracing::warn;
use validation::order::state::pools::AngstromPoolsTracker;

#[derive(Default, Debug, Clone)]
struct BorrowStateTracker {
    borrow_needed: u128,
    amount_onhand: u128,
    amount_owed:   u128
}

impl BorrowStateTracker {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn allocate(&mut self, q: u128) {
        if q > self.amount_onhand {
            self.borrow_needed += (q - self.amount_onhand);
        }
        self.amount_onhand = self.amount_onhand.saturating_sub(q);
    }

    pub fn and_then(&self, other: &Self) -> Self {
        let borrow_needed =
            self.borrow_needed + (other.borrow_needed.saturating_sub(self.amount_onhand));
        let amount_onhand =
            (self.amount_onhand.saturating_sub(other.borrow_needed)) + other.amount_onhand;
        let amount_owed = self.amount_owed + other.amount_owed;
        Self { borrow_needed, amount_onhand, amount_owed }
    }
}

#[derive(Debug, Default)]
struct StageTracker {
    map: HashMap<Address, BorrowStateTracker>
}

impl StageTracker {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn and_then(&self, other: &Self) -> Self {
        let mut new_map = self.map.clone();
        other.map.iter().for_each(|(addr, state)| {
            new_map
                .entry(*addr)
                .and_modify(|e| *e = e.and_then(state))
                .or_insert_with(|| state.clone());
        });
        Self { map: new_map }
    }
}

struct AssetBuilder {}

struct AssetArray {
    assets:     Vec<Asset>,
    assets_idx: HashMap<Address, usize>
}

impl AssetArray {
    pub fn new() -> Self {
        Self { assets: Vec::new(), assets_idx: HashMap::new() }
    }

    pub fn add_or_get_asset(&mut self, asset: Address) -> usize {
        *self.assets_idx.entry(asset).or_insert_with(|| {
            self.assets
                .push(Asset { addr: asset, borrow: 0, save: 0, settle: 0 });
            self.assets.len() - 1
        })
    }

    pub fn to_asset_array(self) -> Vec<Asset> {
        self.assets
    }
}

pub fn to_contract_format(proposal: &Proposal, pools: &AngstromPoolsTracker) -> AngstromBundle {
    let mut top_of_block_orders = Vec::new();
    let mut pool_updates = Vec::new();
    let mut pairs = Vec::new();
    let mut user_orders = Vec::new();
    let mut assets = AssetArray::new();

    // Break out our input orders into lists of orders by pool
    let orders_by_pool = MatchingManager::orders_by_pool_id(&proposal.preproposals);

    // Walk through our solutions to add them to the structure
    for solution in proposal.solutions.iter() {
        // Get the information for the pool or skip this solution if we can't find a
        // pool for it
        let Some((t0, t1)) = pools.get_pool_addresses(solution.id) else {
            // This should never happen but let's handle it as gracefully as possible
            warn!("Skipped a solution as we couldn't find a pool for it: {:?}", solution);
            continue;
        };
        // Make sure the involved assets are in our assets array and we have the
        // appropriate asset index for them
        let t0_idx = assets.add_or_get_asset(t0) as u16;
        let t1_idx = assets.add_or_get_asset(t1) as u16;
        // Build our Pair featuring our uniform clearing price
        // Need to make sure this is a Ray
        let uniswap_price = solution.ucp;
        pairs.push(Pair { t0_idx, t1_idx, uniswap_price });
        let pair_idx = pairs.len() - 1;
        // Add the net AMM order to our swaps
        let (asset_in_index, asset_out_index, swap_in_quantity) = solution
            .amm_quantity
            .as_ref()
            .map(|amm_o| {
                let (asset_in_index, asset_out_index, quantity) = match amm_o {
                    NetAmmOrder::Buy(q, c) => (t1_idx, t0_idx, q),
                    NetAmmOrder::Sell(q, c) => (t0_idx, t1_idx, q)
                };
                let swap_quantity_in: u128 = quantity.to();
                (asset_in_index, asset_out_index, swap_quantity_in)
            })
            .unwrap_or((0, 0, 0));
        // If we have a ToB order, calculate our pool reward and go from there
        let rewards_update = solution
            .searcher
            .as_ref()
            .map(|tob| {
                let amm = MarketSnapshot::new(vec![], SqrtPriceX96::default()).unwrap();
                calculate_reward(tob, amm)
                    .unwrap()
                    .to_donate(t0_idx, t1_idx)
                    .update
            })
            .unwrap_or_default();
        pool_updates.push(PoolUpdate {
            asset_in_index,
            asset_out_index,
            swap_in_quantity,
            rewards_update
        });
        // Add the ToB order to our tob order list - This is currently converting
        // between two ToB order formats
        if let Some(tob) = solution.searcher.as_ref() {
            let contract_tob = TopOfBlockOrder::of(tob);
            top_of_block_orders.push(contract_tob);
        }

        // Get our list of user orders, if we have any
        let order_list: Vec<&OrderWithStorageData<GroupedVanillaOrder>> = orders_by_pool
            .get(&solution.id)
            .map(|order_set| order_set.into_iter().collect())
            .unwrap_or_default();
        // Sort the user order list
        for (outcome, order) in solution.limit.iter().zip(order_list.iter()) {
            let pair_index = (pairs.len() - 1) as u16;
            user_orders.push(to_contract_order(order, outcome, pair_index));
        }
    }
    AngstromBundle::new(
        assets.to_asset_array(),
        pairs,
        pool_updates,
        top_of_block_orders,
        user_orders
    )
}

pub fn to_contract_order(
    order: &OrderWithStorageData<GroupedVanillaOrder>,
    outcome: &OrderOutcome,
    pair_index: u16
) -> UserOrder {
    let order_quantities = match order.order {
        GroupedVanillaOrder::KillOrFill(ref o) => OrderQuantities::Exact {
            quantity: u128::from_le_bytes(o.max_amount_in_or_out.to_le_bytes())
        },
        GroupedVanillaOrder::Partial(ref o) => {
            let max_quantity_in: u128 = u128::from_le_bytes(o.max_amount_in_or_out.to_le_bytes());
            let filled_quantity = match outcome.outcome {
                OrderFillState::CompleteFill => max_quantity_in,
                OrderFillState::PartialFill(fill) => u128::from_le_bytes(fill.to_le_bytes()),
                _ => 0
            };
            OrderQuantities::Partial { min_quantity_in: 0, max_quantity_in, filled_quantity }
        }
    };
    let hook_data = match order.order {
        GroupedVanillaOrder::KillOrFill(ref o) => o.hook_data.clone(),
        GroupedVanillaOrder::Partial(ref o) => o.hook_data.clone()
    };
    UserOrder {
        a_to_b: order.is_bid,
        exact_in: false,
        hook_data: Some(hook_data),
        min_price: *order.price(),
        order_quantities,
        pair_index,
        recipient: None,
        signature: order.signature().clone(),
        standing_validation: None,
        use_internal: false
    }
}
