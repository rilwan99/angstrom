use alloy_primitives::{
    aliases::{I24, U24},
    Address
};
use angstrom_types::{
    matching::{
        uniswap::{PoolPrice, PoolSnapshot},
        SqrtPriceX96
    },
    primitive::{PoolId, PoolKey},
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

#[derive(Clone, Debug)]
pub struct Pool {
    key:      PoolKey,
    snapshot: PoolSnapshot
}

impl Pool {
    pub fn new(key: PoolKey, snapshot: PoolSnapshot) -> Self {
        Self { key, snapshot }
    }

    pub fn price(&self) -> PoolPrice {
        self.snapshot.current_price()
    }

    pub fn id(&self) -> PoolId {
        self.key.clone().into()
    }

    pub fn token0(&self) -> Address {
        self.key.currency0
    }

    pub fn token1(&self) -> Address {
        self.key.currency1
    }
}

#[derive(Default, Debug)]
pub struct PoolBuilder {
    key: Option<PoolKey>,
    amm: Option<PoolSnapshot>
}

impl PoolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    fn random_key() -> PoolKey {
        PoolKey {
            currency0:   Address::random(),
            currency1:   Address::random(),
            fee:         U24::ZERO,
            tickSpacing: I24::unchecked_from(10),
            hooks:       Address::default()
        }
    }

    fn random_snapshot() -> PoolSnapshot {
        let price = SqrtPriceX96::at_tick(100000).unwrap();
        let ranges = vec![];
        PoolSnapshot::new(ranges, price).unwrap()
    }

    pub fn with_key(self, key: PoolKey) -> Self {
        Self { key: Some(key), ..self }
    }

    pub fn snapshot(self, snapshot: PoolSnapshot) -> Self {
        Self { amm: Some(snapshot), ..self }
    }

    pub fn build(self) -> Pool {
        let key = self.key.unwrap_or_else(|| Self::random_key());
        let snapshot = self.amm.unwrap_or_else(|| Self::random_snapshot());
        Pool { key, snapshot }
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
