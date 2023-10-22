use alloy_primitives::Address;
use hex_literal::hex;
mod bundle;
mod evidence;
mod external_state_sim;
mod lower_bound;
mod order;
mod signature;
mod submission;

pub use bundle::*;
pub use evidence::*;
pub use external_state_sim::*;
pub use lower_bound::*;
pub use order::*;
pub use signature::*;
pub use submission::*;

/// This type is for when we want to notify consensus of our new internal best
/// data.
#[derive(Debug, Clone)]
pub struct BestBundles {
    pub vanilla:     Option<VanillaBundle>,
    pub lower_bound: Option<LowerBound>,
    pub mev_bundle:  Option<MevBundle>
}

impl BestBundles {
    pub fn get_weight(&self) -> u128 {
        todo!()
    }
}
