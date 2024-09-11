use std::{
    collections::HashMap,
    ops::{Deref, DerefMut}
};

use alloy_primitives::{Address, TxHash, U256};
use angstrom_types::{
    orders::OrderLocation,
    sol_bindings::{ext::RawPoolOrder, grouped_orders::OrderWithStorageData, sol::AssetIndex}
};
use dashmap::DashMap;
use reth_primitives::B256;

use super::UserOrderPoolInfo;

#[derive(Clone, Debug, Default)]
pub struct AssetIndexToAddress(DashMap<Address, u16>);

#[derive(Debug, Clone)]
pub struct AssetIndexToAddressWrapper<Order: RawPoolOrder> {
    pub asset_in:  u16,
    pub asset_out: u16,
    pub order:     Order
}

impl<Order: RawPoolOrder> Deref for AssetIndexToAddressWrapper<Order> {
    type Target = Order;

    fn deref(&self) -> &Self::Target {
        &self.order
    }
}

impl<Order: RawPoolOrder> DerefMut for AssetIndexToAddressWrapper<Order> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.order
    }
}

impl<Order: RawPoolOrder> AssetIndexToAddressWrapper<Order> {
    pub fn into_order_storage_with_data(
        self,
        block: u64,
        is_cur_valid: bool,
        is_valid: bool,
        is_limit: bool,
        pool_info: UserOrderPoolInfo,
        invalidates: Vec<B256>
    ) -> OrderWithStorageData<Order> {
        OrderWithStorageData {
            priority_data: angstrom_types::orders::OrderPriorityData {
                price:  self.limit_price(),
                volume: self.amount_in(),
                gas:    0
            },
            pool_id: pool_info.pool_id,
            is_currently_valid: is_cur_valid,
            is_bid: pool_info.is_bid,
            is_valid,
            valid_block: block,
            order_id: angstrom_types::orders::OrderId {
                address:  self.from(),
                pool_id:  pool_info.pool_id,
                hash:     self.hash(),
                nonce:    self.nonce(),
                deadline: self.deadline(),
                location: if is_limit { OrderLocation::Limit } else { OrderLocation::Searcher }
            },
            invalidates,
            order: self.order
        }
    }
}

impl AssetIndexToAddress {
    pub fn new(map: DashMap<u16, Address>) -> Self {
        Self(map)
    }

    pub fn get_index(&self, address: &Address) -> Option<u16> {
        self.0.get(address).map(|f| *f)
    }

    pub fn wrap<Order: RawPoolOrder>(
        &self,
        order: Order
    ) -> Option<AssetIndexToAddressWrapper<Order>> {
        let asset_in = self.get_index(&order.token_in())?;
        let asset_out = self.get_index(&order.token_out())?;

        Some(AssetIndexToAddressWrapper { asset_in, asset_out, order })
    }
}
