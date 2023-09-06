use std::collections::HashMap;

use ethers_core::{
    abi::AbiEncode,
    types::{transaction::eip2718::TypedTransaction, Address}
};
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
    all_direct_users: HashMap<PoolKey, SimmedUserSettlement>,
    all_undirect_users: Vec<SimmedUserSettlement>,
    best_searcher_tx: HashMap<PoolKey, SimmedLvrSettlement>,
    bytes_to_pool_key: HashMap<[u8; 32], PoolKey>
) -> Result<SimmedBundle, SimError> {
    // first thing we need todo is sum up all the transactions that is a direct
    // pool. this is because then we can optimize for the multi-hop routes
    let mut volumes = HashMap::new();
    // let mut searcher_lim = HashMap::new();
    for (key, tx) in best_searcher_tx {
        if tx.raw.order.token_in == key.currency_0 {
            volumes.insert(key, (tx.raw.order.amount_in, 0u128));
        } else {
            volumes.insert(key, (0u128, tx.raw.order.amount_in));
        };
    }

    for (key, tx) in all_direct_users {
        if tx.raw.order.token_in == key.currency_0 {
            match volumes.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().0 += tx.raw.order.amount_in;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((tx.raw.order.amount_in, 0u128));
                }
            }
        } else {
            match volumes.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().1 += tx.raw.order.amount_in;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((0u128, tx.raw.order.amount_in));
                }
            }
        }
    }

    // account for all of the direct pool swaps
    // let unmatched = all_users.into_iter().filter_map(|tx| {
    //     let token_in = tx.raw.order.token_in;
    //     let token_out = tx.raw.order.token_out;
    //
    //     if let Some((key, _)) = best_searcher_tx.iter_mut().find(|(k, _)| {
    //         k.currency_0 == token_in
    //             || k.currency_0 == token_out && k.currency_1 == token_in
    //             || k.currency_1 == token_out
    //     }) {
    //     } else {
    //     };
    // });

    // let bundle = join_all(
    //     data.into_iter()
    //         .map(|(pool_key, best_searcher, users)| async move {
    //             let token_in = pool_key.currency_0;
    //
    //             let mut token_in_vol = 0u128;
    //             let mut token_out_vol = 0u128;
    //
    //             // add searcher vol
    //             if best_searcher.raw.order.token_in == token_in {
    //                 token_in_vol += best_searcher.raw.order.amount_in
    //             } else {
    //                 token_out_vol += best_searcher.raw.order.amount_in
    //             }
    //
    //             // add user vol
    //             users.iter().for_each(|u| {
    //                 if u.raw.order.token_in == token_in {
    //                     token_in_vol += u.raw.order.amount_in;
    //                 } else {
    //                     token_out_vol += u.raw.order.amount_in;
    //                 }
    //             });
    //
    //             let (zto, rem) = if token_in_vol > token_out_vol {
    //                 (true, token_in_vol - token_out_vol)
    //             } else {
    //                 (false, token_out_vol - token_in_vol)
    //             };
    //
    //             let mut call_data = pool_key.encode();
    //         })
    // )
    // .await;

    todo!()
}

pub fn build_v4_transaction() -> TypedTransaction {
    todo!()
}
