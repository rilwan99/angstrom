use std::collections::HashMap;

use alloy_primitives::Address;

pub struct AngstromPools(HashMap<[u8; 40], (bool, usize)>);
