use std::collections::HashMap;

use ethers_core::{
    abi::AbiEncode,
    types::{transaction::eip2718::TypedTransaction, Address}
};
use futures::future::join_all;
use guard_types::on_chain::{
    PoolKey, RawBundle, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement
};
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
    best_searcher_tx: HashMap<PoolKey, SimmedLvrSettlement> /* bytes_to_pool_key: HashMap<[u8;
                                                             * 32], PoolKey> */
) -> Result<SimmedBundle, SimError> {
    // yoink all the volume for these transactions
    let mut volumes = HashMap::new();
    for (key, tx) in best_searcher_tx {
        if tx.raw.order.token_in == key.currency_0 {
            volumes
                .insert(key, (tx.raw.order.amount_in as i128, tx.raw.order.amount_out_min as i128));
        } else {
            volumes
                .insert(key, (tx.raw.order.amount_out_min as i128, tx.raw.order.amount_in as i128));
        };
    }

    for (key, tx) in all_direct_users {
        if tx.raw.order.token_in == key.currency_0 {
            match volumes.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().0 += tx.raw.order.amount_in as i128;
                    o.get_mut().1 -= tx.raw.order.amount_out_min as i128;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((tx.raw.order.amount_in as i128, tx.raw.order.amount_out_min as i128));
                }
            }
        } else {
            match volumes.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().1 += tx.raw.order.amount_in as i128;
                    o.get_mut().0 -= tx.raw.order.amount_out_min as i128;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((tx.raw.order.amount_out_min as i128, tx.raw.order.amount_in as i128));
                }
            }
        }
    }

    for (key, (zto_vol, otz_vol)) in volumes {
        let limit_pice = zto_vol / otz_vol;
    }

    todo!()
}

// struct SwapParams {
//         bool zeroForOne;
//         int256 amountSpecified;
//         uint160 sqrtPriceLimitX96;
//     }
// struct TestSettings {
//     bool withdrawTokens;
//     bool settleUsingTransfer;
// }
pub fn build_v4_transaction(pool_key: PoolKey, amount: i128, zto: bool) -> TypedTransaction {
    todo!()
}
