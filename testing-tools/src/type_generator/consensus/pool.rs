use alloy_primitives::{
    aliases::{I24, U24},
    Address
};
use angstrom_types::{
    matching::uniswap::PoolSnapshot,
    primitive::PoolKey,
    sol_bindings::{
        grouped_orders::{GroupedUserOrder, GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};

#[derive(Debug)]
pub enum OrderType {
    FixedOrders(Vec<OrderWithStorageData<GroupedVanillaOrder>>),
    GeneratedOrders(usize)
}

impl Default for OrderType {
    fn default() -> Self {
        Self::FixedOrders(vec![])
    }
}

#[derive(Default, Debug)]
pub struct PoolBuilder {
    key:  Option<PoolKey>,
    amm:  Option<PoolSnapshot>,
    tob:  Option<TopOfBlockOrder>,
    book: Option<OrderType>
}

impl PoolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_key(self, key: PoolKey) -> Self {
        Self { key: Some(key), ..self }
    }
}

pub fn create_key(token0: Address, token1: Address, tick_spacing: i32) -> PoolKey {
    let (currency0, currency1) = if token0 < token1 { (token0, token1) } else { (token1, token0) };
    PoolKey {
        currency0,
        currency1,
        fee: U24::ZERO,
        tickSpacing: I24::unchecked_from(tick_spacing),
        hooks: Address::random()
    }
}
