use std::collections::HashMap;

use angstrom_types::{
    consensus::Proposal,
    contract_payloads::angstrom::{AngstromBundle, OrderQuantities, Pair, PoolSwap, UserOrder},
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

pub fn to_contract_format(proposal: &Proposal, pools: &AngstromPoolsTracker) -> AngstromBundle {
    let mut tob_orders = Vec::new();
    let mut swaps = Vec::new();
    let mut pairs = Vec::new();
    let mut user_orders = Vec::new();
    let mut pool_rewards = Vec::new();
    // Index of asset vector to avoid scanning the vec repeatedly
    let mut asset_idx: HashMap<Address, usize> = HashMap::new();
    // Actual asset vector
    let mut assets = Vec::new();

    // Break out our input orders into lists of orders by pool
    let orders_by_pool = MatchingManager::orders_by_pool_id(&proposal.preproposals);

    for solution in proposal.solutions.iter() {
        // Get the information for the pool or skip this solution if we can't find a
        // pool for it
        let Some((t0, t1)) = pools.get_pool_data(solution.id).map(|(first, second)| {
            if first < second {
                (first, second)
            } else {
                (second, first)
            }
        }) else {
            warn!("Skipped a solution as we couldn't find a pool for it: {:?}", solution);
            continue;
        };
        // Make sure the involved assets are in our assets array and we have the
        // appropriate asset index for them
        let t0_idx = *asset_idx.entry(t0).or_insert_with(|| {
            assets.push(t0);
            assets.len() - 1
        }) as u16;
        let t1_idx = *asset_idx.entry(t1).or_insert_with(|| {
            assets.push(t1);
            assets.len() - 1
        }) as u16;
        // Build our Pair featuring our uniform clearing price
        let uniswap_price = solution.ucp;
        pairs.push(Pair { t0_idx, t1_idx, uniswap_price });
        // Add the net AMM order to our swaps
        if let Some(amm_order) = solution.amm_quantity.as_ref() {
            let (asset_in_index, asset_out_index, quantity) = match amm_order {
                NetAmmOrder::Buy(q) => (t1_idx, t0_idx, q),
                NetAmmOrder::Sell(q) => (t0_idx, t1_idx, q)
            };
            let swap_quantity_in = quantity.to();
            let swap = PoolSwap { asset_in_index, asset_out_index, quantity_in: swap_quantity_in };
            swaps.push(swap);
        }
        // Add the ToB order to our tob order list - This is currently converting
        // between two ToB order formats
        if let Some(tob) = solution.searcher.as_ref() {
            tob_orders.push(tob.order.clone());
        }
        // Now for our user orders
        let Some(order_list) = orders_by_pool.get(&solution.id) else {
            warn!(
                "Skipping a solution as we don't have an order list for this pool (or should we \
                 do that?)"
            );
            continue;
        };

        // Sort our pairs
        let order_list = orders_by_pool.get(&solution.id).unwrap();
        for (outcome, order) in solution.limit.iter().zip(order_list.iter()) {
            let pair_index = (pairs.len() - 1) as u16;
            user_orders.push(to_contract_order(order, outcome, pair_index));
        }

        // If we have a ToB order, calculate our pool reward and go from there
        if let Some(tob) = solution.searcher {
            let amm = MarketSnapshot::new(vec![], SqrtPriceX96::default()).unwrap();
            let pool_reward = calculate_reward(tob, amm);
            pool_rewards.push(pool_reward)
        }
    }
    AngstromBundle::new(assets, pairs, swaps, tob_orders, user_orders, pool_rewards)
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
