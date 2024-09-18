use alloy::{
    primitives::{Address, Bytes, Signature, Signed, U256},
    sol_types::SolValue
};

use crate::PadeEncode;

/// Uses the default alloy `abi_encode_packed` to PADE-encode this type.  We
/// share many primitives with Alloy so this makes it simple to implement the
/// standard encoding for them.  This macro is only meant to run here, so we
/// don't have to worry about it being externally sound
macro_rules! use_alloy_default {
    ($( $x:ty ), *) => {
        $(
            impl PadeEncode for $x {
                fn pade_encode(&self) -> Vec<u8> {
                    self.abi_encode_packed()
                }
            }
        ) *
    };
}

use_alloy_default!(u16, u64, i32, U256, u128, Address, Bytes);

// Custom impl for Signature which needs validation
impl PadeEncode for Signature {
    fn pade_encode(&self) -> Vec<u8> {
        self.as_bytes().into_iter().collect()
    }
}

impl<const B: usize, const L: usize> PadeEncode for Signed<B, L> {
    fn pade_encode(&self) -> Vec<u8> {
        self.to_be_bytes::<B>().iter().cloned().collect()
    }
}
#[cfg(test)]
mod tests {
    use crate::PadeEncode;

    #[test]
    fn implemented_pade() {
        let tim = 128_u128;
        println!("{:?}", tim.pade_header_bits());
    }
}
