use std::{
    collections::{BQueueMap, HashSet},
    ops::RangeBounds,
    sync::Arc,
    time::Instant
};

use reth_db::{database::Database, models::StoredBlockBodyIndices};
use reth_interfaces::{
    blockchain_tree::{BlockchainQueueEngine, BlockchainQueueViewer},
    consensus::ForkchoiceState,
    Error, Result
};
use reth_primitives::{
    stage::{StageCheckpoint, StageId},
    Address, Block, BlockHash, BlockHashOrNumber, BlockId, BlockNumHash, BlockNumber,
    BlockNumberOrTag, BlockWithSenders, ChainInfo, ChainSpec, Header, PruneCheckpoint, PrunePart,
    Receipt, SealedBlock, SealedBlockWithSenders, SealedHeader, TransactionMeta, TransactionSigned,
    TransactionSignedNoHash, TxHash, TxNumber, Withdrawal, H256, U256
};
use reth_revm_primitives::primitives::{BlockEnv, CfgEnv};
pub use state::{
    historical::{HistoricalStateProvider, HistoricalStateProviderRef},
    latest::{LatestStateProvider, LatestStateProviderRef}
};
use tracing::trace;

use crate::{
    BlockHashReader, BlockIdReader, BlockNumReader, BlockReader, BlockReaderIdExt,
    BlockchainTreePendingStateProvider, CanonChainTracker, CanonStateNotifications,
    CanonStateSubscriptions, ChainSpecProvider, ChangeSetReader, EvmEnvProvider, HeaderProvider,
    PostStateDataProvider, ProviderError, PruneCheckpointReader, ReceiptProvider,
    ReceiptProviderIdExt, StageCheckpointReader, StateProviderBox, StateProviderFactory,
    TransactionsProvider, WithdrawalsProvider
};

mod chain_info;
mod database;
mod post_state_provider;
mod state;
pub use database::*;
pub use post_state_provider::PostStateProvider;
use reth_db::models::AccountBeforeTx;
use reth_interfaces::blockchain_tree::{
    error::InsertBlockError, CanonicalOutcome, InsertPayloadOk
};

use crate::{providers::chain_info::ChainInfoTracker, traits::BlockSource};

/// The main type for interacting with the blockchain.
///
/// This type serves as the main entry point for interacting with the blockchain
/// and provides data from database storage and from the blockchain tree
/// (pending state etc.) It is a simple wrapper type that holds an instance of
/// the database and the blockchain tree.
#[derive(Clone)]
pub struct BlockchainProvider<DB, Queue> {
    /// Provider type used to access the database.
    database:   ProviderFactory<DB>,
    /// The blockchain queue instance.
    queue:      Queue,
    /// Tracks the chain info wrt forkchoice updates
    chain_info: ChainInfoTracker
}

impl<DB, Queue> BlockchainProvider<DB, Queue> {
    /// Create new  provider instance that wraps the database and the blockchain
    /// tree, using the provided latest header to initialize the chain info
    /// tracker.
    pub fn with_latest(database: ProviderFactory<DB>, queue: Queue, latest: SealedHeader) -> Self {
        Self { database, queue, chain_info: ChainInfoTracker::new(latest) }
    }
}

impl<DB, Queue> BlockchainProvider<DB, Queue>
where
    DB: Database
{
    /// Create a new provider using only the database and the tree, fetching the
    /// latest header from the database to initialize the provider.
    pub fn new(database: ProviderFactory<DB>, queue: Queue) -> Result<Self> {
        let provider = database.provider()?;
        let best: ChainInfo = provider.chain_info()?;
        match provider.header_by_number(best.best_number)? {
            Some(header) => {
                drop(provider);
                Ok(Self::with_latest(database, queue, header.seal(best.best_hash)))
            }
            None => Err(Error::Provider(ProviderError::HeaderNotFound(best.best_number.into())))
        }
    }
}

impl<DB, Queue> BlockchainProvider<DB, Queue>
where
    DB: Database,
    Queue: BlockchainQueueViewer
{
    /// Ensures that the given block number is canonical (synced)
    ///
    /// This is a helper for guarding the [HistoricalStateProvider] against
    /// block numbers that are out of range and would lead to invalid
    /// results, mainly during initial sync.
    ///
    /// Verifying the block_number would be expensive since we need to lookup
    /// sync table Instead, we ensure that the `block_number` is within the
    /// range of the [Self::best_block_number] which is updated when a block
    /// is synced.
    #[inline]
    fn ensure_canonical_block(&self, block_number: BlockNumber) -> Result<()> {
        let latest = self.best_block_number()?;
        if block_number > latest {
            Err(ProviderError::HeaderNotFound(block_number.into()).into())
        } else {
            Ok(())
        }
    }
}

impl<DB, Queue> BlockchainQueueEngine for BlockchainProvider<DB, Tree>
where
    DB: Send + Sync,
    Queue: BlockchainQueueEngine
{
    fn buffer_block(
        &self,
        block: SealedBlockWithSenders
    ) -> std::result::Result<(), InsertBlockError> {
        self.queue.buffer_block(block)
    }

    fn insert_block(
        &self,
        block: SealedBlockWithSenders
    ) -> std::result::Result<InsertPayloadOk, InsertBlockError> {
        self.queue.insert_block(block)
    }

    fn finalize_block(&self, finalized_block: BlockNumber) {
        self.queue.finalize_block(finalized_block)
    }

    fn restore_canonical_hashes_and_finalize(
        &self,
        last_finalized_block: BlockNumber
    ) -> Result<()> {
        self.queue
            .restore_canonical_hashes_and_finalize(last_finalized_block)
    }

    fn restore_canonical_hashes(&self) -> Result<()> {
        self.queue.restore_canonical_hashes()
    }

    fn make_canonical(&self, block_hash: &BlockHash) -> Result<CanonicalOutcome> {
        self.queue.make_canonical(block_hash)
    }

    fn unwind(&self, unwind_to: BlockNumber) -> Result<()> {
        self.queue.unwind(unwind_to)
    }
}

impl<DB, Queue> BlockchainTreeViewer for BlockchainProvider<DB, Tree>
where
    DB: Send + Sync,
    Queue: BlockchainQueueViewer
{
    fn blocks(&self) -> BQueueMap<BlockNumber, HashSet<BlockHash>> {
        self.queue.blocks()
    }

    fn header_by_hash(&self, hash: BlockHash) -> Option<SealedHeader> {
        self.queue.header_by_hash(hash)
    }

    fn block_by_hash(&self, block_hash: BlockHash) -> Option<SealedBlock> {
        self.queue.block_by_hash(block_hash)
    }

    fn buffered_block_by_hash(&self, block_hash: BlockHash) -> Option<SealedBlock> {
        self.queue.buffered_block_by_hash(block_hash)
    }

    fn buffered_header_by_hash(&self, block_hash: BlockHash) -> Option<SealedHeader> {
        self.queue.buffered_header_by_hash(block_hash)
    }

    fn canonical_blocks(&self) -> BQueueMap<BlockNumber, BlockHash> {
        self.queue.canonical_blocks()
    }

    fn find_canonical_ancestor(&self, hash: BlockHash) -> Option<BlockHash> {
        self.queue.find_canonical_ancestor(hash)
    }

    fn is_canonical(&self, hash: BlockHash) -> std::result::Result<bool, Error> {
        self.queue.is_canonical(hash)
    }

    fn lowest_buffered_ancestor(&self, hash: BlockHash) -> Option<SealedBlockWithSenders> {
        self.queue.lowest_buffered_ancestor(hash)
    }

    fn canonical_tip(&self) -> BlockNumHash {
        self.queue.canonical_tip()
    }

    fn pending_blocks(&self) -> (BlockNumber, Vec<BlockHash>) {
        self.queue.pending_blocks()
    }

    fn pending_block_num_hash(&self) -> Option<BlockNumHash> {
        self.queue.pending_block_num_hash()
    }

    fn pending_block_and_receipts(&self) -> Option<(SealedBlock, Vec<Receipt>)> {
        self.queue.pending_block_and_receipts()
    }

    fn receipts_by_block_hash(&self, block_hash: BlockHash) -> Option<Vec<Receipt>> {
        self.queue.receipts_by_block_hash(block_hash)
    }
}

impl<DB, Queue> CanonChainTracker for BlockchainProvider<DB, Queue>
where
    DB: Send + Sync,
    Queue: Send + Sync,
    Self: BlockReader
{
    fn on_forkchoice_update_received(&self, _update: &ForkchoiceState) {
        // update timestamp
        self.chain_info.on_forkchoice_update_received();
    }

    fn last_received_update_timestamp(&self) -> Option<Instant> {
        self.chain_info.last_forkchoice_update_received_at()
    }

    fn on_transition_configuration_exchanged(&self) {
        self.chain_info.on_transition_configuration_exchanged();
    }

    fn last_exchanged_transition_configuration_timestamp(&self) -> Option<Instant> {
        self.chain_info.last_transition_configuration_exchanged_at()
    }

    fn set_canonical_head(&self, header: SealedHeader) {
        self.chain_info.set_canonical_head(header);
    }

    fn set_safe(&self, header: SealedHeader) {
        self.chain_info.set_safe(header);
    }

    fn set_finalized(&self, header: SealedHeader) {
        self.chain_info.set_finalized(header);
    }
}

impl<DB, Queue> BlockchainQueuePendingStateProvider for BlockchainProvider<DB, Tree>
where
    DB: Send + Sync,
    Queue: BlockchainQueuePendingStateProvider
{
    fn find_pending_state_provider(
        &self,
        block_hash: BlockHash
    ) -> Option<Box<dyn PostStateDataProvider>> {
        self.queue.find_pending_state_provider(block_hash)
    }
}

impl<DB, Queue> CanonStateSubscriptions for BlockchainProvider<DB, Queue>
where
    DB: Send + Sync,
    Queue: CanonStateSubscriptions
{
    fn subscribe_to_canonical_state(&self) -> CanonStateNotifications {
        self.queue.subscribe_to_canonical_state()
    }
}

impl<DB, Queue> ChangeSetReader for BlockchainProvider<DB, Queue>
where
    DB: Database,
    Queue: Sync + Send
{
    fn account_block_changeset(&self, block_number: BlockNumber) -> Result<Vec<AccountBeforeTx>> {
        self.database
            .provider()?
            .account_block_changeset(block_number)
    }
}
