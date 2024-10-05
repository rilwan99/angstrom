mod implementations;
pub use implementations::*;

mod primitives;
use std::ops::{Range, RangeInclusive};

pub use primitives::*;
use rand::{distributions::Standard, prelude::Distribution, rngs::ThreadRng, Rng, RngCore};
use reth_primitives::{Address, Bytes, U256};

use crate::sol_bindings::{
    grouped_orders::{AllOrders, StandingVariants},
    rpc_orders::{OrderMeta, PartialStandingOrder, TopOfBlockOrder},
    sol::{ContractBundle, SolGenericOrder, SolPrice}
};

// need to redefine the Random trait due to trait + types (reth) not being ours
pub trait Randomizer<T>: Rng {
    fn gen(&mut self) -> T;

    fn gen_many(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| Randomizer::gen(self)).collect()
    }
}

impl<T, R> Randomizer<T> for R
where
    Standard: Distribution<T>,
    R: Rng + ?Sized
{
    fn gen(&mut self) -> T {
        self.gen()
    }
}

pub trait RandomizerSized<T>: Rng {
    fn gen_sized<const SIZE: usize>(&mut self) -> T;

    fn gen_many_sized<const SIZE: usize>(&mut self, count: usize) -> Vec<T> {
        (0..count).map(|_| self.gen_sized::<SIZE>()).collect()
    }
}
