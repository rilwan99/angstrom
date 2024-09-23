use alloy::{
    primitives::{aliases::I24, Address, Bytes, Signature, U256},
    sol_types::SolValue
};

use crate::{PadeDecode, PadeEncode};

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
        )*
    };
}

macro_rules! prim_decode {
    ($( $x:ty ), *) => {
        $(
            impl PadeDecode for $x {
                #[allow(unused)]
                fn pade_decode(mut buf: &mut [u8]) -> Result<Self, ()>
                where
                    Self: Sized
                {
                    const BYTES : usize  = <$x>::BITS as usize / 8usize;
                    let mut con_buf = [0u8; BYTES];
                    for i in 0..BYTES{
                        con_buf[i] = buf[i];
                    }
                    let res = <$x>::from_be_bytes(con_buf);
                    buf = &mut buf[BYTES..];
                    Ok(res)
                }

                #[allow(unused)]
                fn pade_decode_with_width(mut buf: &mut [u8], size: usize) -> Result<Self, ()>
                where
                    Self: Sized
                {
                    const BYTES : usize  = <$x>::BITS as usize / 8usize;
                    // grab the padding amount
                    let offset = size - BYTES as usize;
                    let subslice = &buf[offset..size];

                    let mut con_buf = [0u8; BYTES];
                    for i in 0..BYTES{
                        con_buf[i] = subslice[i];
                    }

                    let res = <$x>::from_be_bytes(con_buf);
                    buf = &mut buf[size..];

                    Ok(res)
                }
            }
        )*
    };
}

prim_decode!(u16, u64, i32, I24, U256, u128);
use_alloy_default!(u16, u64, i32, I24, U256, u128, Address, Bytes);

// Custom impl for Signature which needs validation
impl PadeEncode for Signature {
    fn pade_encode(&self) -> Vec<u8> {
        self.as_bytes().into_iter().collect()
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
