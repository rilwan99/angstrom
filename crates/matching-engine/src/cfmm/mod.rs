//! The Underlying V4 cfmm curve. Transformed into a clob with N ticks
//! on each side.

use alloy_primitives::Address;

/// Tick that is split in its price
struct ActiveTick {
    inner: PassiveTick,
    price: f64
}

impl ActiveTick {
    pub fn price_to_bound_sell(&self) -> f64 {}
}

/// Tick that isn't split in its price
struct PassiveTick {
    // tick bounds
    lower:     f64,
    upper:     f64,
    // virtual liq
    token0:    f64,
    // virtual liq
    token1:    f64,
    liquidity: f64
}

impl PassiveTick {
    /// the amount of token0 to sell to fully cross this tick
    fn total_tick_sell_quantity(&self) -> f64 {
        (-self.lower.sqrt() * self.liquidity.powi(2) + (self.upper.sqrt() * self.liquidity.powi(2)))
            / (self.upper.sqrt() * (self.lower.sqrt() * self.liquidity))
    }

    /// the amount of token 0 to buy to fully cross this tick
    fn total_tick_buy_quantity(&self) -> f64 {
        (self.lower.sqrt() * self.liquidity.powi(2) + self.upper.sqrt() * self.liquidity.powi(2))
            / self.liquidity
    }

    /// NOTE: assumes tick is at the upper tick
    fn partial_buy_price(&self, amount: f64) -> f64 {
        let 

    }

    /// NOTE: assumes tick is at lower tick
    fn partial_sell_price(&self, amount: f64) -> f64 {}
}

/// UniswapV4 Ticks in a book format.
/// Math: https://www.desmos.com/calculator/vabrklymy2
pub struct UniswapV4Book<const BOUND: usize> {
    token0: Address,
    token1: Address,
    /// the current split tick
    mid:    ActiveTick,
    /// complete bid ticks,
    bids:   [PassiveTick; BOUND],
    /// complete ask ticks,
    asks:   [PassiveTick; BOUND]
}
