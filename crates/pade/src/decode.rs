use std::fmt::Debug;

use bitvec::{field::BitField, vec::BitVec};

use crate::PadeEncode;

pub struct VisualImpl {
    field_1: u128,
    field_2: u128,
    enum1:   Option<u8>,
    field_3: u64,
    enum2:   Visual
}

pub enum Visual {
    A { u: u128 },
    B(u128, u64),
    C(u64)
}

impl PadeEncode for VisualImpl {
    fn pade_encode(&self) -> Vec<u8> {
        todo!()
    }
}
impl PadeEncode for Visual {
    fn pade_encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl PadeDecode for Visual {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()>
    where
        Self: Sized
    {
        let variant = var.unwrap_or_else(|| {
            let ch = buf[0];
            *buf = &buf[1..];
            ch
        });
        match variant {
            0 => {
                let u = u128::pade_decode(buf, None)?;
                Ok(Self::A { u })
            }
            1 => {
                let f = u128::pade_decode(buf, None)?;
                let f2 = u64::pade_decode(buf, None)?;
                Ok(Self::B(f, f2))
            }
            2 => {
                let u = u64::pade_decode(buf, None)?;
                Ok(Self::C(u))
            }
            _ => return Err(())
        }
    }

    fn pade_decode_with_width(buf: &mut &[u8], width: usize, var: Option<u8>) -> Result<Self, ()>
    where
        Self: Sized
    {
        todo!()
    }
}

impl PadeDecode for VisualImpl {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()>
    where
        Self: Sized
    {
        let bitmap_bytes = Self::PADE_VARIANT_MAP_BITS.div_ceil(8);
        let mut bitmap = BitVec::<u8, bitvec::order::Msb0>::from_slice(&buf[0..bitmap_bytes]);
        let field_1 = u128::pade_decode(buf, var)?;
        let field_2 = u128::pade_decode(buf, var)?;

        let variant_bits = bitmap.split_off(<Option<u8>>::PADE_VARIANT_MAP_BITS);
        let var_e: u8 = variant_bits.load_be();

        let enum1 = <Option<u8>>::pade_decode(buf, Some(var_e))?;
        let field_3 = u64::pade_decode(buf, var)?;

        let variant_bits = bitmap.split_off(<Visual>::PADE_VARIANT_MAP_BITS);
        let var_e: u8 = variant_bits.load_be();
        let enum2 = <Visual>::pade_decode(buf, Some(var_e))?;

        Ok(Self { field_3, field_1, field_2, enum1, enum2 })
    }

    fn pade_decode_with_width(buf: &mut &[u8], width: usize, var: u8) -> Result<Self, ()>
    where
        Self: Sized
    {
        todo!()
    }
}

pub trait PadeDecode: super::PadeEncode {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()>
    where
        Self: Sized;

    /// the varient that was used if enum.
    fn pade_decode_with_width(buf: &mut &[u8], width: usize, var: Option<u8>) -> Result<Self, ()>
    where
        Self: Sized;
}

//Implementation for arrays
impl<T: PadeDecode + Debug, const N: usize> PadeDecode for [T; N] {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()> {
        let mut this = vec![];
        for _ in 0..N {
            this.push(T::pade_decode(buf, var)?);
        }

        Ok(this.try_into().unwrap())
    }

    fn pade_decode_with_width(buf: &mut &[u8], width: usize, var: Option<u8>) -> Result<Self, ()> {
        let mut this = vec![];
        for _ in 0..N {
            this.push(T::pade_decode_with_width(buf, width, var)?);
        }

        Ok(this.try_into().unwrap())
    }
}

// Option<T: PadeEncode> encodes as an enum
impl<T: PadeDecode> PadeDecode for Option<T> {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()> {
        if buf.len() == 0 {
            return Err(())
        }
        // check first byte;
        let ctr = buf[0] != 0;
        // progress buffer
        *buf = &buf[1..];

        if ctr {
            Ok(Some(T::pade_decode(buf, var)?))
        } else {
            Ok(None)
        }
    }

    fn pade_decode_with_width(buf: &mut &[u8], width: usize, var: Option<u8>) -> Result<Self, ()> {
        if buf.len() == 0 {
            return Err(())
        }
        // check first byte;
        let ctr = buf[0] != 0;
        // progress buffer
        *buf = &buf[1..];

        if ctr {
            Ok(Some(T::pade_decode_with_width(buf, width, var)?))
        } else {
            Ok(None)
        }
    }
}

impl PadeDecode for bool {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()> {
        if let Some(var) = var {
            return Ok(var != 0)
        }

        if buf.len() == 0 {
            return Err(())
        }
        // check first byte;
        let ctr = buf[0] != 0;
        // progress buffer
        *buf = &buf[1..];
        Ok(ctr)
    }

    fn pade_decode_with_width(_: &mut &[u8], _: usize, _: Option<u8>) -> Result<Self, ()> {
        unreachable!()
    }
}

// Decided on a generic List<3> implementation - no variant bits because we
// don't want to hoist them in a struct
impl<T: PadeDecode> PadeDecode for Vec<T> {
    fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()> {
        if buf.len() < 3 {
            return Err(())
        }
        // read vec length.
        let length = &buf[0..3];
        let length = usize::from_be_bytes([0, 0, 0, 0, 0, length[0], length[1], length[2]]);

        // progress buf pass offset
        *buf = &buf[3..];
        // capture length to ensure we don't over decode.
        let mut decode_slice = &buf[0..length];
        let mut res = Vec::new();
        while let Ok(d) = T::pade_decode(&mut decode_slice, var) {
            res.push(d);
        }
        assert!(decode_slice.len() == 0);

        // progress
        *buf = &buf[length..];

        Ok(res)
    }

    fn pade_decode_with_width(buf: &mut &[u8], width: usize, var: Option<u8>) -> Result<Self, ()> {
        if buf.len() < 3 {
            return Err(())
        }
        // read vec length.
        let length = &buf[0..3];
        let length = usize::from_be_bytes([0, 0, 0, 0, 0, length[0], length[1], length[2]]);

        // progress buf
        *buf = &buf[3..];

        let mut res = Vec::with_capacity(length);
        for _ in 0..length {
            res.push(T::pade_decode_with_width(buf, width, var)?);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {

    use crate::PadeEncode;

    #[test]
    fn can_encode_decode_array() {
        let array = [100_u128, 300_u128, 256_u128];
        let bytes = array.pade_encode();
        assert!(array.pade_header_bits() == 0);
        assert!(array.pade_variant_map_bits() == 0);
        let mut slice = bytes.as_slice();

        let decoded: [u128; 3] = super::PadeDecode::pade_decode(&mut slice, None).unwrap();
        assert_eq!(array, decoded);
    }

    #[test]
    fn can_encode_decode_vec() {
        let vec = vec![100_u128, 300_u128, 256_u128];
        let bytes = vec.pade_encode();
        assert!(vec.pade_header_bits() == 24);
        assert!(vec.pade_variant_map_bits() == 0);
        let mut slice = bytes.as_slice();

        let decoded: Vec<u128> = super::PadeDecode::pade_decode(&mut slice, None).unwrap();
        assert_eq!(vec, decoded);
    }
}
