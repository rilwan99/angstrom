use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::{Deref, DerefMut, Range, RangeBounds, RangeInclusive},
    sync::Arc
};

use itertools::{izip, Itertools};
use reth_db::{
    common::KeyValue,
    cursor::{DbCursorRO, DbCursorRW, DbDupCursorRO},
    database::{Database, DatabaseGAT},
    models::{
        sharded_key, storage_sharded_key::StorageShardedKey, AccountBeforeTx, BlockNumberAddress,
        ShardedKey, StoredBlockBodyIndices, StoredBlockOmmers, StoredBlockWithdrawals
    },
    table::{Table, TableRow},
    tables,
    transaction::{DbTx, DbTxMut},
    BlockNumberList, DatabaseError
};
use reth_interfaces::{
    executor::{BlockExecutionError, BlockValidationError},
    Result
};
use reth_primitives::{
    keccak256,
    stage::{StageCheckpoint, StageId},
    trie::Nibbles,
    Account, Address, Block, BlockHash, BlockHashOrNumber, BlockNumber, BlockWithSenders,
    ChainInfo, ChainSpec, Hardfork, Head, Header, PruneCheckpoint, PruneModes, PrunePart, Receipt,
    SealedBlock, SealedBlockWithSenders, SealedHeader, StorageEntry, TransactionMeta,
    TransactionSigned, TransactionSignedEcRecovered, TransactionSignedNoHash, TxHash, TxNumber,
    Withdrawal, H256, U256
};
use reth_revm_primitives::{
    config::revm_spec,
    env::{fill_block_env, fill_cfg_and_block_env, fill_cfg_env},
    primitives::{BlockEnv, CfgEnv, SpecId}
};
use reth_trie::{prefix_set::PrefixSetMut, StateRoot};

use crate::{
    guard_set::{GuardSetReader, GuardSetWriter},
    post_state::StorageChangeset,
    rewards::{RewardsReader, RewardsWriter},
    state::{StateReader, StateWriter},
    traits::{
        AccountExtReader, BlockSource, ChangeSetReader, ReceiptProvider, StageCheckpointWriter
    },
    AccountReader, BlockExecutionWriter, BlockHashReader, BlockNumReader, BlockReader, BlockWriter,
    EvmEnvProvider, HashingWriter, HeaderProvider, HistoryWriter, PostState, ProviderError,
    PruneCheckpointReader, PruneCheckpointWriter, StageCheckpointReader, StorageReader,
    TransactionsProvider, WithdrawalsProvider
};

/// A [`DatabaseProvider`] that holds a read-only database transaction.
pub type DatabaseProviderRO<'this, DB> = DatabaseProvider<'this, <DB as DatabaseGAT<'this>>::TX>;

/// A [`DatabaseProvider`] that holds a read-write database transaction.
///
/// Ideally this would be an alias type. However, there's some weird compiler error (<https://github.com/rust-lang/rust/issues/102211>), that forces us to wrap this in a struct instead.
/// Once that issue is solved, we can probably revert back to being an alias
/// type.
#[derive(Debug)]
pub struct DatabaseProviderRW<'this, DB: Database>(
    pub DatabaseProvider<'this, <DB as DatabaseGAT<'this>>::TXMut>
);

impl<'this, DB: Database> Deref for DatabaseProviderRW<'this, DB> {
    type Target = DatabaseProvider<'this, <DB as DatabaseGAT<'this>>::TXMut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'this, DB: Database> DerefMut for DatabaseProviderRW<'this, DB> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'this, DB: Database> DatabaseProviderRW<'this, DB> {
    /// Commit database transaction
    pub fn commit(self) -> Result<bool> {
        self.0.commit()
    }

    /// Consume `DbTx` or `DbTxMut`.
    pub fn into_tx(self) -> <DB as DatabaseGAT<'this>>::TXMut {
        self.0.into_tx()
    }
}

/// A provider struct that fetchs data from the database.
/// Wrapper around [`DbTx`] and [`DbTxMut`]. Example: [`HeaderProvider`]
/// [`BlockHashReader`]
#[derive(Debug)]
pub struct DatabaseProvider<'this, TX>
where
    Self: 'this
{
    /// Database transaction.
    tx:            TX,
    /// Chain spec
    chain_spec:    Arc<ChainSpec>,
    _phantom_data: std::marker::PhantomData<&'this TX>
}

impl<'this, TX: DbTxMut<'this>> DatabaseProvider<'this, TX> {
    /// Creates a provider with an inner read-write transaction.
    pub fn new_rw(tx: TX, chain_spec: Arc<ChainSpec>) -> Self {
        Self { tx, chain_spec, _phantom_data: std::marker::PhantomData }
    }
}

/// For a given key, unwind all history shards that are below the given block
/// number.
///
/// S - Sharded key subtype.
/// T - Table to walk over.
/// C - Cursor implementation.
///
/// This function walks the entries from the given start key and deletes all
/// shards that belong to the key and are below the given block number.
///
/// The boundary shard (the shard is split by the block number) is removed from
/// the database. Any indices that are above the block number are filtered out.
/// The boundary shard is returned for reinsertion (if it's not empty).
fn unwind_history_shards<'a, S, T, C>(
    cursor: &mut C,
    start_key: T::Key,
    block_number: BlockNumber,
    mut shard_belongs_to_key: impl FnMut(&T::Key) -> bool
) -> Result<Vec<usize>>
where
    T: Table<Value = BlockNumberList>,
    T::Key: AsRef<ShardedKey<S>>,
    C: DbCursorRO<'a, T> + DbCursorRW<'a, T>
{
    let mut item = cursor.seek_exact(start_key)?;
    while let Some((sharded_key, list)) = item {
        // If the shard does not belong to the key, break.
        if !shard_belongs_to_key(&sharded_key) {
            break
        }
        cursor.delete_current()?;

        // Check the first item.
        // If it is greater or eq to the block number, delete it.
        let first = list.iter(0).next().expect("List can't be empty");
        if first >= block_number as usize {
            item = cursor.prev()?;
            continue
        } else if block_number <= sharded_key.as_ref().highest_block_number {
            // Filter out all elements greater than block number.
            return Ok(list
                .iter(0)
                .take_while(|i| *i < block_number as usize)
                .collect::<Vec<_>>())
        } else {
            return Ok(list.iter(0).collect::<Vec<_>>())
        }
    }

    Ok(Vec::new())
}

impl<'this, TX: DbTx<'this>> DatabaseProvider<'this, TX> {
    /// Creates a provider with an inner read-only transaction.
    pub fn new(tx: TX, chain_spec: Arc<ChainSpec>) -> Self {
        Self { tx, chain_spec, _phantom_data: std::marker::PhantomData }
    }

    /// Consume `DbTx` or `DbTxMut`.
    pub fn into_tx(self) -> TX {
        self.tx
    }

    /// Pass `DbTx` or `DbTxMut` mutable reference.
    pub fn tx_mut(&mut self) -> &mut TX {
        &mut self.tx
    }

    /// Pass `DbTx` or `DbTxMut` immutable reference.
    pub fn tx_ref(&self) -> &TX {
        &self.tx
    }

    /// Return full table as Vec
    pub fn table<T: Table>(&self) -> std::result::Result<Vec<KeyValue<T>>, DatabaseError>
    where
        T::Key: Default + Ord
    {
        self.tx
            .cursor_read::<T>()?
            .walk(Some(T::Key::default()))?
            .collect::<std::result::Result<Vec<_>, DatabaseError>>()
    }
}

impl<'this, TX: DbTxMut<'this> + DbTx<'this>> DatabaseProvider<'this, TX> {
    /// Commit database transaction.
    pub fn commit(self) -> Result<bool> {
        Ok(self.tx.commit()?)
    }

    // TODO(joshie) TEMPORARY should be moved to trait providers

    /// Traverse over changesets and plain state and recreate the [`PostState`]s
    /// for the given range of blocks.
    ///
    /// 1. Iterate over the [BlockBodyIndices][tables::BlockBodyIndices] table
    ///    to get all
    /// the transaction ids.
    /// 2. Iterate over the [StorageChangeSet][tables::StorageChangeSet] table
    /// and the [AccountChangeSet][tables::AccountChangeSet] tables in reverse
    /// order to reconstruct the changesets.
    ///     - In order to have both the old and new values in the changesets, we
    ///       also access the plain state tables.
    /// 3. While iterating over the changeset tables, if we encounter a new
    ///    account or storage slot,
    /// we:
    ///     1. Take the old value from the changeset
    ///     2. Take the new value from the plain state
    ///     3. Save the old value to the local state
    /// 4. While iterating over the changeset tables, if we encounter an
    ///    account/storage slot we
    /// have seen before we:
    ///     1. Take the old value from the changeset
    ///     2. Take the new value from the local state
    ///     3. Set the local state to the value in the changeset
    ///
    /// If `TAKE` is `true`, the local state will be written to the plain state
    /// tables.
    /// 5. Get all receipts from table
    fn get_take_block_execution_result_range<const TAKE: bool>(
        &self,
        range: RangeInclusive<BlockNumber>
    ) -> Result<Vec<PostState>> {
        if range.is_empty() {
            return Ok(Vec::new())
        }

        // We are not removing block meta as it is used to get block changesets.
        let block_bodies = self.get_or_take::<tables::BlockBodyIndices, false>(range.clone())?;

        // get transaction receipts
        let from_transaction_num = block_bodies
            .first()
            .expect("already checked if there are blocks")
            .1
            .first_tx_num();
        let to_transaction_num = block_bodies
            .last()
            .expect("already checked if there are blocks")
            .1
            .last_tx_num();
        let receipts =
            self.get_or_take::<tables::Receipts, TAKE>(from_transaction_num..=to_transaction_num)?;

        let storage_range = BlockNumberAddress::range(range.clone());

        let storage_changeset =
            self.get_or_take::<tables::StorageChangeSet, TAKE>(storage_range)?;
        let account_changeset = self.get_or_take::<tables::AccountChangeSet, TAKE>(range)?;

        // iterate previous value and get plain state value to create changeset
        // Double option around Account represent if Account state is know (first
        // option) and account is removed (Second Option)
        type LocalPlainState = BTreeMap<Address, (Option<Option<Account>>, BTreeMap<H256, U256>)>;

        let mut local_plain_state: LocalPlainState = BTreeMap::new();

        // iterate in reverse and get plain state.

        // Bundle execution changeset to its particular transaction and block
        let mut block_states = BTreeMap::from_iter(
            block_bodies
                .iter()
                .map(|(num, _)| (*num, PostState::default()))
        );

        let mut plain_accounts_cursor = self.tx.cursor_write::<tables::PlainAccountState>()?;
        let mut plain_storage_cursor = self.tx.cursor_dup_write::<tables::PlainStorageState>()?;

        // add account changeset changes
        for (block_number, account_before) in account_changeset.into_iter().rev() {
            let AccountBeforeTx { info: old_info, address } = account_before;
            let new_info = match local_plain_state.entry(address) {
                Entry::Vacant(entry) => {
                    let new_account = plain_accounts_cursor.seek_exact(address)?.map(|kv| kv.1);
                    entry.insert((Some(old_info), BTreeMap::new()));
                    new_account
                }
                Entry::Occupied(mut entry) => {
                    let new_account = std::mem::replace(&mut entry.get_mut().0, Some(old_info));
                    new_account.expect(
                        "As we are stacking account first, account would always be Some(Some) or \
                         Some(None)"
                    )
                }
            };

            let post_state = block_states.entry(block_number).or_default();
            match (old_info, new_info) {
                (Some(old), Some(new)) => {
                    if new != old {
                        post_state.change_account(block_number, address, old, new);
                    } else {
                        unreachable!(
                            "Junk data in database: an account changeset did not represent any \
                             change"
                        );
                    }
                }
                (None, Some(account)) => post_state.create_account(block_number, address, account),
                (Some(old), None) => post_state.destroy_account(block_number, address, old),
                (None, None) => unreachable!(
                    "Junk data in database: an account changeset transitioned from no account to \
                     no account"
                )
            };
        }

        // add storage changeset changes
        let mut storage_changes: BTreeMap<BlockNumberAddress, StorageChangeset> = BTreeMap::new();
        for (block_and_address, storage_entry) in storage_changeset.into_iter().rev() {
            let BlockNumberAddress((_, address)) = block_and_address;
            let new_storage = match local_plain_state
                .entry(address)
                .or_default()
                .1
                .entry(storage_entry.key)
            {
                Entry::Vacant(entry) => {
                    let new_storage = plain_storage_cursor
                        .seek_by_key_subkey(address, storage_entry.key)?
                        .filter(|storage| storage.key == storage_entry.key)
                        .unwrap_or_default();
                    entry.insert(storage_entry.value);
                    new_storage.value
                }
                Entry::Occupied(mut entry) => {
                    std::mem::replace(entry.get_mut(), storage_entry.value)
                }
            };
            storage_changes
                .entry(block_and_address)
                .or_default()
                .insert(
                    U256::from_be_bytes(storage_entry.key.0),
                    (storage_entry.value, new_storage)
                );
        }

        for (BlockNumberAddress((block_number, address)), storage_changeset) in
            storage_changes.into_iter()
        {
            block_states
                .entry(block_number)
                .or_default()
                .change_storage(block_number, address, storage_changeset);
        }

        if TAKE {
            // iterate over local plain state remove all account and all storages.
            for (address, (account, storage)) in local_plain_state.into_iter() {
                // revert account
                if let Some(account) = account {
                    let existing_entry = plain_accounts_cursor.seek_exact(address)?;
                    if let Some(account) = account {
                        plain_accounts_cursor.upsert(address, account)?;
                    } else if existing_entry.is_some() {
                        plain_accounts_cursor.delete_current()?;
                    }
                }

                // revert storages
                for (storage_key, storage_value) in storage.into_iter() {
                    let storage_entry = StorageEntry { key: storage_key, value: storage_value };
                    // delete previous value
                    // TODO: This does not use dupsort features
                    if plain_storage_cursor
                        .seek_by_key_subkey(address, storage_key)?
                        .filter(|s| s.key == storage_key)
                        .is_some()
                    {
                        plain_storage_cursor.delete_current()?
                    }

                    // TODO: This does not use dupsort features
                    // insert value if needed
                    if storage_value != U256::ZERO {
                        plain_storage_cursor.upsert(address, storage_entry)?;
                    }
                }
            }
        }

        // iterate over block body and create ExecutionResult
        let mut receipt_iter = receipts.into_iter();

        // loop break if we are at the end of the blocks.
        for (block_number, block_body) in block_bodies.into_iter() {
            for _ in block_body.tx_num_range() {
                if let Some((_, receipt)) = receipt_iter.next() {
                    block_states
                        .entry(block_number)
                        .or_default()
                        .add_receipt(block_number, receipt);
                }
            }
        }
        Ok(block_states.into_values().collect())
    }

    /// Return list of entries from table
    ///
    /// If TAKE is true, opened cursor would be write and it would delete all
    /// values from db.
    #[inline]
    pub fn get_or_take<T: Table, const TAKE: bool>(
        &self,
        range: impl RangeBounds<T::Key>
    ) -> std::result::Result<Vec<KeyValue<T>>, DatabaseError> {
        if TAKE {
            let mut cursor_write = self.tx.cursor_write::<T>()?;
            let mut walker = cursor_write.walk_range(range)?;
            let mut items = Vec::new();
            while let Some(i) = walker.next().transpose()? {
                walker.delete_current()?;
                items.push(i)
            }
            Ok(items)
        } else {
            self.tx
                .cursor_read::<T>()?
                .walk_range(range)?
                .collect::<std::result::Result<Vec<_>, _>>()
        }
    }

    /// Get requested blocks transaction with signer
    pub(crate) fn get_take_block_transaction_range<const TAKE: bool>(
        &self,
        range: impl RangeBounds<BlockNumber> + Clone
    ) -> Result<Vec<(BlockNumber, Vec<TransactionSignedEcRecovered>)>> {
        // Raad range of block bodies to get all transactions id's of this range.
        let block_bodies = self.get_or_take::<tables::BlockBodyIndices, false>(range)?;

        if block_bodies.is_empty() {
            return Ok(Vec::new())
        }

        // Compute the first and last tx ID in the range
        let first_transaction = block_bodies
            .first()
            .expect("If we have headers")
            .1
            .first_tx_num();
        let last_transaction = block_bodies.last().expect("Not empty").1.last_tx_num();

        // If this is the case then all of the blocks in the range are empty
        if last_transaction < first_transaction {
            return Ok(block_bodies
                .into_iter()
                .map(|(n, _)| (n, Vec::new()))
                .collect())
        }

        // Get transactions and senders
        let transactions = self
            .get_or_take::<tables::Transactions, TAKE>(first_transaction..=last_transaction)?
            .into_iter()
            .map(|(id, tx)| (id, tx.into()))
            .collect::<Vec<(u64, TransactionSigned)>>();

        let mut senders =
            self.get_or_take::<tables::TxSenders, TAKE>(first_transaction..=last_transaction)?;

        // Recover senders manually if not found in db
        // SAFETY: Transactions are always guaranteed to be in the database whereas
        // senders might be pruned.
        if senders.len() != transactions.len() {
            senders.reserve(transactions.len() - senders.len());
            // Find all missing senders, their corresponding tx numbers and indexes to the
            // original `senders` vector at which the recovered senders will be
            // inserted.
            let mut missing_senders = Vec::with_capacity(transactions.len() - senders.len());
            {
                let mut senders = senders.iter().peekable();

                // `transactions` contain all entries. `senders` contain _some_ of the senders
                // for these transactions. Both are sorted and indexed by
                // `TxNumber`.
                //
                // The general idea is to iterate on both `transactions` and `senders`, and
                // advance the `senders` iteration only if it matches the
                // current `transactions` entry's `TxNumber`. Otherwise, add the
                // transaction to the list of missing senders.
                for (i, (tx_number, transaction)) in transactions.iter().enumerate() {
                    if let Some((sender_tx_number, _)) = senders.peek() {
                        if sender_tx_number == tx_number {
                            // If current sender's `TxNumber` matches current transaction's
                            // `TxNumber`, advance the senders iterator.
                            senders.next();
                        } else {
                            // If current sender's `TxNumber` doesn't match current transaction's
                            // `TxNumber`, add it to missing senders.
                            missing_senders.push((i, tx_number, transaction));
                        }
                    } else {
                        // If there's no more senders left, but we're still iterating over
                        // transactions, add them to missing senders
                        missing_senders.push((i, tx_number, transaction));
                    }
                }
            }

            // Recover senders
            let recovered_senders = TransactionSigned::recover_signers(
                missing_senders
                    .iter()
                    .map(|(_, _, tx)| *tx)
                    .collect::<Vec<_>>(),
                missing_senders.len()
            )
            .ok_or(BlockExecutionError::Validation(BlockValidationError::SenderRecoveryError))?;

            // Insert recovered senders along with tx numbers at the corresponding indexes
            // to the original `senders` vector
            for ((i, tx_number, _), sender) in missing_senders.into_iter().zip(recovered_senders) {
                // Insert will put recovered senders at necessary positions and shift the rest
                senders.insert(i, (*tx_number, sender));
            }

            // Debug assertions which are triggered during the test to ensure that all
            // senders are present and sorted
            debug_assert_eq!(senders.len(), transactions.len(), "missing one or more senders");
            debug_assert!(
                senders.iter().tuple_windows().all(|(a, b)| a.0 < b.0),
                "senders not sorted"
            );
        }

        if TAKE {
            // Remove TxHashNumber
            let mut tx_hash_cursor = self.tx.cursor_write::<tables::TxHashNumber>()?;
            for (_, tx) in transactions.iter() {
                if tx_hash_cursor.seek_exact(tx.hash())?.is_some() {
                    tx_hash_cursor.delete_current()?;
                }
            }

            // Remove TransactionBlock index if there are transaction present
            if !transactions.is_empty() {
                let tx_id_range = transactions.first().unwrap().0..=transactions.last().unwrap().0;
                self.get_or_take::<tables::TransactionBlock, TAKE>(tx_id_range)?;
            }
        }

        // Merge transaction into blocks
        let mut block_tx = Vec::with_capacity(block_bodies.len());
        let mut senders = senders.into_iter();
        let mut transactions = transactions.into_iter();
        for (block_number, block_body) in block_bodies {
            let mut one_block_tx = Vec::with_capacity(block_body.tx_count as usize);
            for _ in block_body.tx_num_range() {
                let tx = transactions.next();
                let sender = senders.next();

                let recovered = match (tx, sender) {
                    (Some((tx_id, tx)), Some((sender_tx_id, sender))) => {
                        if tx_id != sender_tx_id {
                            Err(ProviderError::MismatchOfTransactionAndSenderId { tx_id })
                        } else {
                            Ok(TransactionSignedEcRecovered::from_signed_transaction(tx, sender))
                        }
                    }
                    (Some((tx_id, _)), _) | (_, Some((tx_id, _))) => {
                        Err(ProviderError::MismatchOfTransactionAndSenderId { tx_id })
                    }
                    (None, None) => Err(ProviderError::BlockBodyTransactionCount)
                }?;
                one_block_tx.push(recovered)
            }
            block_tx.push((block_number, one_block_tx));
        }

        Ok(block_tx)
    }

    /// Return range of blocks and its execution result
    fn get_take_block_range<const TAKE: bool>(
        &self,
        chain_spec: &ChainSpec,
        range: impl RangeBounds<BlockNumber> + Clone
    ) -> Result<Vec<SealedBlockWithSenders>> {
        // For block we need Headers, Bodies, Uncles, withdrawals, Transactions, Signers

        let block_headers = self.get_or_take::<tables::Headers, TAKE>(range.clone())?;
        if block_headers.is_empty() {
            return Ok(Vec::new())
        }

        let block_header_hashes =
            self.get_or_take::<tables::CanonicalHeaders, TAKE>(range.clone())?;
        let block_ommers = self.get_or_take::<tables::BlockOmmers, TAKE>(range.clone())?;
        let block_withdrawals =
            self.get_or_take::<tables::BlockWithdrawals, TAKE>(range.clone())?;

        let block_tx = self.get_take_block_transaction_range::<TAKE>(range.clone())?;

        if TAKE {
            // rm HeaderTD
            self.get_or_take::<tables::HeaderTD, TAKE>(range)?;
            // rm HeaderNumbers
            let mut header_number_cursor = self.tx.cursor_write::<tables::HeaderNumbers>()?;
            for (_, hash) in block_header_hashes.iter() {
                if header_number_cursor.seek_exact(*hash)?.is_some() {
                    header_number_cursor.delete_current()?;
                }
            }
        }

        // merge all into block
        let block_header_iter = block_headers.into_iter();
        let block_header_hashes_iter = block_header_hashes.into_iter();
        let block_tx_iter = block_tx.into_iter();

        // Ommers can be empty for some blocks
        let mut block_ommers_iter = block_ommers.into_iter();
        let mut block_withdrawals_iter = block_withdrawals.into_iter();
        let mut block_ommers = block_ommers_iter.next();
        let mut block_withdrawals = block_withdrawals_iter.next();

        let mut blocks = Vec::new();
        for ((main_block_number, header), (_, header_hash), (_, tx)) in
            izip!(block_header_iter.into_iter(), block_header_hashes_iter, block_tx_iter)
        {
            let header = header.seal(header_hash);

            let (body, senders) = tx.into_iter().map(|tx| tx.to_components()).unzip();

            // Ommers can be missing
            let mut ommers = Vec::new();
            if let Some((block_number, _)) = block_ommers.as_ref() {
                if *block_number == main_block_number {
                    ommers = block_ommers.take().unwrap().1.ommers;
                    block_ommers = block_ommers_iter.next();
                }
            };

            // withdrawal can be missing
            let shanghai_is_active = chain_spec
                .fork(Hardfork::Shanghai)
                .active_at_timestamp(header.timestamp);
            let mut withdrawals = Some(Vec::new());
            if shanghai_is_active {
                if let Some((block_number, _)) = block_withdrawals.as_ref() {
                    if *block_number == main_block_number {
                        withdrawals = Some(block_withdrawals.take().unwrap().1.withdrawals);
                        block_withdrawals = block_withdrawals_iter.next();
                    }
                }
            } else {
                withdrawals = None
            }

            blocks.push(SealedBlockWithSenders {
                block: SealedBlock { header, body, ommers, withdrawals },
                senders
            })
        }

        Ok(blocks)
    }

    /// Unwind table by some number key.
    /// Returns number of rows unwound.
    ///
    /// Note: Key is not inclusive and specified key would stay in db.
    #[inline]
    pub fn unwind_table_by_num<T>(&self, num: u64) -> std::result::Result<usize, DatabaseError>
    where
        T: Table<Key = u64>
    {
        self.unwind_table::<T, _>(num, |key| key)
    }

    /// Unwind the table to a provided number key.
    /// Returns number of rows unwound.
    ///
    /// Note: Key is not inclusive and specified key would stay in db.
    pub(crate) fn unwind_table<T, F>(
        &self,
        key: u64,
        mut selector: F
    ) -> std::result::Result<usize, DatabaseError>
    where
        T: Table,
        F: FnMut(T::Key) -> u64
    {
        let mut cursor = self.tx.cursor_write::<T>()?;
        let mut reverse_walker = cursor.walk_back(None)?;
        let mut deleted = 0;

        while let Some(Ok((entry_key, _))) = reverse_walker.next() {
            if selector(entry_key.clone()) <= key {
                break
            }
            reverse_walker.delete_current()?;
            deleted += 1;
        }

        Ok(deleted)
    }

    /// Unwind a table forward by a
    /// [Walker][reth_db::abstraction::cursor::Walker] on another table
    pub fn unwind_table_by_walker<T1, T2>(
        &self,
        start_at: T1::Key
    ) -> std::result::Result<(), DatabaseError>
    where
        T1: Table,
        T2: Table<Key = T1::Value>
    {
        let mut cursor = self.tx.cursor_write::<T1>()?;
        let mut walker = cursor.walk(Some(start_at))?;
        while let Some((_, value)) = walker.next().transpose()? {
            self.tx.delete::<T2>(value, None)?;
        }
        Ok(())
    }

    /// Prune the table for the specified pre-sorted key iterator.
    ///
    /// Returns number of rows pruned.
    pub fn prune_table_with_iterator<T: Table>(
        &self,
        keys: impl IntoIterator<Item = T::Key>,
        limit: usize,
        mut delete_callback: impl FnMut(TableRow<T>)
    ) -> std::result::Result<(usize, bool), DatabaseError> {
        let mut cursor = self.tx.cursor_write::<T>()?;
        let mut deleted = 0;

        let mut keys = keys.into_iter();
        for key in &mut keys {
            let row = cursor.seek_exact(key.clone())?;
            if let Some(row) = row {
                cursor.delete_current()?;
                deleted += 1;
                delete_callback(row);
            }

            if deleted == limit {
                break
            }
        }

        Ok((deleted, keys.next().is_none()))
    }

    /// Prune the table for the specified key range.
    ///
    /// Returns number of total unique keys and total rows pruned pruned.
    pub fn prune_table_with_range<T: Table>(
        &self,
        keys: impl RangeBounds<T::Key> + Clone + Debug,
        limit: usize,
        mut skip_filter: impl FnMut(&TableRow<T>) -> bool,
        mut delete_callback: impl FnMut(TableRow<T>)
    ) -> std::result::Result<(usize, bool), DatabaseError> {
        let mut cursor = self.tx.cursor_write::<T>()?;
        let mut walker = cursor.walk_range(keys)?;
        let mut deleted = 0;

        while let Some(row) = walker.next().transpose()? {
            if !skip_filter(&row) {
                walker.delete_current()?;
                deleted += 1;
                delete_callback(row);
            }

            if deleted == limit {
                break
            }
        }

        Ok((deleted, walker.next().transpose()?.is_none()))
    }

    /// Load shard and remove it. If list is empty, last shard was full or
    /// there are no shards at all.
    fn take_shard<T>(&self, key: T::Key) -> Result<Vec<u64>>
    where
        T: Table<Value = BlockNumberList>
    {
        let mut cursor = self.tx.cursor_read::<T>()?;
        let shard = cursor.seek_exact(key)?;
        if let Some((shard_key, list)) = shard {
            // delete old shard so new one can be inserted.
            self.tx.delete::<T>(shard_key, None)?;
            let list = list.iter(0).map(|i| i as u64).collect::<Vec<_>>();
            return Ok(list)
        }
        Ok(Vec::new())
    }

    /// Insert history index to the database.
    ///
    /// For each updated partial key, this function removes the last shard from
    /// the database (if any), appends the new indices to it, chunks the
    /// resulting integer list and inserts the new shards back into the
    /// database.
    ///
    /// This function is used by history indexing stages.
    fn append_history_index<P, T>(
        &self,
        index_updates: BTreeMap<P, Vec<u64>>,
        mut sharded_key_factory: impl FnMut(P, BlockNumber) -> T::Key
    ) -> Result<()>
    where
        P: Copy,
        T: Table<Value = BlockNumberList>
    {
        for (partial_key, indices) in index_updates {
            let last_shard = self.take_shard::<T>(sharded_key_factory(partial_key, u64::MAX))?;
            // chunk indices and insert them in shards of N size.
            let indices = last_shard.iter().chain(indices.iter());
            let chunks = indices
                .chunks(sharded_key::NUM_OF_INDICES_IN_SHARD)
                .into_iter()
                .map(|chunks| chunks.map(|i| *i as usize).collect::<Vec<usize>>())
                .collect::<Vec<_>>();

            let mut chunks = chunks.into_iter().peekable();
            while let Some(list) = chunks.next() {
                let highest_block_number = if chunks.peek().is_some() {
                    *list.last().expect("`chunks` does not return empty list") as u64
                } else {
                    // Insert last list with u64::MAX
                    u64::MAX
                };
                self.tx.put::<T>(
                    sharded_key_factory(partial_key, highest_block_number),
                    BlockNumberList::new_pre_sorted(list)
                )?;
            }
        }
        Ok(())
    }
}

impl<'this, TX: DbTx<'this>> GuardSetReader for DatabaseProvider<'this, TX> {
    fn get_head(
        &self
    ) -> std::result::Result<guard_types::consensus::GuardSet, crate::DatabaseError> {
        todo!()
    }

    fn get_set_for_block(
        &self,
        block: u64
    ) -> std::result::Result<guard_types::consensus::GuardSet, crate::DatabaseError> {
        todo!()
    }
}

impl<'this, TX: DbTx<'this>> GuardSetWriter for DatabaseProvider<'this, TX> {
    fn new_guard_set(
        &self,
        block: u64,
        set: guard_types::consensus::GuardSet
    ) -> std::result::Result<(), crate::DatabaseError> {
        todo!()
    }
}

impl<'this, TX: DbTx<'this>> StateReader for DatabaseProvider<'this, TX> {
    fn get_state_range(
        &self,
        range: impl RangeBounds<u64>
    ) -> std::result::Result<Vec<guard_types::database::State>, crate::DatabaseError> {
        todo!()
    }

    fn get_newest_state(
        &self
    ) -> std::result::Result<guard_types::database::State, crate::DatabaseError> {
        todo!()
    }

    fn get_state_for_block(
        &self,
        block: u64
    ) -> std::result::Result<Option<guard_types::database::State>, crate::DatabaseError> {
        todo!()
    }
}

impl<'this, TX: DbTx<'this>> StateWriter for DatabaseProvider<'this, TX> {
    fn write_state(
        &self,
        block: u64,
        state: guard_types::database::State
    ) -> std::result::Result<(), crate::DatabaseError> {
        todo!()
    }
}

impl<'this, TX: DbTx<'this>> RewardsReader for DatabaseProvider<'this, TX> {
    fn rewards_for_block(
        &self,
        block: u64
    ) -> std::result::Result<Option<RewardsHeader>, crate::DatabaseError> {
        todo!()
    }

    fn rewards_from_range(
        &self,
        range: impl RangeBounds<u64>
    ) -> std::result::Result<Vec<RewardsHeader>, crate::DatabaseError> {
        todo!()
    }
}

impl<'this, TX: DbTx<'this>> RewardsWriter for DatabaseProvider<'this, TX> {
    fn new_reward(
        &self,
        block: u64,
        reward: RewardsHeader
    ) -> std::result::Result<(), crate::DatabaseError> {
        todo!()
    }
}
