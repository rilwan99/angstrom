use bytes::Bytes;
use ethers_core::{
    types::{Address, H256, U256},
    utils::keccak256
};
use hex_literal::hex;
use reth_primitives::bytes::BytesMut;
use reth_rlp::{Encodable as REncodable, RlpDecodable, RlpEncodable};
use revm::primitives::{TransactTo, TxEnv};
use serde::{Deserialize, Serialize};

use super::{HookSim, Signature, ANGSTROM_CONTRACT_ADDR};

const SEARCHER_TYPE_HASH: H256 =
    H256(hex!("ef33cd4248bb6f0d81cb687a9d2a94598fe7a9e3053889af630fb69125b4ac69"));

/// struct ArbitrageOrder {
///     PoolId pool;
///     Currency tokenIn;
///     Currency tokenOut;
///     uint128 amountIn;
///     uint128 amountOutMin;
///     uint256 deadline;
///     uint256 gasBid;
///     uint256 bribe;
///     bytes preHook;
///     bytes postHook;
/// }
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct SearcherOrder {
    pub pool:           [u8; 32],
    pub token_in:       Address,
    pub token_out:      Address,
    pub amount_in:      u128,
    pub amount_out_min: u128,
    pub deadline:       U256,
    pub gas_cap:        U256,
    pub bribe:          U256,
    pub pre_hook:       Bytes,
    pub post_hook:      Bytes
}

/// struct LvrSettlement {
///     // User provided.
///     SearcherOrder order;
///     bytes signature;
///     // Guard provided.
///     uint256 gasActual;
/// }
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct RawLvrSettlement {
    pub order:     SearcherOrder,
    pub signature: Signature
}

/// struct LimitOrder {
///     PoolId pool;
///     Currency tokenIn;
///     Currency tokenOut;
///     uint128 Price;
///     uint128 amount;
///     uint128 amountOut;
///     uint256 blockNumber;
///     uint256 gasBid;
///     uint256 bribe;
///     bytes preHook;
///     bytes postHook;
/// }

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct LimitOrder {
    pub pool:        [u8; 32],
    pub token_in:    Address,
    pub token_out:   Address,
    pub amount_in:   u128,
    pub price:       u128,
    pub blockNumber: U256,
    pub gas_cap:     U256,
    pub bribe:       U256
}

impl TryInto<HookSim> for RawLvrSettlement {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<HookSim, Self::Error> {
        let mut msg = Vec::new();
        msg.extend(SEARCHER_TYPE_HASH.to_fixed_bytes());
        msg.extend(self.order.pool);
        msg.extend(self.order.token_in.to_fixed_bytes());
        msg.extend(self.order.token_out.to_fixed_bytes());
        msg.extend(self.order.amount_in.to_be_bytes());
        msg.extend(self.order.amount_out_min.to_be_bytes());

        let mut deadbuf = BytesMut::new();
        self.order.deadline.to_big_endian(&mut deadbuf);
        msg.extend(deadbuf.to_vec());
        let mut bribe = BytesMut::new();
        self.order.bribe.to_big_endian(&mut bribe);
        msg.extend(bribe.to_vec());
        msg.extend(keccak256(&self.order.pre_hook));
        msg.extend(keccak256(&self.order.post_hook));

        let digest = keccak256(msg);
        let addr = self.signature.recover(digest)?;

        Ok(HookSim {
            tx: super::SearcherOrUser::Searcher(self.clone()),
            pre_hook: self.order.pre_hook,
            amount_in_req: self.order.amount_in,
            amount_in_token: self.order.token_in,
            post_hook: self.order.post_hook,
            amount_out_lim: self.order.amount_out_min,
            amount_out_token: self.order.token_out,
            addr
        })
    }
}

impl From<RawLvrSettlement> for H256 {
    fn from(value: RawLvrSettlement) -> Self {
        let mut buf = BytesMut::new();
        REncodable::encode(&value, &mut buf);

        H256(ethers_core::utils::keccak256(buf))
    }
}

impl Into<TxEnv> for RawLvrSettlement {
    fn into(self) -> TxEnv {
        let mut data = BytesMut::new();
        REncodable::encode(&self, &mut data);

        TxEnv {
            caller:           Default::default(),
            gas_limit:        self.order.gas_cap.as_u64(),
            gas_price:        reth_primitives::U256::ZERO,
            gas_priority_fee: None,
            transact_to:      TransactTo::Call(ANGSTROM_CONTRACT_ADDR.into()),
            value:            reth_primitives::U256::ZERO,
            data:             data.to_vec().into(),
            chain_id:         Some(1),
            nonce:            None,
            access_list:      Default::default()
        }
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
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct SimmedLvrSettlement {
    pub raw:        RawLvrSettlement,
    pub gas_actual: U256
}

impl From<SimmedLvrSettlement> for RawLvrSettlement {
    fn from(value: SimmedLvrSettlement) -> Self {
        value.raw
    }
}

impl From<SimmedLvrSettlement> for H256 {
    fn from(value: SimmedLvrSettlement) -> Self {
        let mut buf = BytesMut::new();
        REncodable::encode(&value, &mut buf);

        H256(ethers_core::utils::keccak256(buf))
    }
}
