use ethers_core::types::{Address, I256, U256};
use reth_primitives::{bytes, Bytes};
use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};
mod eip712;
mod signature;
mod tee_address;
pub use eip712::*;
use ethers_core::types::H256;
use reth_rlp::{Decodable, DecodeError, Encodable};
use secp256k1::Secp256k1;
pub use signature::*;
pub use tee_address::*;

/// struct Batch {
///     // UniV4 swaps to execute.
///     PoolSwap[] swaps;
///     // Settlement flows.
///     LvrSettlement[] lvr;
///     UserSettlement[] users;
///     CurrencySettlement[] currencies;
///     PoolFees[] pools;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct Batch {
    // Univ4 swaps
    pub swaps:      Vec<PoolSwap>,
    pub lvr:        Vec<LvrSettlement>,
    pub users:      Vec<UserSettlement>,
    pub currencies: Vec<CurrencySettlement>,
    pub pools:      Vec<PoolFees>
}

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
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct SearcherOrder {
    /// TODO: move to wrapped fix size for quicker encoding
    pub pool:           [u8; 32],
    pub token_in:       Address,
    pub token_out:      Address,
    pub amount_in:      u128,
    pub amount_out:     u128,
    pub amount_out_min: u128,
    pub deadline:       U256,
    pub gas_cap:        U256,
    pub bribe:          U256,
    pub pre_hook:       Bytes,
    pub post_hock:      Bytes
}

/// struct PoolSettlement {
///     PoolKey pool;
///     uint256 token0In;
///     uint256 token1In;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct PoolSettlement {
    pub pool:       PoolKey,
    pub token_0_in: U256,
    pub token_1_in: U256
}

/// struct UserSettlement {
///     // User provided.
///     UserOrder order;
///     bytes signature;
///     // Guard provided.
///     uint256 amountOut;
///     uint256 gasActual;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct UserSettlement {
    pub order:      UserOrder,
    pub signature:  Bytes,
    // guard provider
    pub amount_out: U256,
    pub gas_actual: U256
}

/// struct LvrSettlement {
///     // User provided.
///     SearcherOrder order;
///     bytes signature;
///     // Guard provided.
///     uint256 gasActual;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct LvrSettlement {
    order:      SearcherOrder,
    signature:  Bytes,
    // guard provided
    gas_actual: U256
}
/// struct PoolSwap {
///     PoolKey pool;
///     Currency currencyIn;
///     uint256 amountIn;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct PoolSwap {
    pool:        PoolKey,
    currency_in: Address,
    amount_in:   U256
}

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
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
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

/// struct PoolKey {
///     Currency currency0;
///     Currency currency1;
///     uint24 fee;
///     int24 tickSpacing;
///     address hooks;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct PoolKey {
    pub currency_0:   Address,
    pub currency_1:   Address,
    pub fee:          u32,
    pub tick_spacing: u32,
    pub hooks:        Address
}

/// struct PoolFees {
///     PoolKey pool;
///     uint256 fees0;
///     uint256 fees1;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct PoolFees {
    pool:   PoolKey,
    fees_0: U256,
    fees_1: U256
}

/// struct CurrencySettlement {
///     Currency currency;
///     int256 amountNet;
/// }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
