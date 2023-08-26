/// struct Batch {
///     ArbitrageOrderSigned[] arbs;
///     PoolSettlement[] pools;
///     UserSettlement[] users;
/// }
pub struct Bundle {
    arbs: Vec<ArbitrageOrderSigned>,
    pools: Vec<PoolSettlement>,
    users: Vec<UserSettlement>

}

}
/// struct ArbitrageOrderSigned {
///     ArbitrageOrder order;
///    bytes signature;
/// }
pub struct ArbitrageOrderSigned {
 signature: Bytes,
 order: ArbitrageOrder
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
    pool: [u8; 32],
    token_in: Address,
    token_out: Address,
    amount_in: u128,
    amount_out: u128,
    amount_out_min: u128,
    deadline: U256,
    gas_bid: U256,
    bride: U256,
    pre_hook: Bytes,
    post_hock: Bytes
}

/// struct PoolSettlement {
///     PoolKey pool;
///     uint256 token0In;
///     uint256 token1In;
/// }
pub struct PoolSettlement {
    pool: PoolKey,
    token_0_in: U256,
    token_1_in: U256,
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
    order: UserOrder,
    signature: Bytes,
    amount_out: U245,
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
    token_in: Address,
    token_out: Address,
    amount_in: u128,
    amount_out_min: u128,
    deadline: U256,
    gas_bid: U256,
    pre_hook: Bytes,
    post_hook: Bytes,
}

/// struct PoolKey {
///     Currency currency0;
///     Currency currency1;
///     uint24 fee;
///     int24 tickSpacing;
///     address hooks;
/// }
pub struct PoolKey {
    currency_0: Address,
    currency_1: Address,
    fee: u32,
    tick_spacing: u32,
    hooks: Address,
}
