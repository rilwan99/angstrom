use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    primitive::{BLSValidatorID, Lvr}
};
use blsful::SecretKey;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use reth_network_peers::PeerId;
use secp256k1::SecretKey as Secp256SecretKey;

pub fn generate_random_commit() -> Commit {
    let mut rng = thread_rng();
    let sk = SecretKey::new();

    Commit::generate_commit_all(rng.gen(), rng.gen(), rng.gen(), rng.gen(), &sk)
}

pub fn generate_random_preposal() -> PreProposal {
    let mut rng = thread_rng();
    let sk = Secp256SecretKey::new(&mut rng);

    // let mut pre_bundle = Vec::new();
    // for _ in 0..rng.gen_range(3..10) {
    // pre_bundle.push(PoolOrders {
    //     pool:         rng.gen(),
    //     orders:       vec![],
    //     // (0..rng.gen_range(3..10))
    //     //     .map(|_| generate_rand_valid_limit_order())
    //     //     .collect(),
    //     // searcher_bid: generate_rand_valid_searcher_order()
    //     searcher_bid: TopOfBlockOrder::default()
    // })
    // }

    PreProposal::generate_pre_proposal(rng.gen(), PeerId::default(), vec![], vec![], &sk)
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

fn gen_lvr(rng: &mut ThreadRng) -> Lvr {
    Lvr { postArbPrice: rng.gen(), lvrComp: rng.gen(), proportion: rng.gen() }
}
