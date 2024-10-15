use std::collections::HashMap;

use alloy::primitives::{aliases::I24, U256};
use eyre::eyre;

use super::rewards::RewardsUpdate;
use crate::{
    matching::uniswap::{PoolSnapshot, Quantity, Tick},
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ToBOutcome {
    pub start_tick:      i32,
    pub start_liquidity: u128,
    pub tribute:         U256,
    pub total_cost:      U256,
    pub total_reward:    U256,
    pub tick_donations:  HashMap<Tick, U256>
}

impl ToBOutcome {
    /// Sum of the donations across all ticks
    pub fn total_donations(&self) -> U256 {
        self.tick_donations
            .iter()
            .fold(U256::ZERO, |acc, (_tick, donation)| acc + donation)
    }

    /// Tick donations plus tribute to determine total value of this outcome
    pub fn total_value(&self) -> U256 {
        self.total_donations() + self.tribute
    }

    pub fn from_tob_and_snapshot(
        tob: &OrderWithStorageData<TopOfBlockOrder>,
        snapshot: &PoolSnapshot
    ) -> eyre::Result<Self> {
        let output = match tob.is_bid {
            true => Quantity::Token0(tob.quantityOut),
            false => Quantity::Token1(tob.quantityOut)
        };
        let pricevec = (snapshot.current_price() - output)?;
        let total_cost: u128 = pricevec.input().saturating_to();
        if total_cost > tob.quantityIn {
            return Err(eyre!("Not enough input to cover the transaction"));
        }
        let leftover = tob.quantityIn - total_cost;
        let donation = pricevec.donation(leftover);
        let rewards = ToBOutcome {
            start_tick:      snapshot.current_price().tick(),
            start_liquidity: snapshot.current_price().liquidity(),
            tribute:         U256::from(donation.tribute),
            total_cost:      pricevec.input(),
            total_reward:    U256::from(donation.total_donated),
            tick_donations:  donation.tick_donations
        };
        Ok(rewards)
    }

    pub fn to_rewards_update(&self) -> RewardsUpdate {
        let mut donations = self.tick_donations.iter().collect::<Vec<_>>();
        // Will sort from lowest to highest (donations[0] will be the lowest tick
        // number)
        donations.sort_by_key(|f| f.0);
        // Each reward value is the cumulative sum of the rewards before it
        let quantities = donations
            .iter()
            .scan(U256::ZERO, |state, (_tick, q)| {
                *state += **q;
                Some(u128::try_from(*state).unwrap())
            })
            .collect::<Vec<_>>();
        let start_tick = I24::try_from(donations.first().map(|(a, _)| *a + 1).unwrap_or_default())
            .unwrap_or_default();
        match quantities.len() {
            0 | 1 => RewardsUpdate::CurrentOnly {
                amount: quantities.first().copied().unwrap_or_default()
            },
            _ => RewardsUpdate::MultiTick {
                start_tick,
                start_liquidity: self.start_liquidity,
                quantities
            }
        }
    }
}

#[cfg(test)]
mod test {
    use alloy::primitives::Uint;
    use rand::thread_rng;
    use testing_tools::type_generator::orders::generate_top_of_block_order;
    use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

    use crate::{
        contract_payloads::tob::ToBOutcome,
        matching::{
            uniswap::{LiqRange, PoolSnapshot},
            SqrtPriceX96
        }
    };

    fn generate_amm_market(target_tick: i32) -> PoolSnapshot {
        let range =
            LiqRange::new(target_tick - 1000, target_tick + 1000, 100_000_000_000_000).unwrap();
        let ranges = vec![range];
        let sqrt_price_x96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(target_tick).unwrap());
        PoolSnapshot::new(ranges, sqrt_price_x96).unwrap()
    }

    #[test]
    fn calculates_reward() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        let total_payment = 10_000_000_000_000_u128;
        let tob = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot).unwrap();
        let total_donations = result.total_donations();
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            Uint::from(total_payment),
            "Total allocations do not add up to input payment"
        );
    }

    #[test]
    fn handles_insufficient_funds() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        let tob = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(10_000_000_u128),
            Some(100000000_u128)
        );
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot);
        assert!(result.is_err_and(|e| {
            e.to_string()
                .starts_with("Total cost greater than amount offered")
        }));
    }

    #[test]
    fn handles_precisely_zero_donation() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        // Hand-calculated that this is the correct payment for this starting price and
        // liquidity
        let total_payment = 2_201_872_310_000_u128;
        let tob = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot).unwrap();
        let total_donations = result.total_donations();
        assert!(
            result.tick_donations.is_empty(),
            "Donations are being offered when we shouldn't have any"
        );
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            Uint::from(total_payment),
            "Total allocations do not add up to input payment"
        );
    }

    #[test]
    fn handles_partial_donation() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        let partial_donation = 20_000_000_u128;
        let total_payment = 2_201_872_310_000_u128 + partial_donation;
        let tob = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot).unwrap();
        let total_donations = result.total_donations();
        assert_eq!(result.tick_donations.len(), 1, "Wrong number of donations");
        assert!(result.tick_donations.contains_key(&99000), "Donation missing");
        assert_eq!(
            result
                .tick_donations
                .get(&99000)
                .unwrap()
                .saturating_to::<u128>(),
            partial_donation,
            "Donation of incorrect size"
        );
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            Uint::from(total_payment),
            "Total allocations do not add up to input payment"
        );
    }

    #[test]
    fn handles_bid_order() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        let tob = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(10_000_000_000_000_u128),
            Some(100000000_u128)
        );
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn handles_ask_order() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        let tob = generate_top_of_block_order(
            &mut rng,
            false,
            None,
            None,
            Some(10_000_000_000_000_u128),
            Some(800000000_u128)
        );
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot);
        assert!(result.is_ok());
    }

    #[test]
    fn only_rewards_initialized_ticks() {
        let mut rng = thread_rng();
        let snapshot = generate_amm_market(100000);
        let total_payment = 2_203_371_417_593_u128;
        let tob = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let first_tick = 100000 - 1000;
        let result = ToBOutcome::from_tob_and_snapshot(&tob, &snapshot).unwrap();
        assert!(
            result.tick_donations.len() == 1,
            "Too many donations - only one initialized tick in this market"
        );
        assert!(
            result.tick_donations.contains_key(&first_tick),
            "Donation not made to only initialized tick"
        );
    }
}
