// use angstrom_types::sol_bindings::sol::{
//     SolMockContractMessage, SolPoolRewardsUpdate, SolRewardsUpdate
// };

mod decode;
mod encode;
mod primitives;
// Re-export bitvec so our macro crate can rely on it
pub use bitvec;
pub use decode::*;
pub use encode::*;

pub struct Sequence<const B: usize, T>(std::marker::PhantomData<T>);
impl<const B: usize, T> Sequence<B, T> {}

fn test(buf: &mut &[u8]) {
    let bit = bitvec::vec::BitVec::<u8, bitvec::order::Msb0>::from_slice(buf);
}
