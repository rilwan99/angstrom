use bytes::Bytes;
use ethers_core::{
    types::{Address, H256, U256},
    utils::keccak256
};
use hex_literal::hex;
use reth_primitives::bytes::BytesMut;
use reth_rlp::{Encodable, RlpDecodable, RlpEncodable};
use revm::primitives::{TransactTo, TxEnv};
use serde::{Deserialize, Serialize};

use crate::{HookSim, Signature, ANGSTROM_CONTRACT_ADDR};

const USER_TYPE_HASH: H256 =
    H256(hex!("5c886e27068c81ac66cd398205116e348b8a76cdb54d88c62fb46bb255f15ac7"));
/// struct UserOrder {
///     Currency tokenIn;
///     Currency tokenOut;
///     uint128 amountIn;
///     uint128 amountOutMin;
///     uint256 deadline;
///     uint256 gasCap;
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
pub struct UserOrder {
    pub token_out:      Address,
    pub token_in:       Address,
    pub amount_in:      u128,
    pub amount_out_min: u128,
    pub deadline:       U256,
    pub gas_cap:        U256,
    pub pre_hook:       Bytes,
    pub post_hook:      Bytes
}

impl UserOrder {}

/// struct UserSettlement {
///     // User provided.
///     UserOrder order;
///     bytes signature;
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
pub struct RawUserSettlement {
    pub order:     UserOrder,
    pub signature: Signature
}

impl TryInto<HookSim> for RawUserSettlement {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<HookSim, Self::Error> {
        let mut msg = Vec::new();
        msg.extend(USER_TYPE_HASH.to_fixed_bytes());
        msg.extend(self.order.token_in.to_fixed_bytes());
        msg.extend(self.order.token_out.to_fixed_bytes());
        msg.extend(self.order.amount_in.to_be_bytes());
        msg.extend(self.order.amount_out_min.to_be_bytes());

        let mut deadbuf = BytesMut::new();
        self.order.deadline.to_big_endian(&mut deadbuf);
        msg.extend(deadbuf.to_vec());
        let mut bribe = BytesMut::new();
        self.order.gas_cap.to_big_endian(&mut bribe);
        msg.extend(bribe.to_vec());
        msg.extend(keccak256(&self.order.pre_hook));
        msg.extend(keccak256(&self.order.post_hook));

        let digest = keccak256(msg);
        let addr = self.signature.recover(digest)?;

        Ok(HookSim {
            tx: crate::SearcherOrUser::User(self.clone()),
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

impl From<RawUserSettlement> for H256 {
    fn from(value: RawUserSettlement) -> Self {
        let mut buf = BytesMut::new();
        value.encode(&mut buf);

        H256(ethers_core::utils::keccak256(buf))
    }
}

impl Into<TxEnv> for RawUserSettlement {
    fn into(self) -> TxEnv {
        let mut data = BytesMut::new();
        self.encode(&mut data);

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
pub struct SimmedUserSettlement {
    pub raw:               RawUserSettlement,
    pub amount_out:        U256,
    pub amount_gas_actual: U256
}

impl From<SimmedUserSettlement> for H256 {
    fn from(value: SimmedUserSettlement) -> Self {
        let mut buf = BytesMut::new();
        value.encode(&mut buf);

        H256(ethers_core::utils::keccak256(buf))
    }
}

impl From<SimmedUserSettlement> for RawUserSettlement {
    fn from(value: SimmedUserSettlement) -> Self {
        value.raw
    }
}
