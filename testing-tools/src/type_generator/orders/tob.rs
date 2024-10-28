use alloy::primitives::Address;
use angstrom_types::sol_bindings::rpc_orders::TopOfBlockOrder;

#[derive(Default, Debug)]
pub struct ToBOrderBuilder {
    asset_in:     Option<Address>,
    asset_out:    Option<Address>,
    quantity_in:  Option<u128>,
    quantity_out: Option<u128>,
    valid_block:  Option<u64>
}

impl ToBOrderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn asset_in(self, asset_in: Address) -> Self {
        Self { asset_in: Some(asset_in), ..self }
    }

    pub fn asset_out(self, asset_out: Address) -> Self {
        Self { asset_out: Some(asset_out), ..self }
    }

    pub fn quantity_in(self, quantity_in: u128) -> Self {
        Self { quantity_in: Some(quantity_in), ..self }
    }

    pub fn quantity_out(self, quantity_out: u128) -> Self {
        Self { quantity_out: Some(quantity_out), ..self }
    }

    pub fn valid_block(self, valid_block: u64) -> Self {
        Self { valid_block: Some(valid_block), ..self }
    }

    pub fn build(self) -> TopOfBlockOrder {
        TopOfBlockOrder {
            asset_in: self.asset_in.unwrap_or_default(),
            asset_out: self.asset_out.unwrap_or_default(),
            quantity_in: self.quantity_in.unwrap_or_default(),
            quantity_out: self.quantity_out.unwrap_or_default(),
            valid_for_block: self.valid_block.unwrap_or_default(),
            ..Default::default()
        }
    }
}
