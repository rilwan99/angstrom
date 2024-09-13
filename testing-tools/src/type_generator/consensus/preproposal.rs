use alloy_primitives::FixedBytes;
use angstrom_types::{consensus::PreProposal, primitive::PoolId};
use rand::thread_rng;
use reth_network_peers::pk2id;
use secp256k1::{Secp256k1, SecretKey as Secp256SecretKey};

use super::generate_limit_order_distribution;
use crate::type_generator::orders::generate_top_of_block_order;

#[derive(Default)]
pub struct PreproposalBuilder {
    order_count: Option<usize>,
    block:       Option<u64>,
    pools:       Option<Vec<PoolId>>,
    sk:          Option<Secp256SecretKey>
}

impl PreproposalBuilder {
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

    pub fn build(self) -> PreProposal {
        // Extract values from our struct
        let pools: Vec<FixedBytes<32>> = self.pools.unwrap_or_default();
        let count = self.order_count.unwrap_or_default();
        let block = self.block.unwrap_or_default();
        let sk = self
            .sk
            .unwrap_or_else(|| Secp256SecretKey::new(&mut thread_rng()));
        // Build the source ID from the secret/public keypair
        let source = pk2id(&sk.public_key(&Secp256k1::new()));

        let limit = pools
            .iter()
            .flat_map(|pool_id| generate_limit_order_distribution(count, *pool_id, block))
            .collect();

        let searcher = pools
            .iter()
            .map(|pool_id| {
                generate_top_of_block_order(
                    &mut thread_rng(),
                    true,
                    Some(*pool_id),
                    Some(block),
                    None,
                    None
                )
            })
            .collect();

        PreProposal::generate_pre_proposal(block, source, limit, searcher, &sk)
    }
}
