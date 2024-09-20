use alloy::{dyn_abi::Eip712Domain, sol, sol_types::eip712_domain};

mod angstrom;
pub use angstrom::{Angstrom::*, *};

sol! {
#![sol(all_derives = true)]
ERC20,
"src/primitive/contract/ERC20.json"}

pub use ERC20::*;

// The `eip712_domain` macro lets you easily define an EIP-712 domain
// object :)
pub const ANGSTROM_DOMAIN: Eip712Domain = eip712_domain!(
   name: "Angstrom",
   version: "1",
);
