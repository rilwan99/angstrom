use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::Deref
};

// uint 160 for represending SqrtPriceX96
use alloy::primitives::{aliases::U256, Uint};
use angstrom_types::{
    matching::{Ray, SqrtPriceX96},
    orders::OrderPrice
};
use eyre::{eyre, Context, Error, OptionExt};
use uniswap_v3_math::{
    sqrt_price_math::{
        _get_amount_0_delta, _get_amount_1_delta, get_next_sqrt_price_from_input,
        get_next_sqrt_price_from_output
    },
    tick_math::{get_sqrt_ratio_at_tick, get_tick_at_sqrt_ratio, MAX_TICK, MIN_TICK}
};

pub mod pool;
pub mod pool_manager;
pub mod pool_providers;
pub mod tob;

type Tick = i32;

/// A PoolRange describes the liquidity conditions within a specific range of
/// ticks.  The range can be described as `[lower_tick, upper_tick)`.  The range
/// must start and end on a tick bound, but may include an arbitrary number of
/// ticks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolRange {
    /// Lower tick for this range
    lower_tick: Tick,
    /// Upper tick for this range
    upper_tick: Tick,
    /// Total liquidity within this range
    liquidity:  u128
}

impl PoolRange {
    pub fn new(lower_tick: Tick, upper_tick: Tick, liquidity: u128) -> Result<Self, Error> {
        // Validate our inputs
        if upper_tick <= lower_tick {
            return Err(eyre!(
                "Upper tick bound less than or equal to lower tick bound for range ({}, {})",
                lower_tick,
                upper_tick
            ));
        }
        if upper_tick > MAX_TICK {
            return Err(eyre!("Proposed upper tick '{}' out of valid tick range", upper_tick));
        }
        if lower_tick < MIN_TICK {
            return Err(eyre!("Proposed lower tick '{}' out of valid tick range", lower_tick));
        }
        Ok(Self { lower_tick, upper_tick, liquidity })
    }

    pub fn lower_tick(&self) -> i32 {
        self.lower_tick
    }

    pub fn upper_tick(&self) -> i32 {
        self.upper_tick
    }

    pub fn liquidity(&self) -> u128 {
        self.liquidity
    }
}

/// Snapshot of a particular Uniswap contract and its liquidity.  A contract has
/// a Token0 and a Token1 which represent the two quantities that are being
/// exchanged.  This snapshot contains the current price and liquidity
/// information representing the state of a Uniswap contract at a point in time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketSnapshot {
    /// Known tick ranges and liquidity positions gleaned from the market
    /// snapshot
    ranges:         Vec<PoolRange>,
    /// The current SqrtPriceX96 for this pairing as of this snapshot
    /// (𝛥Token1/𝛥Token0)
    sqrt_price_x96: SqrtPriceX96,
    /// The current tick our price lives in - price might not be precisely on a
    /// tick bound, this is the LOWER of the possible ticks
    current_tick:   Tick,
    /// Index into the 'ranges' vector for the PoolRange that includes the tick
    /// our current price lives at/in
    cur_tick_idx:   usize
}

impl MarketSnapshot {
    pub fn new(mut ranges: Vec<PoolRange>, sqrt_price_x96: SqrtPriceX96) -> Result<Self, Error> {
        // Sort our ranges
        ranges.sort_by(|a, b| a.lower_tick.cmp(&b.lower_tick));

        // Ensure the ranges are contiguous
        if !ranges
            .windows(2)
            .all(|w| w[0].upper_tick == w[1].lower_tick)
        {
            return Err(eyre!("Tick windows not contiguous, cannot create snapshot"));
        }

        // Get our current tick from our current price
        let current_tick = get_tick_at_sqrt_ratio(sqrt_price_x96.into()).wrap_err_with(|| {
            eyre!("Unable to get a tick from our current price '{:?}'", sqrt_price_x96)
        })?;

        // Find the tick range that our current tick lies within
        let Some(cur_tick_idx) = ranges
            .iter()
            .position(|r| r.lower_tick <= current_tick && current_tick < r.upper_tick)
        else {
            return Err(eyre!("Unable to find initialized tick window for tick '{}'", current_tick));
        };

        Ok(Self { ranges, sqrt_price_x96, current_tick, cur_tick_idx })
    }

    /// Find the PoolRange in this market snapshot that the provided tick lies
    /// within, if any
    pub fn get_range_for_tick(&self, tick: Tick) -> Option<PoolRangeRef> {
        self.ranges
            .iter()
            .enumerate()
            .find(|(_, r)| r.lower_tick <= tick && tick < r.upper_tick)
            .map(|(range_idx, range)| PoolRangeRef { market: self, range, range_idx })
    }

    pub fn current_position(&self) -> MarketPrice {
        let range = self
            .ranges
            .get(self.cur_tick_idx)
            .map(|range| PoolRangeRef { market: self, range, range_idx: self.cur_tick_idx })
            .unwrap();
        MarketPrice {
            market_pool: range,
            tick:        self.current_tick,
            price:       self.sqrt_price_x96
        }
    }

    pub fn liquidity_at_tick(&self, tick: Tick) -> Option<u128> {
        self.get_range_for_tick(tick).map(|range| range.liquidity())
    }
}

#[derive(Clone, Debug)]
pub struct PoolRangeRef<'a> {
    market:    &'a MarketSnapshot,
    range:     &'a PoolRange,
    range_idx: usize
}

impl<'a> Deref for PoolRangeRef<'a> {
    type Target = PoolRange;

    fn deref(&self) -> &Self::Target {
        self.range
    }
}

impl<'a> PoolRangeRef<'a> {
    pub fn new(market: &'a MarketSnapshot, range: &'a PoolRange, range_idx: usize) -> Self {
        Self { market, range, range_idx }
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
    /// Current PoolRange that the price is in
    market_pool: PoolRangeRef<'a>,
    /// Tick number within the current PoolRange that we're working with
    tick:        Tick,
    /// The ratio between Token0 and Token1
    price:       SqrtPriceX96
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

    /// Buy from the AMM with a quantity of the input token that exceeds the
    /// amount required to purchase the requested quantity of the output token.
    /// The excess quantity is distributed as our "bribe" to the LPs present in
    /// each tick based on our ToB distribution algorithm.
    pub fn buy_and_bribe(
        &self,
        _input: U256,
        _output: U256
    ) -> Result<(Self, HashMap<Tick, U256>), String> {
        Ok((self.clone(), HashMap::new()))
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
        self.market_pool.liquidity
    }

    pub fn range_to(&self, price: SqrtPriceX96) -> eyre::Result<PriceRange<'a>> {
        let tick = price.to_tick()?;
        let market_pool = self
            .market_pool
            .market
            .get_range_for_tick(tick)
            .ok_or_eyre("Unable to find pool")?;
        let target = Self { market_pool, tick, price };
        Ok(PriceRange::new(self.clone(), target))
    }

    /// This will produce a Uniswap Price Range that spans from the current
    /// price to the CLOSER of the target price or the nearest liquidity
    /// pool boundary
    pub fn order_to_target(
        &self,
        target_price: Option<SqrtPriceX96>,
        buy: bool
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
        let bound_price = if buy {
            SqrtPriceX96::at_tick(self.market_pool.upper_tick).ok()?
        } else {
            SqrtPriceX96::at_tick(self.market_pool.lower_tick).ok()?
        };

        let mut new_range_idx = self.market_pool.range_idx;
        let mut pool = self.market_pool.range;
        let (mut tick_bound_price, next_tick) = if buy {
            let upper_tick_price = get_sqrt_ratio_at_tick(pool.upper_tick)
                .ok()
                .map(SqrtPriceX96::from)?;
            let next_tick = self.market_pool.range_idx.checked_sub(1);
            (upper_tick_price, next_tick)
        } else {
            let lower_tick_price = get_sqrt_ratio_at_tick(pool.lower_tick)
                .ok()
                .map(SqrtPriceX96::from)?;
            let next_tick = self.market_pool.range_idx.checked_add(1);
            (lower_tick_price, next_tick)
        };
        if self.price == tick_bound_price {
            // We're at the tick bound, we need to look at the next pool!
            new_range_idx = next_tick?;
            pool = self.market_pool.market.ranges.get(new_range_idx)?;
            tick_bound_price = if buy {
                get_sqrt_ratio_at_tick(pool.upper_tick)
                    .ok()
                    .map(SqrtPriceX96::from)?
            } else {
                get_sqrt_ratio_at_tick(pool.lower_tick)
                    .ok()
                    .map(SqrtPriceX96::from)?
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
        let end_bound = Self {
            market_pool: PoolRangeRef { range: pool, range_idx: new_range_idx, ..self.market_pool },
            price:       closest_price,
            tick:        get_tick_at_sqrt_ratio(closest_price.into()).ok()?
        };
        Some(PriceRange::new(self.clone(), end_bound))
    }

    pub fn price(&self) -> &SqrtPriceX96 {
        &self.price
    }

    /// Return the current price as a float - we need to figure out what our
    /// price representation is going to look like overall
    pub fn as_float(&self) -> f64 {
        self.price.as_f64()
    }

    pub fn as_u256(&self) -> U256 {
        self.price.into()
    }
}

#[derive(Clone, Debug)]
pub struct PriceRange<'a> {
    pub start_bound: MarketPrice<'a>,
    pub end_bound:   MarketPrice<'a>,
    pub d_t0:        U256,
    pub d_t1:        U256
}

impl<'a> PriceRange<'a> {
    pub fn new(start_bound: MarketPrice<'a>, end_bound: MarketPrice<'a>) -> Self {
        let (d_t0, d_t1) =
            Self::delta_to_price(start_bound.price, end_bound.price, start_bound.liquidity());
        Self { start_bound, end_bound, d_t0, d_t1 }
    }

    fn delta_to_price(
        start_price: SqrtPriceX96,
        end_price: SqrtPriceX96,
        liquidity: u128
    ) -> (U256, U256) {
        let sqrt_ratio_a_x_96 = start_price.into();
        let sqrt_ratio_b_x_96 = end_price.into();
        let d_t0 = _get_amount_0_delta(sqrt_ratio_a_x_96, sqrt_ratio_b_x_96, liquidity, false)
            .unwrap_or(Uint::from(0));
        let d_t1 = _get_amount_1_delta(sqrt_ratio_a_x_96, sqrt_ratio_b_x_96, liquidity, false)
            .unwrap_or(Uint::from(0));
        (d_t0, d_t1)
    }

    pub fn is_buy(&self) -> bool {
        self.start_bound.price < self.end_bound.price
    }

    /// Returns `(quantity, price)`
    pub fn quantity(&self, target_price: OrderPrice) -> (U256, U256) {
        let t: SqrtPriceX96 = Ray::from(*target_price).into();

        // If our target price is past our end bound, our quantity is the entire range
        if self.is_buy() && t > self.end_bound.price {
            return (self.d_t0, self.d_t1);
        } else if t < self.end_bound.price {
            return (self.d_t0, self.d_t1);
        }

        let (quantity, price) =
            Self::delta_to_price(self.start_bound.price, t, self.start_bound.liquidity());
        (quantity, price)
    }

    // Maybe it's OK that I don't check the price again here because in the matching
    // algo I've only offered a quantity bounded by the price, so we should
    // always be OK?
    pub fn fill(&self, quantity: U256) -> Self {
        let liquidity = self.start_bound.liquidity();
        let end_sqrt_price = if self.is_buy() {
            get_next_sqrt_price_from_output(
                self.start_bound.price.into(),
                liquidity,
                quantity,
                true
            )
            .map(SqrtPriceX96::from)
            .unwrap()
        } else {
            get_next_sqrt_price_from_input(self.start_bound.price.into(), liquidity, quantity, true)
                .map(SqrtPriceX96::from)
                .unwrap()
        };
        let (d_t0, d_t1) = Self::delta_to_price(self.start_bound.price, end_sqrt_price, liquidity);
        let mut end_bound = self.start_bound.clone();
        end_bound.price = end_sqrt_price;
        Self { end_bound, start_bound: self.start_bound.clone(), d_t0, d_t1 }
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::U160;

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

        let valid_price = SqrtPriceX96::from(get_sqrt_ratio_at_tick(2325).unwrap());

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
