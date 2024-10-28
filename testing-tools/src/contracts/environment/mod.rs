use alloy::{network::Ethereum, primitives::Address};

pub mod angstrom;
pub mod mockreward;
pub mod uniswap;

pub trait TestAnvilEnvironment {
    type T: Clone + Send + Sync + alloy::transports::Transport;
    type P: alloy::providers::Provider<Self::T, Ethereum>;

    fn provider(&self) -> &Self::P;
    fn controller(&self) -> Address;
}
