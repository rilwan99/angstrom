use ethers_core::types::{Address, H160};
use hex_literal::hex;

mod bundle;
mod evidence;
mod external_state_sim;
mod lower_bound;
mod order;
mod signature;
mod submission;
mod uniswap_data;

pub use bundle::*;
pub use evidence::*;
pub use external_state_sim::*;
pub use lower_bound::*;
pub use order::*;
pub use signature::*;
pub use submission::*;
pub use uniswap_data::*;

/// 1234567890abcedf1234567890abcdef12345678
pub const ANGSTROM_CONTRACT_ADDR: H160 = H160(hex!("1234567890abcedf1234567890abcdef12345678"));
pub type Currency = Address;

/// This type is for when we want to notify consensus of our new internal best
/// data.
#[derive(Debug, Clone)]
pub struct BestBundles {
    pub vanilla:     Option<VanillaBundle>,
    pub lower_bound: Option<LowerBound>,
    pub non_vanilla: Option<ComposableBundle>
}

impl BestBundles {
    pub fn get_weight(&self) -> u128 {
        todo!()
    }
}
