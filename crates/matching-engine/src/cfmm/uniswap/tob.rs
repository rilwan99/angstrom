use std::collections::HashMap;

use alloy_primitives::{I256, U256};
use angstrom_types::{
    matching::{Ray, SqrtPriceX96},
    sol_bindings::{grouped_orders::OrderWithStorageData, sol::TopOfBlockOrder}
};
use eyre::{eyre, Context, OptionExt};
use uniswap_v3_math::{swap_math::compute_swap_step, tick_math::get_sqrt_ratio_at_tick};

use super::{MarketSnapshot, Tick};

#[derive(Debug)]
pub struct ToBOutcome {
    pub tribute:        U256,
    pub tick_donations: HashMap<Tick, U256>
}

pub fn calculate_reward(
    tob: OrderWithStorageData<TopOfBlockOrder>,
    amm: MarketSnapshot
) -> eyre::Result<ToBOutcome> {
    // This implies that a bid will be purchasing T0 out of the pool, therefore
    // increasing the price while an ask will be selling T0 to the pool, decreasing
    // the price
    let tick_motion = if tob.is_bid { 1 } else { -1 };

    // We start out at the tick and price that the AMM begins at
    let mut current_tick = amm.current_tick;
    let mut current_price = amm.sqrt_price_x96;
    // TODO:  Figure out how fee pips factor into this
    let fee_pips = 600;

    // Turn our output into a negative number so compute_swap_step knows we're
    // looking to get an exact amount out
    let mut expected_out = I256::try_from(tob.order.amountOut).wrap_err_with(|| {
        format!(
            "Expected ToB order output too large to convert U256 -> I256: {}",
            tob.order.amountOut
        )
    })? * I256::MINUS_ONE;

    // Initialize some things we're going to do to track
    let mut total_cost = U256::ZERO;
    let mut stakes = Vec::new();

    while expected_out < I256::ZERO {
        let next_tick = current_tick + tick_motion;
        let next_price = SqrtPriceX96::from(
            get_sqrt_ratio_at_tick(next_tick)
                .wrap_err_with(|| format!("Unable to get SqrtPrice at tick {}", next_tick))?
        );
        let liquidity = amm
            .liquidity_at_tick(current_tick)
            .ok_or_else(|| eyre!("Unable to find liquidity for tick {}", current_tick))?;
        let (fin_price, amount_in, amount_out, amount_fee) = compute_swap_step(
            current_price.into(),
            next_price.into(),
            liquidity,
            expected_out,
            fee_pips
        )
        .wrap_err_with(|| {
            format!("Unable to compute swap step from tick {} to {}", current_tick, next_tick)
        })?;

        // See how much output we have yet to go
        let signed_out = I256::try_from(amount_out)
            .wrap_err("Output of step too large to convert U256 -> I256")?;
        expected_out = expected_out
            .checked_add(signed_out)
            .ok_or_eyre("Unable to add signed_out to expected_out")?;

        // Add the amount in and our total fee to our cost
        total_cost += amount_in;
        total_cost += amount_fee;

        // How much should this have cost if it was done by the raw price
        let end_price = Ray::from(SqrtPriceX96::from(fin_price));

        // This seems to work properly, so let's run with it
        let avg_price = Ray::calc_price(amount_in, amount_out);

        // See if we have enough bribe left over to cover the total amount so far (can
        // we do this)?
        stakes.push((avg_price, end_price, amount_out));

        // Iterate!
        current_tick += tick_motion;
        current_price = SqrtPriceX96::from(fin_price);
    }

    // Determine how much extra amountIn we have that will be used as tribute to the
    // LPs
    let bribe = tob.amountIn.checked_sub(total_cost).ok_or_else(|| {
        eyre!("Total cost ({}) greater than ammount offered ({})", total_cost, tob.amountIn)
    })?;

    if stakes.is_empty() {
        // TODO: Maybe this should just be a big donation to the current tick?
        return Err(eyre!("No actual purchases could be made with this TOB order"));
    }

    let mut rem_bribe = bribe;
    let mut cur_q = stakes[0].2;
    let mut filled_price = stakes[0].0;
    for window in stakes.windows(2) {
        // Price difference between the average price for the current tick and the
        // average price for the next tick
        let d_price = window[1].0 - window[0].0;
        // Total cost to move this tick and all previous ticks to the average price of
        // the next tick
        let step_cost = d_price.mul_quantity(cur_q + window[0].2);
        if rem_bribe >= step_cost {
            // We have enough tribute to do this move, update our final clearing
            // price and continue iterating
            cur_q += window[0].2;
            filled_price = window[1].0;
            rem_bribe -= step_cost;
        } else {
            // We do not have enough tribute to do this move, figure out what our final
            // clearing price is and break out of this loop
            let partial_dprice = Ray::calc_price(rem_bribe, cur_q + window[0].2);
            filled_price += partial_dprice;
            rem_bribe = U256::ZERO;
            break;
        }
    }

    // Any bribe we have left over after all this is taken as tribute
    let tribute = rem_bribe;

    // We've now found our filled price, we can allocate our reward to each tick
    // based on how much it costs to bring them up to that price.
    let tick_donations: HashMap<Tick, U256> = stakes
        .iter()
        .enumerate()
        .map(|(i, stake)| {
            let tick_num = amm.current_tick + i as i32;
            let total_dprice = filled_price - stake.0;
            let total_reward = total_dprice.mul_quantity(stake.2);
            (tick_num, total_reward)
        })
        .collect();

    Ok(ToBOutcome { tribute, tick_donations })
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

    #[test]
    fn handles_bid_order() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        order.is_bid = true;
        order.order.amountIn = Uint::from(1_000_000_000_000_0_u128);
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm);
        println!("Result: {:?}", result);

        panic!("Butts!");
    }

    #[test]
    fn handles_ask_order() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        order.is_bid = false;
        order.order.amountIn = Uint::from(1_000_000_000_000_0_u128);
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm);
        println!("Result: {:?}", result);

        panic!("Butts!");
    }
}
