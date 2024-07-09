use angstrom_types::sol_bindings::sol::{FlashOrder, StandingOrder};

use super::{BookID, OrderID, OrderPrice, OrderVolume};
/// Definition of the various types of order that we can serve, as well as the
/// outcomes we're able to have for them
use crate::cfmm::uniswap::PriceRange;

#[derive(Clone, Debug)]
pub struct OrderCoordinate {
    pub book:  BookID,
    pub order: OrderID
}

#[derive(Clone, Debug)]
pub enum OrderDirection {
    Bid,
    Ask
}

impl OrderDirection {
    pub fn is_bid(&self) -> bool {
        match self {
            OrderDirection::Bid => true,
            OrderDirection::Ask => false
        }
    }

    pub fn is_ask(&self) -> bool {
        match self {
            OrderDirection::Bid => false,
            OrderDirection::Ask => true
        }
    }
}

#[derive(Clone, Debug)]
pub enum OrderExclusion {
    Live(usize),
    Dead(usize)
}

impl OrderExclusion {
    pub fn flip(&self) -> Self {
        match self {
            Self::Live(ttl) => Self::Dead(ttl + 1),
            Self::Dead(ttl) => Self::Live(ttl + 1)
        }
    }

    pub fn ttl(&self) -> usize {
        match self {
            Self::Live(ttl) | Self::Dead(ttl) => *ttl
        }
    }

    pub fn is_live(&self) -> bool {
        matches!(self, Self::Live(_))
    }

    pub fn is_dead(&self) -> bool {
        matches!(self, Self::Dead(_))
    }
}

#[derive(Clone, Debug)]
pub enum OrderOutcome {
    /// The order has not yet been processed
    Unfilled,
    /// The order has been completely filled
    CompleteFill,
    /// The order has been partially filled (and how much)
    PartialFill(OrderVolume),
    /// We have dropped this order, it can not or should not be filled.
    Killed
}

impl OrderOutcome {
    pub fn is_filled(&self) -> bool {
        matches!(self, Self::CompleteFill | Self::PartialFill(_))
    }

    pub fn partial_fill(&self, quantity: OrderVolume) -> Self {
        match self {
            Self::Unfilled => Self::PartialFill(quantity),
            Self::PartialFill(f) => Self::PartialFill(f + quantity),
            Self::CompleteFill | Self::Killed => self.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Order<'a> {
    KillOrFill(FlashOrder),
    PartialFill(StandingOrder),
    AMM(PriceRange<'a>)
}

impl<'a> Order<'a> {
    /// Determine if this is an AMM order
    pub fn is_amm(&self) -> bool {
        matches!(self, Self::AMM(_))
    }

    pub fn id(&self) -> Option<OrderID> {
        match self {
            Self::KillOrFill(_) => Some(0),
            Self::PartialFill(_) => Some(0),
            _ => None
        }
    }

    pub fn related(&self) -> Option<&Vec<OrderCoordinate>> {
        match self {
            Self::KillOrFill(_) => None,
            Self::PartialFill(_) => None,
            _ => None
        }
    }

    /// Retrieve the quantity available within the bounds of a given order
    pub fn quantity(&self, limit_price: OrderPrice) -> OrderVolume {
        match self {
            Self::KillOrFill(lo) => lo.max_amount_in_or_out,
            Self::PartialFill(lo) => lo.max_amount_in_or_out,
            Self::AMM(ammo) => ammo.quantity(limit_price)
        }
    }

    /// Retrieve the price for a given order
    pub fn price(&self) -> OrderPrice {
        match self {
            Self::KillOrFill(lo) => lo.min_price,
            Self::PartialFill(lo) => lo.min_price,
            Self::AMM(ammo) => ammo.start_bound.as_u256()
        }
    }

    /// Produce a new order representing the remainder of the current order
    /// after the fill operation has been performed
    pub fn fill(&self, filled_quantity: OrderVolume) -> Self {
        match self {
            Self::KillOrFill(lo) => Self::KillOrFill(FlashOrder {
                max_amount_in_or_out: lo.max_amount_in_or_out - filled_quantity,
                ..lo.clone()
            }),
            Self::PartialFill(lo) => Self::PartialFill(StandingOrder {
                max_amount_in_or_out: lo.max_amount_in_or_out - filled_quantity,
                ..lo.clone()
            }),
            Self::AMM(r) => {
                r.fill(filled_quantity);
                // Return a bogus order that we never use
                Self::PartialFill(StandingOrder::default())
            }
        }
    }
}
