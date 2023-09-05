use ethers_core::{abi::AbiEncode, types::Address};
use futures::future::join_all;
use shared::{PoolKey, RawBundle, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement};
use sim::{errors::SimError, Simulator};

// struct SwapParams {
//     bool zeroForOne;
//     int256 amountSpecified;
//     uint160 sqrtPriceLimitX96;
// }
//
// struct TestSettings {
//         bool withdrawTokens;
//         bool settleUsingTransfer;
//     }
//  function swap(PoolKey memory key, IPoolManager.SwapParams memory
// params,TestSettings memory testSettings)
pub async fn match_cow<S: Simulator>(
    sim: S,
    data: Vec<(PoolKey, SimmedLvrSettlement, Vec<SimmedUserSettlement>)>
) -> Result<SimmedBundle, SimError> {
    let bundle = join_all(
        data.into_iter()
            .map(|(pool_key, best_searcher, users)| async move {
                let token_in = pool_key.currency_0;

                let mut token_in_vol = 0u128;
                let mut token_out_vol = 0u128;

                // add searcher vol
                if best_searcher.raw.order.token_in == token_in {
                    token_in_vol += best_searcher.raw.order.amount_in
                } else {
                    token_out_vol += best_searcher.raw.order.amount_in
                }

                // add user vol
                users.iter().for_each(|u| {
                    if u.raw.order.token_in == token_in {
                        token_in_vol += u.raw.order.amount_in;
                    } else {
                        token_out_vol += u.raw.order.amount_in;
                    }
                });

                let (zto, rem) = if token_in_vol > token_out_vol {
                    (true, token_in_vol - token_out_vol)
                } else {
                    (false, token_out_vol - token_in_vol)
                };

                let mut call_data = pool_key.encode();
            })
    )
    .await;

    todo!()
}
