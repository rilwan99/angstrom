use alloy::primitives::Address;
use angstrom_types::{
    matching::Ray,
    sol_bindings::{
        grouped_orders::{FlashVariants, GroupedVanillaOrder, StandingVariants},
        rpc_orders::{
            ExactFlashOrder, ExactStandingOrder, PartialFlashOrder, PartialStandingOrder
        }
    }
};

use super::StoredOrderBuilder;

#[derive(Clone, Debug, Default)]
pub struct UserOrderBuilder {
    /// If the order is not a Standing order, it is KillOrFill
    is_standing: bool,
    /// If the order is not an Exact order, it is Partial
    is_exact:    bool,
    block:       u64,
    nonce:       u64,
    recipient:   Address,
    asset_in:    Address,
    asset_out:   Address,
    amount:      u128,
    min_price:   Ray
}

impl UserOrderBuilder {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn standing(self) -> Self {
        Self { is_standing: true, ..self }
    }

    pub fn kill_or_fill(self) -> Self {
        Self { is_standing: false, ..self }
    }

    /// Sets the order to be kill-or-fill or standing by explicit boolean
    pub fn is_standing(self, is_standing: bool) -> Self {
        Self { is_standing, ..self }
    }

    pub fn exact(self) -> Self {
        Self { is_exact: true, ..self }
    }

    pub fn partial(self) -> Self {
        Self { is_exact: false, ..self }
    }

    /// Sets the order to be exact or partial by explicit boolean
    pub fn is_exact(self, is_exact: bool) -> Self {
        Self { is_exact, ..self }
    }

    pub fn block(self, block: u64) -> Self {
        Self { block, ..self }
    }

    pub fn nonce(self, nonce: u64) -> Self {
        Self { nonce, ..self }
    }

    pub fn recipient(self, recipient: Address) -> Self {
        Self { recipient, ..self }
    }

    pub fn asset_in(self, asset_in: Address) -> Self {
        Self { asset_in, ..self }
    }

    pub fn asset_out(self, asset_out: Address) -> Self {
        Self { asset_out, ..self }
    }

    pub fn amount(self, amount: u128) -> Self {
        Self { amount, ..self }
    }

    pub fn min_price(self, min_price: Ray) -> Self {
        Self { min_price, ..self }
    }

    pub fn build(self) -> GroupedVanillaOrder {
        match (self.is_standing, self.is_exact) {
            (true, true) => {
                let order = ExactStandingOrder {
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    amount: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    nonce: self.nonce,
                    ..Default::default()
                };
                GroupedVanillaOrder::Standing(StandingVariants::Exact(order))
            }
            (true, false) => {
                let order = PartialStandingOrder {
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    maxAmountIn: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    ..Default::default()
                };
                GroupedVanillaOrder::Standing(StandingVariants::Partial(order))
            }
            (false, true) => {
                let order = ExactFlashOrder {
                    validForBlock: self.block,
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    amount: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    ..Default::default()
                };
                GroupedVanillaOrder::KillOrFill(FlashVariants::Exact(order))
            }
            (false, false) => {
                let order = PartialFlashOrder {
                    validForBlock: self.block,
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    maxAmountIn: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    ..Default::default()
                };
                GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(order))
            }
        }
    }

    /// Lets us chain right into the storage wrapper
    pub fn with_storage(self) -> StoredOrderBuilder {
        let block = self.block;
        StoredOrderBuilder::new(self.build()).valid_block(block)
    }
}
