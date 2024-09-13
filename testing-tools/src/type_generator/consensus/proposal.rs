use std::collections::HashMap;

use alloy_primitives::FixedBytes;
use angstrom_types::{
    consensus::Proposal,
    primitive::PoolId,
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};
use matching_engine::{
    strategy::{MatchingStrategy, SimpleCheckpointStrategy},
    MatchingManager
};
use rand::thread_rng;
use reth_network_peers::pk2id;
use secp256k1::{Secp256k1, SecretKey as Secp256SecretKey};

use super::preproposal::PreproposalBuilder;

#[derive(Debug, Default)]
pub struct ProposalBuilder {
    ethereum_height: Option<u64>,
    order_count:     Option<usize>,
    block:           Option<u64>,
    pools:           Option<Vec<PoolId>>,
    sk:              Option<Secp256SecretKey>
}

impl ProposalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn order_count(self, order_count: usize) -> Self {
        Self { order_count: Some(order_count), ..self }
    }

    pub fn for_block(self, block: u64) -> Self {
        Self { block: Some(block), ..self }
    }

    pub fn for_pools(self, pools: Vec<PoolId>) -> Self {
        Self { pools: Some(pools), ..self }
    }

    pub fn for_random_pools(self, pool_count: usize) -> Self {
        let pools: Vec<PoolId> = (0..pool_count).map(|_| FixedBytes::random()).collect();
        Self { pools: Some(pools), ..self }
    }

    pub fn with_secret_key(self, sk: Secp256SecretKey) -> Self {
        Self { sk: Some(sk), ..self }
    }

    pub fn build(self) -> Proposal {
        // Extract values from our struct
        let ethereum_height = self.ethereum_height.unwrap_or_default();
        let pools: Vec<FixedBytes<32>> = self.pools.unwrap_or_default();
        let count = self.order_count.unwrap_or_default();
        let block = self.block.unwrap_or_default();
        let sk = self
            .sk
            .unwrap_or_else(|| Secp256SecretKey::new(&mut thread_rng()));
        // Build the source ID from the secret/public keypair
        let source = pk2id(&sk.public_key(&Secp256k1::new()));

        let preproposals = (0..5)
            .map(|_| {
                PreproposalBuilder::new()
                    .for_block(block)
                    .order_count(count)
                    .for_pools(pools.clone())
                    .build()
            })
            .collect::<Vec<_>>();
        let books = MatchingManager::build_books(&preproposals);
        let searcher_orders: HashMap<PoolId, OrderWithStorageData<TopOfBlockOrder>> = preproposals
            .iter()
            .flat_map(|p| p.searcher.iter())
            .fold(HashMap::new(), |mut acc, order| {
                acc.entry(order.pool_id).or_insert(order.clone());
                acc
            });
        let solutions = books
            .into_iter()
            .map(|b| {
                let searcher = searcher_orders.get(&b.id()).cloned();
                SimpleCheckpointStrategy::run(&b)
                    .map(|s| s.solution(searcher))
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Proposal::generate_proposal(ethereum_height, source, preproposals, solutions, &sk)
    }
}
