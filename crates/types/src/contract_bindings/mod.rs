use alloy_sol_macro::sol;

//sol!(Angstrom, "src/contract_bindings/angstrom.json");

//sol!(AngstromSpoof, "src/contract_bindings/angstrom_spoof.json");
sol! {
#![sol(all_derives = true)]
PoolManager,
"src/contract_bindings/pool_manager.json"}

sol! {
#![sol(all_derives = true)]
Angstrom,
"src/contract_bindings/angstrom.json"}

sol! {
#![sol(all_derives = true)]
ERC20,
"src/contract_bindings/ERC20.json"}

//pub use AngstromSpoof::*;
pub use Angstrom::*;
pub use PoolManager::*;
