use alloy_sol_macro::sol;

mod angstrom;
pub use angstrom::*;

sol! {
#![sol(all_derives = true)]
ERC20,
"src/primitive/contract/ERC20.json"}

pub use ERC20::*;
