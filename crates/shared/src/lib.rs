use ethers_core::types::{Address, Bytes, U256};
/// struct Batch {
///     ArbitrageOrderSigned[] arbs;
///     PoolSettlement[] pools;
///     UserSettlement[] users;
/// }
pub struct Bundle {
    pub arbs:  Vec<ArbitrageOrderSigned>,
    pub pools: Vec<PoolSettlement>,
    pub users: Vec<UserSettlement>
}

pub struct SealedOrder(pub [u8; 32]);

pub struct SealedBundle {
    pub arbs:  Vec<SealedOrder>,

    pub pools: Vec<PoolSettlement>,
    pub users: Vec<UserSettlement>
}

impl SealedBundle {
    pub fn gas_bid_sum(&self) -> U256 {
        self.users.iter().map(|user| user.order.gas_bid).sum()
    }
}


/// struct ArbitrageOrderSigned {
///     ArbitrageOrder order;
///    bytes signature;
/// }
pub struct ArbitrageOrderSigned {
    pub signature: Bytes,
    pub order:     ArbitrageOrder
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
pub struct ArbitrageOrder {
    /// TODO: move to wrapped fix size for quicker encoding
    pub pool:           [u8; 32],
    pub token_in:       Address,
    pub token_out:      Address,
    pub amount_in:      u128,
    pub amount_out:     u128,
    pub amount_out_min: u128,
    pub deadline:       U256,
    pub gas_bid:        U256,
    pub bride:          U256,
    pub pre_hook:       Bytes,
    pub post_hock:      Bytes
}

/// struct PoolSettlement {
///     PoolKey pool;
///     uint256 token0In;
///     uint256 token1In;
/// }
pub struct PoolSettlement {
    pub pool:       PoolKey,
    pub token_0_in: U256,
    pub token_1_in: U256
}

/// struct UserSettlement {
///     // User provided.
///     UserOrder order;
///     bytes signature;
///
///     // Guard provided.
///     uint256 amountOut;
/// }
pub struct UserSettlement {
    pub order:      UserOrder,
    pub signature:  Bytes,
    pub amount_out: U256
}

/// struct UserOrder {
///     Currency tokenIn;
///     Currency tokenOut;
///     uint128 amountIn;
///     uint128 amountOutMin;
///     uint256 deadline;
///     uint256 gasBid;
///     bytes preHook;
///     bytes postHook;
/// }
pub struct UserOrder {
    pub token_out:      Address,
    pub token_in:       Address,
    pub amount_in:      u128,
    pub amount_out_min: u128,
    pub deadline:       U256,
    pub gas_bid:        U256,
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
pub struct PoolKey {
    pub currency_0:   Address,
    pub currency_1:   Address,
    pub fee:          u32,
    pub tick_spacing: u32,
    pub hooks:        Address
}
