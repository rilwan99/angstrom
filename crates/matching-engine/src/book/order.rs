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
        if let Self::Live(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_dead(&self) -> bool {
        if let Self::Dead(_) = self {
            true
        } else {
            false
        }
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
        match self {
            Self::CompleteFill | Self::PartialFill(_) => true,
            _ => false
        }
    }

    pub fn partial_fill(&self, quantity: f64) -> Self {
        match self {
            Self::Unfilled => Self::PartialFill(quantity),
            Self::PartialFill(f) => Self::PartialFill(f + quantity),
            Self::CompleteFill | Self::Killed => self.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct LimitOrder {
    id:       OrderID,
    related:  Option<Vec<OrderCoordinate>>,
    price:    OrderPrice,
    quantity: OrderVolume
}

impl LimitOrder {
    pub fn new(price: OrderPrice, quantity: OrderVolume) -> Self {
        Self { id: 10, price, quantity, related: None }
    }

    pub fn related(&self) -> Option<&Vec<OrderCoordinate>> {
        self.related.as_ref()
    }

    pub fn id(&self) -> OrderID {
        self.id
    }
}

#[derive(Clone, Debug)]
pub enum Order<'a> {
    KillOrFill(LimitOrder),
    PartialFill(LimitOrder),
    AMM(PriceRange<'a>)
}

impl<'a> Order<'a> {
    /// Determine if this is an AMM order
    pub fn is_amm(&self) -> bool {
        match self {
            Self::AMM(_) => true,
            _ => false
        }
    }

    pub fn id(&self) -> Option<OrderID> {
        match self {
            Self::KillOrFill(l) | Self::PartialFill(l) => Some(l.id),
            _ => None
        }
    }

    pub fn related(&self) -> Option<&Vec<OrderCoordinate>> {
        match self {
            Self::KillOrFill(l) | Self::PartialFill(l) => l.related.as_ref(),
            _ => None
        }
    }

    /// Retrieve the quantity available within the bounds of a given order
    pub fn quantity(&self, limit_price: OrderPrice) -> OrderVolume {
        match self {
            Self::KillOrFill(lo) => lo.quantity,
            Self::PartialFill(lo) => lo.quantity,
            Self::AMM(ammo) => ammo.quantity(limit_price)
        }
    }

    /// Retrieve the price for a given order
    pub fn price(&self) -> OrderPrice {
        match self {
            Self::KillOrFill(lo) => lo.price,
            Self::PartialFill(lo) => lo.price,
            Self::AMM(ammo) => ammo.start_bound.as_float()
        }
    }

    /// Produce a new order representing the remainder of the current order
    /// after the fill operation has been performed
    pub fn fill(&self, filled_quantity: OrderVolume) -> Self {
        match self {
            Self::KillOrFill(lo) => Self::KillOrFill(LimitOrder {
                id:       0,
                related:  None,
                price:    lo.price,
                quantity: lo.quantity - filled_quantity
            }),
            Self::PartialFill(lo) => Self::PartialFill(LimitOrder {
                id:       0,
                related:  None,
                price:    lo.price,
                quantity: lo.quantity - filled_quantity
            }),
            Self::AMM(r) => {
                r.fill(filled_quantity);
                // Return a bogus order that we never use
                Self::PartialFill(LimitOrder {
                    id:       0,
                    related:  None,
                    quantity: 0.0,
                    price:    0.0
                })
            }
        }
    }
}
