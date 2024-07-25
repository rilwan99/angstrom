use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    primitive::{BLSValidatorID, Lvr},
    sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData}
};
use blsful::SecretKey;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use reth_network_peers::PeerId;
use secp256k1::SecretKey as Secp256SecretKey;

use super::orders::DistributionParameters;
use crate::type_generator::orders::{generate_limit_order, generate_order_distribution};

pub fn generate_random_commit() -> Commit {
    let mut rng = thread_rng();
    let sk = SecretKey::new();

    Commit::generate_commit_all(rng.gen(), rng.gen(), rng.gen(), rng.gen(), &sk)
}

pub fn generate_limit_order_set(
    rng: &mut ThreadRng,
    count: usize,
    is_bid: bool,
    block: u64
) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
    (0..count)
        .map(|_| generate_limit_order(rng, true, is_bid, None, Some(block)))
        .collect()
}

pub fn generate_limit_order_distribution(
    count: usize,
    pool_id: usize,
    block: u64
) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
    let mut res = Vec::with_capacity(count * 2);
    let (bidprice, askprice) = DistributionParameters::crossed_at(100_000_000.0);
    let (bidquant, askquant) = DistributionParameters::fixed_at(100.0);
    res.extend(
        generate_order_distribution(true, count, bidprice, bidquant, pool_id, block).unwrap()
    );
    res.extend(
        generate_order_distribution(false, count, askprice, askquant, pool_id, block).unwrap()
    );
    res
}

pub fn generate_random_preposal(count: usize, block: u64) -> PreProposal {
    let mut rng = thread_rng();
    let sk = Secp256SecretKey::new(&mut rng);

    let limit = generate_limit_order_distribution(count, 10, block);

    PreProposal::generate_pre_proposal(block, PeerId::random(), limit, vec![], &sk)
}

pub fn generate_random_proposal(validator_id: BLSValidatorID) -> Proposal {
    let mut rng = thread_rng();
    let sk = SecretKey::new();

    // let mut order_buf = Vec::new();
    // for _ in 0..rng.gen_range(5..10) {
    //     order_buf.push(OrderBuffer {
    //         excess_orders:  (0..rng.gen_range(3..10))
    //             .map(|_| generate_rand_valid_limit_order())
    //             .collect(),
    //         reserve_orders: (0..rng.gen_range(3..10))
    //             .map(|_| generate_rand_valid_limit_order())
    //             .collect()
    //     })
    // }

    Proposal::generate_proposal(rng.gen(), PeerId::default(), vec![], vec![], validator_id, &sk)
}
