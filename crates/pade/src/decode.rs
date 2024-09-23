use std::fmt::Debug;

pub trait PadeDecode {
    const PADE_HEADER_BITS: usize = 0;
    const PADE_VARIANT_MAP_BITS: usize = 0;

    fn pade_decode(buf: &mut [u8]) -> Result<Self, ()>
    where
        Self: Sized;

    fn pade_decode_with_width(buf: &mut [u8], width: usize) -> Result<Self, ()>
    where
        Self: Sized;
    /// The number of bytes in the PADE-encoded output that represent header
    /// information.  0 for most encoding schemes but Enum and List both
    /// have header metadata that are added
    #[inline]
    fn pade_header_bits(&self) -> usize {
        Self::PADE_HEADER_BITS
    }

    #[inline]
    fn pade_variant_map_bits(&self) -> usize {
        Self::PADE_VARIANT_MAP_BITS
    }
}

//Implementation for arrays
impl<T: PadeDecode + Debug, const N: usize> PadeDecode for [T; N] {
    fn pade_decode(buf: &mut [u8]) -> Result<Self, ()> {
        let mut this = vec![];
        for _ in 0..N {
            this.push(T::pade_decode(buf)?);
        }

        Ok(this.try_into().unwrap())
    }

    fn pade_decode_with_width(buf: &mut [u8], width: usize) -> Result<Self, ()> {
        let mut this = vec![];
        for _ in 0..N {
            this.push(T::pade_decode_with_width(buf, width)?);
        }

        Ok(this.try_into().unwrap())
    }
}

// Option<T: PadeEncode> encodes as an enum
impl<T: PadeDecode> PadeDecode for Option<T> {
    fn pade_decode(mut buf: &mut [u8]) -> Result<Self, ()> {
        if buf.len() == 0 {
            return Err(())
        }
        // check first byte;
        let ctr = buf[0] != 0;
        // progress buffer
        buf = &mut buf[1..];

        if ctr {
            Ok(Some(T::pade_decode(buf)?))
        } else {
            Ok(None)
        }
    }

    fn pade_decode_with_width(mut buf: &mut [u8], width: usize) -> Result<Self, ()> {
        if buf.len() == 0 {
            return Err(())
        }
        // check first byte;
        let ctr = buf[0] != 0;
        // progress buffer
        buf = &mut buf[1..];

        if ctr {
            Ok(Some(T::pade_decode_with_width(buf, width)?))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn pade_variant_map_bits(&self) -> usize {
        1
    }
}

impl PadeDecode for bool {
    fn pade_decode(mut buf: &mut [u8]) -> Result<Self, ()> {
        if buf.len() == 0 {
            return Err(())
        }
        // check first byte;
        let ctr = buf[0] != 0;
        // progress buffer
        buf = &mut buf[1..];
        Ok(ctr)
    }

    fn pade_decode_with_width(_: &mut [u8], _: usize) -> Result<Self, ()> {
        unreachable!()
    }

    #[inline]
    fn pade_variant_map_bits(&self) -> usize {
        1
    }
}

// Decided on a generic List<3> implementation - no variant bits because we
// don't want to hoist them in a struct
impl<T: PadeDecode> PadeDecode for Vec<T> {
    const PADE_HEADER_BITS: usize = 24;

    fn pade_decode(mut buf: &mut [u8]) -> Result<Self, ()> {
        if buf.len() < 3 {
            return Err(())
        }
        // read vec length.
        let length = &buf[0..3];
        let length = usize::from_be_bytes([0, 0, 0, 0, 0, length[0], length[1], length[2]]);

        // progress buf
        buf = &mut buf[3..];

        let mut res = Vec::with_capacity(length);
        for _ in 0..length {
            res.push(T::pade_decode(buf)?);
        }

        Ok(res)
    }

    fn pade_decode_with_width(mut buf: &mut [u8], width: usize) -> Result<Self, ()> {
        if buf.len() < 3 {
            return Err(())
        }
        // read vec length.
        let length = &buf[0..3];
        let length = usize::from_be_bytes([0, 0, 0, 0, 0, length[0], length[1], length[2]]);

        // progress buf
        buf = &mut buf[3..];

        let mut res = Vec::with_capacity(length);
        for _ in 0..length {
            res.push(T::pade_decode_with_width(buf, width)?);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::PadeDecode;
    use crate::PadeEncode;

    #[test]
    fn can_encode_decode_array() {
        let array = [100_u128, 300_u128, 256_u128];
        let mut bytes = array.pade_encode();
        assert!(array.pade_header_bits() == 0);
        assert!(array.pade_variant_map_bits() == 0);

        let decoded: [u128; 3] = PadeDecode::pade_decode(&mut bytes).unwrap();
    }

    #[test]
    fn can_encode_decode_vec() {
        let vec = vec![100_u128, 300_u128, 256_u128];
        vec.pade_encode();
        assert!(vec.pade_header_bits() == 24);
        assert!(vec.pade_variant_map_bits() == 0);
    }
}
