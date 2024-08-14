use bincode::{Decode, Encode};

use super::OrderVolume;

#[derive(Clone, Debug, Default, PartialEq, Eq, Encode, Decode)]
pub enum OrderFillState {
    /// The order has not yet been processed
    #[default]
    Unfilled,
    /// The order has been completely filled
    CompleteFill,
    /// The order has been partially filled (and how much)
    PartialFill(#[bincode(with_serde)] OrderVolume),
    /// We have dropped this order, it can not or should not be filled.
    Killed
}

impl OrderFillState {
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
