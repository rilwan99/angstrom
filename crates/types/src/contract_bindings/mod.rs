use alloy_sol_macro::sol;

pub mod angstrom;

sol! {
#![sol(all_derives = true)]
ERC20,
"src/contract_bindings/ERC20.json"}

pub use angstrom::*;
