mod origin;
use std::{collections::HashMap, fmt};

pub mod orderpool;

pub use orderpool::*;
pub use origin::*;

mod pooled;
pub use pooled::*;

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
