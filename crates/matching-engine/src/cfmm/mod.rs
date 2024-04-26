//! The Underlying V4 cfmm curve. Transformed into a clob with N ticks
//! on each side.

#![allow(unused)]
use alloy_primitives::Address;

/// Tick that is split in its price
struct ActiveTick {
    inner: PassiveTick,
    price: f64
}

impl ActiveTick {
    fn get_x_comp_at_price(&self, price: f64) -> f64 {
        (self.inner.liquidity
            * (-price * self.inner.upper.sqrt() + self.inner.upper * price.sqrt()))
            / (price * self.inner.upper)
    }

    fn get_y_comp_at_price(&self, price: f64) -> f64 {
        -self.inner.liquidity * self.inner.lower.sqrt() + self.inner.liquidity * price.sqrt()
    }

    // amount to sell to reach tick edge
    fn full_sell_amount(&self) -> f64 {
        self.inner.y_given_x(0.0) - self.inner.y_given_x(self.get_x_comp_at_price(self.price))
    }

    /// amount to buy to reach tick edge
    fn full_buy_amount(&self) -> f64 {
        self.inner.x_given_y(0.0) - self.inner.x_given_y(self.get_y_comp_at_price(self.price))
    }

    // the new price of the tick given we are buying some amount.
    // NOTE: this assumes tick overflow checks have been done
    pub fn new_price_buy(&self, amount: f64) -> f64 {
        // buy token 0
        let dx = self.get_x_comp_at_price(self.price) + amount;
        // token 1 sell
        let dy = self.get_y_comp_at_price(self.price) - self.inner.y_given_x(dx);
        self.inner.price(dx, dy)
    }

    // the new price of the tick given we are buying some amount.
    // NOTE: this assumes tick overflow checks have been done
    pub fn new_price_sell(&self, amount: f64) -> f64 {
        // buy token 0
        let dy = self.get_y_comp_at_price(self.price) + amount;
        // token 1 sell
        let dx = self.get_x_comp_at_price(self.price) - self.inner.x_given_y(dy);
        self.inner.price(dx, dy)
    }
}

/// Tick that isn't split in its price
struct PassiveTick {
    // tick bounds
    lower:     f64,
    upper:     f64,
    token1:    f64,
    liquidity: f64
}

impl PassiveTick {
    // the x component for the given tick.
    fn x_given_y(&self, y: f64) -> f64 {
        (-self.lower.sqrt() * self.liquidity.powi(2) + (self.upper.sqrt() * self.liquidity.powi(2))
            - self.liquidity * y)
            / (self.upper.sqrt() * (self.lower.sqrt() * self.liquidity + y))
    }

    /// the amount of token 0 to buy to fully cross this tick
    fn y_given_x(&self, x: f64) -> f64 {
        ((-self.lower.sqrt() * self.upper.sqrt() * self.liquidity * x)
            - (self.lower.sqrt() * self.liquidity.powi(2)
                + self.upper.sqrt() * self.liquidity.powi(2)))
            / (self.upper.sqrt() * x + self.liquidity)
    }

    fn price(&self, x: f64, y: f64) -> f64 {
        (y + self.liquidity * self.lower.sqrt()) / ((self.liquidity / self.upper.sqrt()) + x)
    }

    /// NOTE: assumes tick is at the upper tick
    fn partial_buy_price(&self, amount: f64) -> f64 {
        // becuase x at zero, dx = amount;
        let dy = self.y_given_x(0.0) - self.y_given_x(amount);
        self.price(amount, dy)
    }

    /// NOTE: assumes tick is at lower tick
    fn partial_sell_price(&self, amount: f64) -> f64 {
        let dx = self.x_given_y(0.0) - self.x_given_y(amount);
        self.price(dx, amount)
    }
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
