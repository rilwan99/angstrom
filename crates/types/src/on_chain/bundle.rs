use std::collections::HashMap;

use ethers_core::types::U256;
use reth_primitives::{bytes::BytesMut, H256};
use reth_rlp::{Encodable, RlpDecodable, RlpEncodable};
use revm::primitives::{TransactTo, TxEnv, B160, U256 as RU256};
use serde::{Deserialize, Serialize};

use super::{
    CurrencySettlement, Order, PoolFees, PoolSwap, Signature, UniswapData, ANGSTROM_CONTRACT_ADDR
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct SignedVanillaBundle {
    pub bundle:     VanillaBundle,
    pub signatures: Vec<Signature>
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct VanillaBundle {
    orders:       Vec<Order>,
    uniswap_data: UniswapData
}

impl VanillaBundle {
    pub fn new(orders: Vec<Order>, uniswap_data: UniswapData) -> anyhow::Result<Self> {
        let non_vanilla = orders.iter().find(|order| {
            !order.order.details.pre_hook.is_empty() || !order.order.details.post_hook.is_empty()
        });

        if non_vanilla.is_some() {
            anyhow::bail!("found a non_villa order: {:?}", non_vanilla);
        }

        Ok(Self { orders, uniswap_data })
    }
}

impl From<VanillaBundle> for TxEnv {
    fn from(value: VanillaBundle) -> Self {
        todo!()
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct ComposableBundle {
    pub orders:       Vec<Order>,
    pub uniswap_data: UniswapData
}

impl From<ComposableBundle> for TxEnv {
    fn from(value: ComposableBundle) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct CallerInfo {
    pub address:   B160,
    pub nonce:     u64,
    pub overrides: HashMap<B160, HashMap<RU256, RU256>>
}
