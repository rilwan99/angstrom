use alloy_sol_types::SolValue;
// use angstrom_types::sol_bindings::sol::{
//     SolMockContractMessage, SolPoolRewardsUpdate, SolRewardsUpdate
// };

mod primitives;
// Re-export bitvec so our macro crate can rely on it
pub use bitvec;

pub struct Sequence<const B: usize, T>(std::marker::PhantomData<T>);
impl<const B: usize, T> Sequence<B, T> {}

pub trait PadeEncode {
    fn pade_encode(&self) -> Vec<u8>;
    /// The number of bytes in the PADE-encoded output that represent header
    /// information.  0 for most encoding schemes but Enum and List both
    /// have header metadata that are added
    #[inline]
    fn pade_header_bits(&self) -> u8 {
        0
    }
}

//Implementation for arrays
impl<T: PadeEncode, const N: usize> PadeEncode for [T; N] {
    fn pade_encode(&self) -> Vec<u8> {
        self.iter().flat_map(|item| item.pade_encode()).collect()
    }
}

// Decided on a generic List<3> implementation - no header bits because we don't
// want to hoist them in a struct
impl<T: PadeEncode> PadeEncode for Vec<T> {
    fn pade_encode(&self) -> Vec<u8> {
        let len_bytes = self.len().to_le_bytes();
        let len = vec![len_bytes[0], len_bytes[1], len_bytes[2]];
        let items = self.iter().flat_map(|i| i.pade_encode()).collect();
        [len, items].concat()
    }
}

// Specific implementations for testing - these should go away soon
// impl PadeEncode for SolRewardsUpdate {
//     fn pade_encode(&self) -> Vec<u8> {
//         let start_tick = self.startTick.abi_encode_packed();
//         let start_liquidity = self.startLiquidity.abi_encode_packed();
//         let quantities = self.quantities.pade_encode();
//         [start_tick, start_liquidity, quantities].concat()
//     }
// }

// impl PadeEncode for SolPoolRewardsUpdate {
//     fn pade_encode(&self) -> Vec<u8> {
//         let a0 = self.asset0.abi_encode_packed();
//         let a1 = self.asset1.abi_encode_packed();
//         let update = self.update.pade_encode();
//         [a0, a1, update].concat()
//     }
// }

// impl PadeEncode for SolMockContractMessage {
//     fn pade_encode(&self) -> Vec<u8> {
//         let address_list = self.addressList.pade_encode();
//         let update = self.update.pade_encode();
//         [address_list, update].concat()
//     }
// }
