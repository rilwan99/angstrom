use alloy_sol_macro::sol;

//sol!(angstrom, "src/contract_bindings/angstrom.json");

//sol!(angstromSpoof, "src/contract_bindings/angstrom_spoof.json");

pub mod angstrom;
pub mod pool_manager;

sol! {
#![sol(all_derives = true)]
ERC20,
"src/contract_bindings/ERC20.json"}

//pub use angstromSpoof::*;
pub use angstrom::*;
pub use pool_manager::*;
