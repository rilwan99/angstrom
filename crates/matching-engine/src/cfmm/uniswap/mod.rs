use std::{
    cmp::{max, min},
    collections::HashMap
};

// uint 160 for represending SqrtPriceX96
use alloy::primitives::{aliases::U256, Uint};
use angstrom_types::{
    matching::{Ray, SqrtPriceX96},
    orders::OrderPrice
};
use uniswap_v3_math::{
    sqrt_price_math::{
        _get_amount_0_delta, get_next_sqrt_price_from_input, get_next_sqrt_price_from_output
    },
    tick_math::{get_sqrt_ratio_at_tick, get_tick_at_sqrt_ratio, MAX_TICK, MIN_TICK}
};

pub mod mock_block_stream;
pub mod pool;
pub mod pool_manager;
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
        let current_tick = get_tick_at_sqrt_ratio(sqrt_price_x96.into()).map_err(|_| {
            format!("Unable to get a tick from our current price '{:?}'", sqrt_price_x96)
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
            state:     self,
            range_idx: self.cur_tick_idx,
            tick:      self.current_tick,
            price:     self.sqrt_price_x96
        }
    }

    pub fn liquidity_at_tick(&self, tick: Tick) -> Option<u128> {
        self.get_range_for_tick(tick).map(|range| range.1.liquidity)
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
    state:     &'a MarketSnapshot,
    /// Index of the current PoolRange our price lies within
    range_idx: usize,
    /// Tick number within the current PoolRange that we're working with
    tick:      Tick,
    /// The ratio between Token0 and Token1
    price:     SqrtPriceX96
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
        let mut new_range_idx = self.range_idx;
        let mut pool = self.state.get_range(new_range_idx)?;
        let (mut tick_bound_price, next_tick) = if buy {
            let upper_tick_price = get_sqrt_ratio_at_tick(pool.upper_tick)
                .ok()
                .map(SqrtPriceX96::from)?;
            let next_tick = self.range_idx.checked_sub(1);
            (upper_tick_price, next_tick)
        } else {
            let lower_tick_price = get_sqrt_ratio_at_tick(pool.lower_tick)
                .ok()
                .map(SqrtPriceX96::from)?;
            let next_tick = self.range_idx.checked_add(1);
            (lower_tick_price, next_tick)
        };
        if self.price == tick_bound_price {
            // We're at the tick bound, we need to look at the next pool!
            new_range_idx = next_tick?;
            pool = self.state.get_range(new_range_idx)?;
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
        let quantity =
            _get_amount_0_delta(self.price.into(), closest_price.into(), pool.liquidity, false)
                .ok()?;
        let end_bound = Self {
            state:     self.state,
            price:     closest_price,
            range_idx: new_range_idx,
            tick:      get_tick_at_sqrt_ratio(closest_price.into()).ok()?
        };
        Some(PriceRange { start_bound: self.clone(), end_bound, quantity })
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
    pub quantity:    U256
}

impl<'a> PriceRange<'a> {
    pub fn quantity(&self, target_price: OrderPrice) -> U256 {
        let t: SqrtPriceX96 = Ray::from(*target_price).into();

        // If our target price is past our end bound, our quantity is the entire range
        if self.end_bound.price > self.start_bound.price {
            if t > self.end_bound.price {
                return self.quantity;
            }
        } else if t < self.end_bound.price {
            return self.quantity;
        }

        // Otherwise we have to calculate the precise quantity we have to sell
        _get_amount_0_delta(
            self.start_bound.price.into(),
            t.into(),
            self.start_bound.liquidity(),
            false
        )
        .unwrap_or(Uint::from(0))
    }

    // Maybe it's OK that I don't check the price again here because in the matching
    // algo I've only offered a quantity bounded by the price, so we should
    // always be OK?
    pub fn fill(&self, quantity: U256) -> Self {
        let liquidity = self.start_bound.liquidity();
        let end_sqrt_price = if self.end_bound.price > self.start_bound.price {
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
        let mut end_bound = self.start_bound.clone();
        end_bound.price = end_sqrt_price;
        Self { end_bound, start_bound: self.start_bound.clone(), quantity }
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
