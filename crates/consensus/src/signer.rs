use alloy_primitives::{FixedBytes, B512};
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::PoolSolution,
    primitive::BLSValidatorID
};
use blsful::{Bls12381G1Impl, SecretKey as BlsSecretKey};
use reth_rpc_types::PeerId;
use secp256k1::SecretKey;

/// The Signer deals with verifying external signatures as well as
/// signing our payloads.  Pub fields for now.
#[derive(Clone)]
pub struct Signer {
    pub my_id:        PeerId,
    pub validator_id: BLSValidatorID,
    pub key:          SecretKey,
    pub bls_key:      BlsSecretKey<Bls12381G1Impl>
}

impl Default for Signer {
    fn default() -> Self {
        let key = SecretKey::new(&mut secp256k1::rand::thread_rng());
        Signer {
            my_id: FixedBytes::random(),
            validator_id: BLSValidatorID::default(),
            key,
            bls_key: BlsSecretKey::default()
        }
    }
}

impl Signer {
    #[allow(dead_code)]
    pub fn sign_proposal(
        &self,
        ethereum_block: u64,
        preproposals: Vec<PreProposal>,
        solutions: Vec<PoolSolution>
    ) -> eyre::Result<Proposal> {
        Ok(Proposal::generate_proposal(
            ethereum_block,
            self.my_id,
            preproposals,
            solutions,
            &self.key
        ))
    }

    #[allow(dead_code)]
    pub fn sign_commit(&self, _ethereum_block: u64, proposal: &Proposal) -> eyre::Result<Commit> {
        Ok(Commit::from_proposal(proposal, &self.bls_key))
    }

    // #[allow(dead_code)]
    // pub fn sign_pre_propose(
    //     &self,
    //     _ethereum_block: u64,
    //     _orders: Vec<PoolOrders>
    // ) -> eyre::Result<PreProposal> {
    //     todo!()
    // }

    pub fn is_us(&self, _addr: &B512) -> bool {
        todo!("change key to proper")
    }
}
