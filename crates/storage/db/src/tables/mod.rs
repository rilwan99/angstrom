//! Tables and data models.
//!
//! # Overview
//!
//! This module defines the tables in reth, as well as some table-related
//! abstractions:
//!
//! - [`codecs`] integrates different codecs into
//!   [`Encode`](crate::abstraction::table::Encode) and
//!   [`Decode`](crate::abstraction::table::Decode)
//! - [`models`] defines the values written to tables
//!
//! # Database Tour
//!
//! TODO(onbjerg): Find appropriate format for this...

pub mod codecs;
pub mod models;
mod raw;
pub(crate) mod utils;

use std::{fmt::Display, str::FromStr};

use guard_types::database::State;
pub use raw::{RawDupSort, RawKey, RawTable, RawValue, TableRawRow};
use reth_primitives::{
    stage::StageCheckpoint,
    trie::{BranchNodeCompact, StorageTrieEntry, StoredNibbles, StoredNibblesSubKey},
    Account, Address, BlockHash, BlockNumber, Bytecode, Header, IntegerList, PruneCheckpoint,
    PrunePart, Receipt, StorageEntry, TransactionSignedNoHash, TxHash, TxNumber, H256
};

use crate::abstraction::table::Table;
/// Declaration of all Database tables.
use crate::{
    table::DupSort,
    tables::{
        codecs::CompactU256,
        models::{
            accounts::{AccountBeforeTx, BlockNumberAddress},
            blocks::{HeaderHash, StoredBlockOmmers},
            storage_sharded_key::StorageShardedKey,
            ShardedKey, StoredBlockBodyIndices, StoredBlockWithdrawals
        }
    }
};

/// Enum for the types of tables present in libmdbx.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TableType {
    /// key value table
    Table,
    /// Duplicate key value table
    DupSort
}

/// Number of tables that should be present inside database.
pub const NUM_TABLES: usize = 26;

/// The general purpose of this is to use with a combination of Tables enum,
/// by implementing a `TableViewer` trait you can operate on db tables in an
/// abstract way.
///
/// # Example
///
/// ```
/// use reth_db::{ table::Table, TableViewer, Tables };
/// use std::str::FromStr;
///
/// let headers = Tables::from_str("Headers").unwrap();
/// let transactions = Tables::from_str("Transactions").unwrap();
///
/// struct MyTableViewer;
///
/// impl TableViewer<()> for MyTableViewer {
///     type Error = &'static str;
///
///     fn view<T: Table>(&self) -> Result<(), Self::Error> {
///         // operate on table in generic way
///         Ok(())
///     }
/// }
///
/// let viewer = MyTableViewer {};
///
/// let _ = headers.view(&viewer);
/// let _ = transactions.view(&viewer);
/// ```
pub trait TableViewer<R> {
    /// type of error to return
    type Error;

    /// operate on table in generic way
    fn view<T: Table>(&self) -> Result<R, Self::Error>;
}

macro_rules! tables {
    ([$(($table:ident, $type:expr)),*]) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        /// Default tables that should be present inside database.
        pub enum Tables {
            $(
                #[doc = concat!("Represents a ", stringify!($table), " table")]
                $table,
            )*
        }

        impl Tables {
            /// Array of all tables in database
            pub const ALL: [Tables; NUM_TABLES] = [$(Tables::$table,)*];

            /// The name of the given table in database
            pub const fn name(&self) -> &str {
                match self {
                    $(Tables::$table => {
                        $table::NAME
                    },)*
                }
            }

            /// The type of the given table in database
            pub const fn table_type(&self) -> TableType {
                match self {
                    $(Tables::$table => {
                        $type
                    },)*
                }
            }

            /// Allows to operate on specific table type
            pub fn view<T, R>(&self, visitor: &T) -> Result<R, T::Error>
            where
                T: TableViewer<R>,
            {
                match self {
                    $(Tables::$table => {
                        visitor.view::<$table>()
                    },)*
                }
            }
        }

        impl Display for Tables {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl FromStr for Tables {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($table::NAME => {
                        return Ok(Tables::$table)
                    },)*
                    _ => {
                        return Err("Unknown table".to_string())
                    }
                }
            }
        }
    };
}

tables!([
    (Blocks, TableType::Table),
    (ConsensusState, TableType::Table),
    (GuardSet, TableType::Table)
]);

#[macro_export]
/// Macro to declare key value table.
macro_rules! table {
    ($(#[$docs:meta])+ ( $table_name:ident ) $key:ty | $value:ty) => {
        $(#[$docs])+
        ///
        #[doc = concat!("Takes [`", stringify!($key), "`] as a key and returns [`", stringify!($value), "`]")]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $table_name;

        impl $crate::table::Table for $table_name {
            const NAME: &'static str = $table_name::const_name();
            type Key = $key;
            type Value = $value;
        }

        impl $table_name {
            #[doc=concat!("Return ", stringify!($table_name), " as it is present inside the database.")]
            pub const fn const_name() -> &'static str {
                stringify!($table_name)
            }
        }

        impl std::fmt::Display for $table_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($table_name))
            }
        }
    };
}

#[macro_export]
/// Macro to declare duplicate key value table.
macro_rules! dupsort {
    ($(#[$docs:meta])+ ( $table_name:ident ) $key:ty | [$subkey:ty] $value:ty) => {
        table!(
            $(#[$docs])+
            ///
            #[doc = concat!("`DUPSORT` table with subkey being: [`", stringify!($subkey), "`].")]
            ( $table_name ) $key | $value
        );
        impl DupSort for $table_name {
            type SubKey = $subkey;
        }
    };
}

//
//  TABLE DEFINITIONS
//

// table!(
/// Stores the block related to the block number
// ( Blocks ) BlockNumber | Block
// );

table!(
    /// Stores the State related to block number
    ( ConsensusState ) BlockNumber | State
);

table!(
    /// Stores the block number corresponding to a header.
    ( GuardSet ) BlockNumber | GuardSet
);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::*;

    const TABLES: [(TableType, &str); NUM_TABLES] = [
        (TableType::Table, CanonicalHeaders::const_name()),
        (TableType::Table, HeaderTD::const_name()),
        (TableType::Table, HeaderNumbers::const_name()),
        (TableType::Table, Headers::const_name()),
        (TableType::Table, BlockBodyIndices::const_name()),
        (TableType::Table, BlockOmmers::const_name()),
        (TableType::Table, BlockWithdrawals::const_name()),
        (TableType::Table, TransactionBlock::const_name()),
        (TableType::Table, Transactions::const_name()),
        (TableType::Table, TxHashNumber::const_name()),
        (TableType::Table, Receipts::const_name()),
        (TableType::Table, PlainAccountState::const_name()),
        (TableType::DupSort, PlainStorageState::const_name()),
        (TableType::Table, Bytecodes::const_name()),
        (TableType::Table, AccountHistory::const_name()),
        (TableType::Table, StorageHistory::const_name()),
        (TableType::DupSort, AccountChangeSet::const_name()),
        (TableType::DupSort, StorageChangeSet::const_name()),
        (TableType::Table, HashedAccount::const_name()),
        (TableType::DupSort, HashedStorage::const_name()),
        (TableType::Table, AccountsTrie::const_name()),
        (TableType::DupSort, StoragesTrie::const_name()),
        (TableType::Table, TxSenders::const_name()),
        (TableType::Table, SyncStage::const_name()),
        (TableType::Table, SyncStageProgress::const_name()),
        (TableType::Table, PruneCheckpoints::const_name())
    ];

    #[test]
    fn parse_table_from_str() {
        for (table_index, &(table_type, table_name)) in TABLES.iter().enumerate() {
            let table = Tables::from_str(table_name).unwrap();

            assert_eq!(table as usize, table_index);
            assert_eq!(table.table_type(), table_type);
            assert_eq!(table.name(), table_name);
        }
    }
}
