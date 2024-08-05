use std::collections::HashMap;

use alloy_primitives::{I256, U256};
use angstrom_types::{
    matching::{Ray, SqrtPriceX96},
    sol_bindings::{grouped_orders::OrderWithStorageData, sol::TopOfBlockOrder}
};
use uniswap_v3_math::{swap_math::compute_swap_step, tick_math::get_sqrt_ratio_at_tick};

use super::{MarketSnapshot, Tick};

pub fn calculate_reward(
    tob: OrderWithStorageData<TopOfBlockOrder>,
    amm: MarketSnapshot
) -> HashMap<Tick, U256> {
    // Determine whether we're buying from or selling to the AMM - is this
    // necessary?
    let tick_motion = if tob.is_bid { 1 } else { -1 };

    // We start out at the tick and price that the AMM begins at
    let mut current_tick = amm.current_tick;
    let mut current_price = amm.sqrt_price_x96;
    // TODO:  Figure out how fee pips factor into this
    let fee_pips = 600;

    // Turn our output into a negative number so compute_swap_step knows we're
    // looking to get an exact amount out
    let mut expected_out = I256::try_from(tob.order.amountOut).unwrap() * I256::MINUS_ONE;

    // Initialize some things we're going to do to track
    let mut total_cost = U256::ZERO;
    let mut stakes = Vec::new();

    while expected_out < I256::ZERO {
        let next_price =
            SqrtPriceX96::from(get_sqrt_ratio_at_tick(current_tick + tick_motion).unwrap());
        let liquidity = amm
            .liquidity_at_tick(current_tick)
            .expect("Unable to find market range for the current tick");
        let (fin_price, amount_in, amount_out, amount_fee) = compute_swap_step(
            current_price.into(),
            next_price.into(),
            liquidity,
            expected_out,
            fee_pips
        )
        .unwrap();

        // See how much output we have yet to go
        let signed_out = I256::try_from(amount_out).unwrap();
        expected_out = expected_out.checked_add(signed_out).unwrap();

        // Add the amount in and our total fee to our cost
        total_cost += amount_in;
        // We are going to totally neglect fee for now to see if we can just get this
        // working cost += amount_fee;

        // How much should this have cost if it was done by the raw price
        let end_price = Ray::from(SqrtPriceX96::from(fin_price));

        // This seems to work properly, so let's run with it
        let calced_price = Ray::calc_price(amount_in, amount_out);

        // See if we have enough bribe left over to cover the total amount so far (can
        // we do this)?
        stakes.push((calced_price, end_price, amount_out));

        println!("Amount out for step: {} - Amount in for step: {}", amount_out, amount_in);

        // Iterate!
        current_tick += tick_motion;
        current_price = SqrtPriceX96::from(fin_price);
    }

    // Determine how much extra amountIn we have that will be used as tribute to the
    // LPs
    let tribute = tob
        .amountIn
        .checked_sub(total_cost)
        .expect("What the heck, not enough input for my bribearino!");
    println!("Tribute: {}", tribute);

    let mut rem_tribute = tribute;
    let mut cur_q = U256::ZERO;
    let mut filled_price = stakes[0].0;
    for window in stakes.windows(2) {
        // Price difference between the average price for the current tick and the
        // average price for the next tick
        let d_price = window[1].0 - window[0].0;
        // Total cost to move this tick and all previous ticks to the average price of
        // the next tick
        println!("Quantity for step: {}", cur_q + window[0].2);
        println!("Average price for window: {:?} - Diff from window2: {:?}", window[0].0, d_price);
        let step_cost = d_price.mul_quantity(cur_q + window[0].2);
        if rem_tribute >= step_cost {
            // We have enough tribute to do this move, update our final clearing
            // price and continue iterating
            println!("We have enough to complete a step - {}", step_cost);
            cur_q += window[0].2;
            filled_price = window[1].0;
            rem_tribute -= step_cost;
        } else {
            // We do not have enough tribute to do this move, figure out what our final
            // clearing price is and break out of this loop
            println!(
                "We don't have enough to complete a step - {} needed {} available",
                step_cost, rem_tribute
            );
            let partial_dprice = Ray::calc_price(rem_tribute, (cur_q + window[0].2));
            filled_price = filled_price + partial_dprice;
            rem_tribute = U256::ZERO;
            break;
        }
    }
    // If we have any remaining tribute, up our overall filled price
    if rem_tribute > U256::ZERO {
        println!("Extra tribute, and I'm gonna use it: {:?}", rem_tribute);
        let final_dprice = Ray::calc_price(rem_tribute, cur_q);
        filled_price = filled_price + final_dprice;
    }

    // We've now found our filled price, we can allocate our reward to each tick
    // based on how much it costs to bring them up to that price.
    let result = stakes
        .iter()
        .enumerate()
        .map(|(i, stake)| {
            let tick_num = amm.current_tick + i as i32;
            let total_dprice = filled_price - stake.0;
            let total_reward = total_dprice.mul_quantity(stake.2);
            (tick_num, total_reward)
        })
        .collect();
    result
}

#[cfg(test)]
mod test {

    use alloy_primitives::Uint;
    use angstrom_types::matching::SqrtPriceX96;
    use rand::thread_rng;
    use testing_tools::type_generator::orders::generate_top_of_block_order;
    use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

    use super::calculate_reward;
    use crate::cfmm::uniswap::{MarketSnapshot, PoolRange};

    fn generate_amm_market(target_tick: i32) -> MarketSnapshot {
        let range =
            PoolRange::new(target_tick - 1000, target_tick + 1000, 100_000_000_000_000).unwrap();
        let ranges = vec![range];
        let sqrt_price_x96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(target_tick).unwrap());
        MarketSnapshot::new(ranges, sqrt_price_x96).unwrap()
    }

    #[test]
    fn calculates_reward() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        order.order.amountIn = Uint::from(1_000_000_000_000_0_u128);
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm);
        println!("Result: {:?}", result);

        panic!("Butts!");
    }
}
