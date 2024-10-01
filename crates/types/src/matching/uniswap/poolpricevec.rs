use alloy::primitives::{Uint, U256};
use uniswap_v3_math::sqrt_price_math::{
    _get_amount_0_delta, _get_amount_1_delta, get_next_sqrt_price_from_input,
    get_next_sqrt_price_from_output
};

use super::poolprice::PoolPrice;
use crate::{
    matching::{Ray, SqrtPriceX96},
    orders::OrderPrice
};

#[derive(Clone, Debug)]
pub struct PoolPriceVec<'a> {
    pub start_bound: PoolPrice<'a>,
    pub end_bound:   PoolPrice<'a>,
    pub d_t0:        U256,
    pub d_t1:        U256
}

impl<'a> PoolPriceVec<'a> {
    pub fn new(start_bound: PoolPrice<'a>, end_bound: PoolPrice<'a>) -> Self {
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
