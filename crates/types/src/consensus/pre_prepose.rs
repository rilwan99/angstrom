use bincode::{config::standard, encode_to_vec, Decode, Encode};
use reth_network_peers::PeerId;
use reth_primitives::keccak256;
use secp256k1::SecretKey;

use crate::{
    orders::OrderSet,
    primitive::Signature,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        sol::TopOfBlockOrder
    }
};

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RlpDecodable,
// RlpEncodable)] pub struct PoolOrders {
//     pub pool:         PoolId,
//     pub searcher_bid: TopOfBlockOrder,
//     pub orders:       Vec<OrderWithStorageData<GroupedVanillaOrder>>
// }

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct PreProposal {
    pub ethereum_height: u64,
    #[bincode(with_serde)]
    pub source:          PeerId,
    pub limit:           Vec<OrderWithStorageData<GroupedVanillaOrder>>,
    pub searcher:        Vec<OrderWithStorageData<TopOfBlockOrder>>,
    /// The signature is over the ethereum height as well as the limit and
    /// searcher sets
    pub signature:       Signature
}

impl PreProposal {
    pub fn generate_pre_proposal(
        ethereum_height: u64,
        source: PeerId,
        limit: Vec<OrderWithStorageData<GroupedVanillaOrder>>,
        searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>,
        sk: &SecretKey
    ) -> Self {
        let mut buf = Vec::new();
        let std = standard();
        buf.extend(encode_to_vec(ethereum_height, std).unwrap());
        buf.extend(encode_to_vec(&limit, std).unwrap());
        buf.extend(encode_to_vec(&searcher, std).unwrap());

        let hash = keccak256(buf);
        let sig = reth_primitives::sign_message(sk.secret_bytes().into(), hash).unwrap();

        Self { limit, source, searcher, ethereum_height, signature: Signature(sig) }
    }

    pub fn new(
        ethereum_height: u64,
        sk: &SecretKey,
        source: PeerId,
        orders: OrderSet<GroupedVanillaOrder, TopOfBlockOrder>
    ) -> Self {
        let OrderSet { limit, searcher } = orders;
        Self::generate_pre_proposal(ethereum_height, source, limit, searcher, sk)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::FixedBytes;

    use super::{PreProposal, SecretKey};

    #[test]
    fn can_be_constructed() {
        let ethereum_height = 100;
        let limit = vec![];
        let searcher = vec![];
        let source = FixedBytes::default();
        let sk = SecretKey::new(&mut rand::thread_rng());
        PreProposal::generate_pre_proposal(ethereum_height, source, limit, searcher, &sk);
    }
}
