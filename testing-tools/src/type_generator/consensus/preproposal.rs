use angstrom_types::consensus::PreProposal;
use rand::thread_rng;
use reth_network_peers::pk2id;
use secp256k1::{Secp256k1, SecretKey as Secp256SecretKey};

use super::pool::{Pool, PoolBuilder};
use crate::type_generator::orders::{
    generate_top_of_block_order, DistributionParameters, OrderDistributionBuilder
};

#[derive(Debug, Default)]
pub struct PreproposalBuilder {
    order_count: Option<usize>,
    block:       Option<u64>,
    pools:       Option<Vec<Pool>>,
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

    pub fn for_pools(self, pools: Vec<Pool>) -> Self {
        Self { pools: Some(pools), ..self }
    }

    pub fn for_random_pools(self, pool_count: usize) -> Self {
        let pools: Vec<Pool> = (0..pool_count)
            .map(|_| PoolBuilder::new().build())
            .collect();
        Self { pools: Some(pools), ..self }
    }

    pub fn with_secret_key(self, sk: Secp256SecretKey) -> Self {
        Self { sk: Some(sk), ..self }
    }

    pub fn build(self) -> PreProposal {
        // Extract values from our struct
        let pools = self.pools.unwrap_or_default();
        // let pools: Vec<PoolId> = self
        //     .pools
        //     .map(|p| p.iter().map(|key| key.id()).collect())
        //     .unwrap_or_default();
        let count = self.order_count.unwrap_or_default();
        let block = self.block.unwrap_or_default();
        let sk = self
            .sk
            .unwrap_or_else(|| Secp256SecretKey::new(&mut thread_rng()));
        // Build the source ID from the secret/public keypair
        let source = pk2id(&sk.public_key(&Secp256k1::new()));

        let limit = pools
            .iter()
            .flat_map(|pool| {
                let (bid_dist, ask_dist) =
                    DistributionParameters::crossed_at(pool.price().as_float());
                let (bid_quant, ask_quant) = DistributionParameters::fixed_at(100.0);
                let bids = OrderDistributionBuilder::new()
                    .bid()
                    .order_count(count)
                    .pool_id(pool.id())
                    .valid_block(block)
                    .price_params(bid_dist)
                    .volume_params(bid_quant)
                    .build()
                    .unwrap();
                let asks = OrderDistributionBuilder::new()
                    .ask()
                    .order_count(count)
                    .pool_id(pool.id())
                    .valid_block(block)
                    .price_params(ask_dist)
                    .volume_params(ask_quant)
                    .build()
                    .unwrap();
                [bids, asks].concat()
            })
            .collect();

        let searcher = pools
            .iter()
            .map(|pool_id| {
                generate_top_of_block_order(
                    &mut thread_rng(),
                    true,
                    Some(pool_id.id()),
                    Some(block),
                    None,
                    None
                )
            })
            .collect();

        PreProposal::generate_pre_proposal(block, source, limit, searcher, &sk)
    }
}

#[cfg(test)]
mod tests {
    use super::PreproposalBuilder;

    #[test]
    fn generates_order_spread_that_crosses() {
        // It is MAYBE statistically possible that this will fail due to probability one
        // day?
        let pre_proposal = PreproposalBuilder::new()
            .order_count(100)
            .for_random_pools(1)
            .build();
        let (high_price, low_price) =
            pre_proposal
                .limit
                .iter()
                .fold((f64::MIN, f64::MAX), |mut acc, order| {
                    let price = order.float_price();
                    if order.is_bid && price > acc.0 {
                        acc.0 = price;
                    }
                    if !order.is_bid && price < acc.1 {
                        acc.1 = price;
                    }
                    acc
                });
        assert!(high_price > low_price, "Prices do not cross");
    }
}
