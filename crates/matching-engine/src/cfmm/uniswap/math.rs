/// This small lib wraps the Uniswap v3 Math lib we're using and handles all the conversions in and out of
/// the formats used by that library.  Ideally we can stop needing to do all this at some point in the future
/// but this keeps us from having to do a ton of complicated casting in our code between very similar Uint types

use alloy_primitives::U256;
use uniswap_v3_math::utils::{ruint_to_u256, u256_to_ruint};
use uniswap_v3_math::tick_math::{get_tick_at_sqrt_ratio, get_sqrt_ratio_at_tick};
use uniswap_v3_math::sqrt_price_math::{_get_amount_0_delta, get_next_sqrt_price_from_input, get_next_sqrt_price_from_output};

use super::SqrtPriceX96;


pub fn tick_at_sqrt_price(price: SqrtPriceX96) -> Result<i32, ()> {
    let cast_price = ruint_to_u256(price.into());
    get_tick_at_sqrt_ratio(cast_price).map_err(|_| ())
}

pub fn sqrt_price_at_tick(tick: i32) -> Result<SqrtPriceX96, ()> {
    let uncast_price = get_sqrt_ratio_at_tick(tick).map_err(|_| ())?;
    Ok(SqrtPriceX96::from(u256_to_ruint(uncast_price)))
}

pub fn token_0_delta(start_price: SqrtPriceX96, end_price: SqrtPriceX96, liquidity: u128, round_up: bool) -> Option<U256> {
    let sqrt_ratio_a_x_96 = ruint_to_u256(start_price.into());
    let sqrt_ratio_b_x_96 = ruint_to_u256(end_price.into());
    _get_amount_0_delta(sqrt_ratio_a_x_96, sqrt_ratio_b_x_96, liquidity, round_up).ok()
        .map(|x| u256_to_ruint(x))
}

/// Computes the new sqrt_price from a given sqrt_price, a liquidity level, and the amount of Token0 we are pulling out of the pool
/// (i.e. buying from the pool).  Note that this does not respect tick bounds or liquidity pools or anything like that, it's up
/// to ensure it's operating over a range with homogeneous liquidity.
pub fn new_sqrt_price_from_output(start_price: SqrtPriceX96, liquidity: u128, quantity: U256, zero_for_one: bool) -> Result<SqrtPriceX96, ()> {
    let sqrt_price = ruint_to_u256(start_price.into());
    let amount_out = ruint_to_u256(quantity);
    get_next_sqrt_price_from_output(sqrt_price, liquidity, amount_out, zero_for_one)
        .map(|x| SqrtPriceX96::from(u256_to_ruint(x))).map_err(|_| ())
}

/// Computes the new sqrt_price from a given sqrt_price, a liquidity level, and the amount of Token0 we are putting into the pool
/// (i.e. selling to the pool).  Note that this does not respect tick bounds or liquidity pools or anything like that, it's up
/// to ensure it's operating over a range with homogeneous liquidity.
pub fn new_sqrt_price_from_input(start_price: SqrtPriceX96, liquidity: u128, quantity: U256, zero_for_one: bool) -> Result<SqrtPriceX96, ()> {
    let sqrt_price = ruint_to_u256(start_price.into());
    let amount_out = ruint_to_u256(quantity);
    get_next_sqrt_price_from_input(sqrt_price, liquidity, amount_out, zero_for_one)
        .map(|x| SqrtPriceX96::from(u256_to_ruint(x))).map_err(|_| ())
}
