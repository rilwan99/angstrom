use ethers_core::{
    types::{Address, I256, U256},
    utils::keccak256
};
use reth_primitives::bytes;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

/// Uniswap instructions to execute after lock is taken
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
pub struct UniswapData {
    /// The discrete swap to perform, there should be at most one entry per
    /// pool.
    pub swaps:      Vec<PoolSwap>,
    /// The currency settlements to perform, there should be at most one entry
    /// per currency.
    pub currencies: Vec<CurrencySettlement>,
    /// the fees to pay to each pool, there should be at most one entry per
    /// pool.
    pub pools:      Vec<PoolFees>
}

/// struct PoolSettlement {
///     PoolKey pool;
///     uint256 token0In;
///     uint256 token1In;
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
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct PoolSettlement {
    pub pool:       PoolKey,
    pub token_0_in: U256,
    pub token_1_in: U256
}

/// struct PoolSwap {
///     PoolKey pool;
///     Currency currencyIn;
///     uint256 amountIn;
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
pub struct PoolSwap {
    pool:        PoolKey,
    currency_in: Address,
    amount_in:   U256
}

/// struct PoolKey {
///     Currency currency0;
///     Currency currency1;
///     uint24 fee;
///     int24 tickSpacing;
///     address hooks;
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
pub struct PoolKey {
    pub currency_0:   Address,
    pub currency_1:   Address,
    pub fee:          u32,
    pub tick_spacing: u32,
    pub hooks:        Address
}

impl From<PoolKey> for [u8; 32] {
    fn from(value: PoolKey) -> Self {
        keccak256(ethers::abi::AbiEncode::encode(value))
    }
}

/// struct PoolFees {
///     PoolKey pool;
///     uint256 fees0;
///     uint256 fees1;
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
pub struct PoolFees {
    pool:   PoolKey,
    fees_0: U256,
    fees_1: U256
}

/// struct CurrencySettlement {
///     Currency currency;
///     int256 amountNet;
/// }
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct CurrencySettlement {
    currency:   Address,
    amount_net: I256
}

impl Encodable for CurrencySettlement {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.currency.encode(out);
        self.amount_net.into_raw().encode(out);
    }

    fn length(&self) -> usize {
        self.amount_net.into_raw().length()
    }
}

impl Decodable for CurrencySettlement {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        let currency = Address::decode(buf)?;
        let amount_net = U256::decode(buf)?;
        Ok(Self { currency, amount_net: I256::from_raw(amount_net) })
    }
}
