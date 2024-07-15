mod origin;
use std::{collections::HashMap, fmt};

pub mod orderpool;

use alloy_rlp::{RlpDecodable, RlpEncodable};
pub use orderpool::*;
pub use origin::*;

mod pooled;
pub use pooled::*;
use serde::{Deserialize, Serialize};

use crate::{
    primitive::PoolId,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        sol::TopOfBlockOrder
    }
};

#[derive(Debug)]
pub struct OrderSet<Limit, Searcher> {
    pub limit:    Vec<OrderWithStorageData<Limit>>,
    pub searcher: Vec<OrderWithStorageData<Searcher>>
}

#[derive(
    Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq, RlpEncodable, RlpDecodable,
)]
pub struct PoolSolution {
    /// Id of this pool
    pub id:       PoolId,
    /// Winning searcher order to be executed
    pub searcher: OrderWithStorageData<TopOfBlockOrder>,
    /// IDs of limit orders to be executed - it might be easier to just use
    /// hashes here
    pub limit:    Vec<OrderId>
}
