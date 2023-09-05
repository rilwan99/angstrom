#[rustfmt::skip]
pub mod angstrom;
#[rustfmt::skip]
pub mod angstrom_spoof;
#[rustfmt::skip]
pub mod pool_manager;

use ethers::prelude::abigen;
abigen!(ERC20, "./src/contract_bindings/ERC20.json");
