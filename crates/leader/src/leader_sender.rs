use std::{
    ops::{Deref, DerefMut},
    task::{Context, Poll}
};

use ethers_core::types::transaction::{
    eip2718::TypedTransaction,
    eip712::{EIP712Domain, TypedData}
};
use ethers_flashbots::{BroadcasterMiddleware, PendingBundle};
use ethers_middleware::SignerMiddleware;
use ethers_providers::Middleware;
use ethers_signers::LocalWallet;
use shared::Bundle;
use sim::Simulator;

use crate::cow_solver::CowSolver;

type StakedWallet = LocalWallet;
type BundleKey = LocalWallet;

pub struct LeaderSender<M: Middleware>(
    pub &'static SignerMiddleware<BroadcasterMiddleware<M, BundleKey>, StakedWallet>
);

impl<M: Middleware> LeaderSender<M> {
    pub async fn submit_bundle(&self, tx: TypedTransaction, edsca: Address) -> anyhow::Result<()> {
        let block_number = self.0.get_block_number().await?;

        let tx = {
            self.0.fill_transaction(&mut tx, None).await?;
            tx
        };
        let signature = self.0.signer().sign_transaction(&tx, edsca).await?;
        let bundle = BundleRequest::new()
            .push_transaction(tx.rlp_signed(&signature))
            .set_block(block_number + 1)
            .set_simulation_block(block_number)
            .set_simulation_timestamp(0);

        let results: Vec<
            Result<
                PendingBundle<'_, <Self as Middleware>::Provider>,
                FlashbotsMiddlewareError<M, S>
            >
        > = self.0.inner().send_bundle(&bundle).await?;

        // You can also optionally wait to see if the bundle was included
        for result in results {
            match result {
                Ok(pending_bundle) => match pending_bundle.await {
                    Ok(bundle_hash) => {
                        println!("Bundle with hash {:?} was included in target block", bundle_hash)
                    }
                    Err(PendingBundleError::BundleNotIncluded) => {
                        println!("Bundle was not included in target block.")
                    }
                    Err(e) => println!("An error occured: {}", e)
                },
                Err(e) => println!("An error occured: {}", e)
            }
        }

        Ok(())
    }
}
