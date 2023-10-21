use alloy_sol_macro::sol;

//sol!(angstrom, "src/contract_bindings/angstrom.json");

//sol!(angstromSpoof, "src/contract_bindings/angstrom_spoof.json");
sol! {
#![sol(all_derives = true)]
PoolManager,
"src/contract_bindings/pool_manager.json"}

pub mod angstrom;

sol! {
#![sol(all_derives = true)]
ERC20,
"src/contract_bindings/ERC20.json"}

//pub use angstromSpoof::*;
pub use angstrom::*;
pub use PoolManager::*;
