// Let's define what, precisely, an Order even is

// An order should be described as a quantity that can be sold against a pricing curve
// When trying to fill an order we will then need to give it two stops - a quantity stop
// and a pricing stop

// We can then determine whether an order has been entirely filled or not

use crate::cfmm::uniswap::PriceRange;

#[derive(Clone)]
pub enum OrderOutcome {
    /// The order has not yet been processed
    Unfilled,
    /// The order has been completely filled
    CompleteFill,
    /// The order has been partially filled (and how much)
    PartialFill(f64),
    /// We have dropped this order, it can not be filled
    Killed
}

impl OrderOutcome {
    pub fn partial_fill(&self, quantity: f64) -> Self {
        match self {
            Self::Unfilled => Self::PartialFill(quantity),
            Self::CompleteFill => Self::CompleteFill,
            Self::Killed => Self::Killed,
            Self::PartialFill(f) => Self::PartialFill(f + quantity)
        }
    }
}

#[derive(Clone)]
pub struct LimitOrder { price: f64, quantity: f64 }

#[derive(Clone)]
pub enum Order<'a> {
    KillOrFill(LimitOrder),
    PartialFill(LimitOrder),
    AMM(PriceRange<'a>)
}

impl<'a> Order<'a> {
    /// Retrieve the quantity available within the bounds of a given order
    pub fn quantity(&self, limit_price: f64) -> f64 {
        match self {
            Self::KillOrFill(lo) => lo.quantity,
            Self::PartialFill(lo) => lo.quantity,
            Self::AMM(ammo) => ammo.quantity(limit_price),
        }
    }

    /// Retrieve the price for a given order
    pub fn price(&self) -> f64 {
        match self {
            Self::KillOrFill(lo) => lo.price,
            Self::PartialFill(lo) => lo.price,
            Self::AMM(ammo) => ammo.start_bound.as_float()
        }
    }

    /// Produce a new order representing the remainder of the current order after the fill operation has been performed
    /// We need the target price to make sure to bound our AMM orders
    pub fn fill(&self, filled_quantity: f64) -> Self {
        match self {
            Self::KillOrFill(lo) => Self::KillOrFill(LimitOrder { price: lo.price, quantity: lo.quantity - filled_quantity}),
            Self::PartialFill(lo) => Self::PartialFill(LimitOrder { price: lo.price, quantity: lo.quantity - filled_quantity}),
            Self::AMM(r) => {
                r.fill(filled_quantity);
                Self::PartialFill(LimitOrder { quantity: 0.0, price: 0.0 })
            }
        }
    }
}