use std::{
    cmp::{max, min},
    ops::Deref,
};

// uint 160 for represending SqrtPriceX96
use alloy_primitives::aliases::{U160, U256};
use alloy_primitives::Uint;
// Malachite stuff for our math and conversions
use malachite::num::arithmetic::traits::{Pow, PowerOf2};
use malachite::{num::conversion::traits::RoundingInto, Natural, Rational};

use self::math::{
    new_sqrt_price_from_input, new_sqrt_price_from_output, sqrt_price_at_tick, tick_at_sqrt_price,
    token_0_delta,
};

pub mod math;
pub mod pool;
mod loader;

/// A Tick is represented as an i32 as its value range is from around
/// -900000..900000
const MIN_TICK: i32 = -887272;
const MAX_TICK: i32 = 887272;
type Tick = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SqrtPriceX96(U160);

impl SqrtPriceX96 {
    /// Uses malachite.rs to approximate this value as a floating point number.
    /// Converts from the internal U160 representation of `sqrt(P)` to an
    /// approximated f64 representation of `P`, which is a change to the
    /// value of this number and why this isn't `From<SqrtPriceX96> for f64`
    pub fn as_float_price(&self) -> f64 {
        let numerator = Natural::from_limbs_asc(self.0.as_limbs());
        let denominator: Natural = Natural::power_of_2(96u64);
        let sqrt_price = Rational::from_naturals(numerator, denominator);
        let price = sqrt_price.pow(2u64);
        let (res, _) = price.rounding_into(malachite::rounding_modes::RoundingMode::Floor);
        res
    }

    /// Convert a floating point price `P` into a SqrtPriceX96 `sqrt(P)`
    pub fn from_float_price(price: f64) -> Self {
        SqrtPriceX96(U160::from(price.sqrt() * (2.0_f64.pow(96))))
    }
}

impl Deref for SqrtPriceX96 {
    type Target = U160;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SqrtPriceX96> for U256 {
    fn from(value: SqrtPriceX96) -> Self {
        Uint::from(value.0)
    }
}

impl From<U256> for SqrtPriceX96 {
    fn from(value: U256) -> Self {
        Self(Uint::from(value))
    }
}

impl From<U160> for SqrtPriceX96 {
    fn from(value: U160) -> Self {
        Self(value)
    }
}

/// A PoolRange describes the liquidity conditions within a specific range of
/// ticks.  The range can be described as `[lower_tick, upper_tick)`.  The range
/// must start and end on a tick bound, but may include an arbitrary number of
/// ticks.
#[derive(Debug, Clone)]
pub struct PoolRange {
    /// Lower tick for this range
    lower_tick: Tick,
    /// Upper tick for this range
    upper_tick: Tick,
    /// Total liquidity within this range
    liquidity: u128,
}

impl PoolRange {
    pub fn new(lower_tick: Tick, upper_tick: Tick, liquidity: u128) -> Result<Self, String> {
        // Validate our inputs
        if upper_tick <= lower_tick {
            return Err(format!(
                "Upper tick bound less than or equal to lower tick bound for range ({}, {})",
                lower_tick, upper_tick
            ));
        }
        if upper_tick > MAX_TICK {
            return Err(format!("Proposed upper tick '{}' out of valid tick range", upper_tick));
        }
        if lower_tick < MIN_TICK {
            return Err(format!("Proposed lower tick '{}' out of valid tick range", lower_tick));
        }
        Ok(Self { lower_tick, upper_tick, liquidity })
    }
}

/// Snapshot of a particular Uniswap contract and its liquidity.  A contract has
/// a Token0 and a Token1 which represent the two quantities that are being
/// exchanged.  This snapshot contains the current price and liquidity
/// information representing the state of a Uniswap contract at a point in time.
#[derive(Debug)]
pub struct MarketSnapshot {
    /// Known tick ranges and liquidity positions gleaned from the market
    /// snapshot
    ranges: Vec<PoolRange>,
    /// The current SqrtPriceX96 for this pairing as of this snapshot
    /// (𝛥Token1/𝛥Token0)
    sqrt_price_x96: SqrtPriceX96,
    /// The current tick our price lives in - price might not be precisely on a
    /// tick bound, this is the LOWER of the possible ticks
    current_tick: Tick,
    /// Index into the 'ranges' vector for the PoolRange that includes the tick
    /// our current price lives at/in
    cur_tick_idx: usize,
}

impl MarketSnapshot {
    pub fn new(mut ranges: Vec<PoolRange>, sqrt_price_x96: SqrtPriceX96) -> Result<Self, String> {
        // Sort our ranges
        ranges.sort_by(|a, b| a.lower_tick.cmp(&b.lower_tick));

        // Ensure the ranges are contiguous
        if !ranges
            .windows(2)
            .all(|w| w[0].upper_tick == w[1].lower_tick)
        {
            return Err("Tick windows not contiguous, cannot create snapshot".to_string());
        }

        // Get our current tick from our current price
        let current_tick = tick_at_sqrt_price(sqrt_price_x96).map_err(|_| {
            format!("Unable to get a tick from our current price '{}'", sqrt_price_x96.0)
        })?;

        // Find the tick range that our current tick lies within
        let Some(cur_tick_idx) = ranges
            .iter()
            .position(|r| r.lower_tick <= current_tick && current_tick < r.upper_tick)
        else {
            return Err(format!(
                "Unable to find initialized tick window for tick '{}'",
                current_tick
            ));
        };

        Ok(Self { ranges, sqrt_price_x96, current_tick, cur_tick_idx })
    }

    /// Find the PoolRange in this market snapshot that the provided tick lies
    /// within, if any
    pub fn get_range_for_tick(&self, tick: Tick) -> Option<(usize, &PoolRange)> {
        let range_idx = self
            .ranges
            .iter()
            .position(|r| r.lower_tick <= tick && tick < r.upper_tick);
        range_idx.map(|idx| (idx, self.ranges.get(idx).unwrap()))
    }

    /// Get a pool range directly out of this snapshot by internal storage index
    pub fn get_range(&self, index: usize) -> Option<&PoolRange> {
        self.ranges.get(index)
    }

    pub fn current_position(&self) -> MarketPrice {
        MarketPrice {
            state: self,
            range_idx: self.cur_tick_idx,
            tick: self.current_tick,
            price: self.sqrt_price_x96,
        }
    }
}

/// A MarketPrice represents a price based on the market state preserved in a
/// parent MarketSnapshot.  The MarketPrice can be moved and operated on to
/// simulate the behavior of the price if the underlying assets are bought and
/// sold.  This price will always depend on a specific MarketSnapshot so if
/// underlying parameters such as Liquidity or the decimal representation of the
/// assets were to change, you would need to procure a new MarketSnapshot and
/// new MarketPrices dependent on that.
#[derive(Clone, Debug)]
pub struct MarketPrice<'a> {
    /// Reference to the Market State we're using as the basis for computation
    state: &'a MarketSnapshot,
    /// Index of the current PoolRange our price lies within
    range_idx: usize,
    /// Tick number within the current PoolRange that we're working with
    tick: Tick,
    /// The ratio between Token0 and Token1
    price: SqrtPriceX96,
}

impl<'a> MarketPrice<'a> {
    /// Build a PriceRange from the current price to the closer of either the
    /// target price or the bound of the current liquidity pool.  Will jump
    /// to the next liquidity pool if we're currently on the edge of one
    pub fn buy_to_price(&self, target_price: SqrtPriceX96) -> Option<PriceRange<'a>> {
        self.order_to_target(Some(target_price), true)
    }

    /// Build a PriceRange from the current price to the bound of the current
    /// liquidity pool.  Will jump to the next liquidity pool if we're
    /// currently on the edge of one.
    pub fn buy_to_next_bound(&self) -> Option<PriceRange<'a>> {
        self.order_to_target(None, true)
    }

    pub fn sell_to_price(&self, target_price: SqrtPriceX96) -> Option<PriceRange<'a>> {
        self.order_to_target(Some(target_price), false)
    }

    pub fn sell_to_next_bound(&self) -> Option<PriceRange<'a>> {
        self.order_to_target(None, false)
    }

    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn liquidity(&self) -> u128 {
        self.state
            .get_range(self.range_idx)
            .map(|p| p.liquidity)
            .unwrap_or(0)
    }

    /// This will produce a Uniswap Price Range that spans from the current
    /// price to the CLOSER of the target price or the nearest liquidity
    /// pool boundary
    pub fn order_to_target(
        &self,
        target_price: Option<SqrtPriceX96>,
        buy: bool,
    ) -> Option<PriceRange<'a>> {
        // Bounds check our target price if provided
        if let Some(p) = target_price {
            if buy {
                // Buying from the market will raise the price, so if our target price is on the
                // wrong side of our current price, we can't do this.
                if p <= self.price {
                    return None;
                }
            } else {
                // Selling to the market will lower the price, so the same applies here
                if p >= self.price {
                    return None;
                }
            }
        }
        let mut new_range_idx = self.range_idx;
        let mut pool = self.state.get_range(new_range_idx)?;
        let (mut tick_bound_price, next_tick) = if buy {
            let upper_tick_price = sqrt_price_at_tick(pool.upper_tick).ok()?;
            let next_tick = self.range_idx.checked_sub(1);
            (upper_tick_price, next_tick)
        } else {
            let lower_tick_price = sqrt_price_at_tick(pool.lower_tick).ok()?;
            let next_tick = self.range_idx.checked_add(1);
            (lower_tick_price, next_tick)
        };
        if self.price == tick_bound_price {
            // We're at the tick bound, we need to look at the next pool!
            new_range_idx = next_tick?;
            pool = self.state.get_range(new_range_idx)?;
            tick_bound_price = if buy {
                sqrt_price_at_tick(pool.upper_tick).ok()?
            } else {
                sqrt_price_at_tick(pool.lower_tick).ok()?
            };
        }
        let closest_price = if let Some(p) = target_price {
            if buy {
                min(p, tick_bound_price)
            } else {
                max(p, tick_bound_price)
            }
        } else {
            tick_bound_price
        };
        let quantity = token_0_delta(self.price, closest_price, pool.liquidity, false)?;
        let end_bound = Self {
            state: self.state,
            price: closest_price,
            range_idx: new_range_idx,
            tick: tick_at_sqrt_price(closest_price).ok()?,
        };
        Some(PriceRange { start_bound: self.clone(), end_bound, quantity })
    }

    pub fn price(&self) -> &SqrtPriceX96 {
        &self.price
    }

    /// Return the current price as a float - we need to figure out what our
    /// price representation is going to look like overall
    pub fn as_float(&self) -> f64 {
        self.price.as_float_price()
    }
}

#[derive(Clone, Debug)]
pub struct PriceRange<'a> {
    pub start_bound: MarketPrice<'a>,
    pub end_bound: MarketPrice<'a>,
    pub quantity: U256,
}

impl<'a> PriceRange<'a> {
    pub fn quantity(&self, target_price: f64) -> f64 {
        let t = SqrtPriceX96::from_float_price(target_price);

        // If our target price is past our end bound, our quantity is the entire range
        if self.end_bound.price > self.start_bound.price {
            if t > self.end_bound.price {
                return self.quantity.into();
            }
        } else if t < self.end_bound.price {
            return self.quantity.into();
        }

        // Otherwise we have to calculate the precise quantity we have to sell
        let quantity =
            token_0_delta(self.start_bound.price, t, self.start_bound.liquidity(), false)
                .unwrap_or(Uint::from(0));
        quantity.into()
    }

    // Maybe it's OK that I don't check the price again here because in the matching
    // algo I've only offered a quantity bounded by the price, so we should
    // always be OK?
    pub fn fill(&self, quantity: f64) -> Self {
        let q = Uint::from(quantity);
        let liquidity = self.start_bound.liquidity();
        let end_sqrt_price = if self.end_bound.price > self.start_bound.price {
            new_sqrt_price_from_output(self.start_bound.price, liquidity, q, true).unwrap()
        } else {
            new_sqrt_price_from_input(self.start_bound.price, liquidity, q, true).unwrap()
        };
        let mut end_bound = self.start_bound.clone();
        end_bound.price = end_sqrt_price;
        Self { end_bound, start_bound: self.start_bound.clone(), quantity: q }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_contiguous_ticks() {
        let good_ranges = vec![
            PoolRange::new(2000, 2100, 10000000).unwrap(),
            PoolRange::new(2100, 2200, 10000000).unwrap(),
            PoolRange::new(2200, 2300, 10000000).unwrap(),
            PoolRange::new(2300, 2400, 10000000).unwrap(),
            PoolRange::new(2400, 2500, 10000000).unwrap(),
        ];

        let bad_ranges = vec![
            PoolRange::new(2000, 2100, 10000000).unwrap(),
            PoolRange::new(2100, 2200, 10000000).unwrap(),
            PoolRange::new(2210, 2300, 10000000).unwrap(),
            PoolRange::new(2300, 2400, 10000000).unwrap(),
            PoolRange::new(2400, 2500, 10000000).unwrap(),
        ];

        let valid_price = sqrt_price_at_tick(2325).unwrap();

        // Good ranges, good price, should be fine
        MarketSnapshot::new(good_ranges.clone(), valid_price).unwrap();
        // Good ranges, bad price, should fail
        assert!(MarketSnapshot::new(good_ranges, U160::from(0).into()).is_err());
        // Bad ranges, good price, should fail
        assert!(MarketSnapshot::new(bad_ranges, U160::from(0).into()).is_err());
    }

    #[test]
    fn span_sums_and_rounding_work() {
        let liq = 50000000000;
        let t1 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(10).unwrap();
        let t2 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(20).unwrap();
        let t3 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(30).unwrap();

        let step_12 =
            uniswap_v3_math::sqrt_price_math::_get_amount_0_delta(t1, t2, liq, true).unwrap();
        let step_23 =
            uniswap_v3_math::sqrt_price_math::_get_amount_0_delta(t2, t3, liq, true).unwrap();
        let step_13 =
            uniswap_v3_math::sqrt_price_math::_get_amount_0_delta(t1, t3, liq, true).unwrap();

        assert_eq!(step_12 + step_23, step_13, "Sums not equal");
    }

    #[test]
    fn test_ask_iter() {}
}
