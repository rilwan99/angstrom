use alloy::{
    dyn_abi::DynSolType,
    network::Network,
    providers::Provider,
    sol,
    transports::Transport,
};
use amms::amm::uniswap_v3::UniswapV3Pool;
use amms::errors::AMMError;
use std::sync::Arc;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV3TickDataBatchRequest,
    "src/cfmm/uniswap/GetUniswapV3TickDataBatchRequestABI.json"
}

pub struct UniswapV3TickData {
    pub initialized: bool,
    pub tick: i32,
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
}

// Mostly copied from https://github.com/darkforestry/amms-rs/blob/3045906b2e5590b6d8f5b41e8db41ee2a5b85124/src/amm/uniswap_v3/batch_request/mod.rs#L109
pub async fn get_uniswap_v3_tick_data_batch_request<T, N, P>(
    pool: &UniswapV3Pool,
    tick_start: i32,
    zero_for_one: bool,
    num_ticks: u16,
    block_number: Option<u64>,
    provider: Arc<P>,
) -> Result<(Vec<UniswapV3TickData>, u64), AMMError>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    let deployer = IGetUniswapV3TickDataBatchRequest::deploy_builder(
        provider,
        pool.address,
        zero_for_one,
        tick_start,
        num_ticks,
        pool.tick_spacing,
    );
    let res = if let Some(block_number) = block_number {
        deployer.block(block_number.into()).call_raw().await?
    } else {
        deployer.call_raw().await?
    };

    let constructor_return = DynSolType::Tuple(vec![
        DynSolType::Array(Box::new(DynSolType::Tuple(vec![
            DynSolType::Bool,
            DynSolType::Int(24),
            DynSolType::Uint(128),
            DynSolType::Int(128),
        ]))),
        DynSolType::Uint(32),
    ]);
    let return_data_tokens = constructor_return.abi_decode_sequence(&res)?;

    let return_data_tuple = return_data_tokens
        .as_tuple()
        .ok_or(AMMError::BatchRequestError(pool.address))?;

    let tick_data_arr = return_data_tuple[0]
        .as_array()
        .ok_or(AMMError::BatchRequestError(pool.address))?;

    let mut tick_data = vec![];
    for tokens in tick_data_arr {
        if let Some(tick_data_tuple) = tokens.as_tuple() {
            let initialized = tick_data_tuple[0]
                .as_bool()
                .ok_or(AMMError::BatchRequestError(pool.address))?;

            let tick = tick_data_tuple[1]
                .as_int()
                .ok_or(AMMError::BatchRequestError(pool.address))?
                .0
                .as_i32();

            let liquidity_gross = tick_data_tuple[2]
                .as_uint()
                .ok_or(AMMError::BatchRequestError(pool.address))?
                .0
                .try_into()
                .map_err(|e| AMMError::EyreError(eyre::eyre!("{e}")))?;

            let liquidity_net = tick_data_tuple[3]
                .as_int()
                .ok_or(AMMError::BatchRequestError(pool.address))?
                .0
                .try_into()
                .map_err(|e| AMMError::EyreError(eyre::eyre!("{e}")))?;

            tick_data.push(UniswapV3TickData { initialized, tick, liquidity_gross, liquidity_net });
        }
    }

    let block_number = return_data_tuple[1]
        .as_uint()
        .ok_or(AMMError::BatchRequestError(pool.address))?
        .0
        .to::<u64>();

    Ok((tick_data, block_number))
}
