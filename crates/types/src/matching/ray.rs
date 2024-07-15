use std::ops::Deref;

use alloy_primitives::{aliases::U320, Uint, U256};
use malachite::{
    num::{
        arithmetic::traits::{DivRound, Pow},
        conversion::traits::{FromSciString, RoundingInto}
    },
    Natural, Rational
};

use super::{const_2_192, SqrtPriceX96};
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Ray(U256);

impl Deref for Ray {
    type Target = U256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<U256> for Ray {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl From<Ray> for U256 {
    fn from(value: Ray) -> Self {
        value.0
    }
}

impl From<u8> for Ray {
    fn from(value: u8) -> Self {
        let inner = Uint::from(value);
        Self(inner)
    }
}

// Can't do this because of something in the blsful crate, annoying!
// trait RayConvert {}

// impl<T: Into<U256> + RayConvert> From<T> for Ray {
//     fn from(value: T) -> Self {
//         Self(Uint::from(value))
//     }
// }

impl From<f64> for Ray {
    fn from(value: f64) -> Self {
        Self(U256::from((value * (10.0_f64.pow(27))).floor()))
    }
}

impl From<&Ray> for f64 {
    fn from(value: &Ray) -> Self {
        let numerator = Natural::from_limbs_asc(value.0.as_limbs());
        let denominator: Natural = Natural::from_sci_string("1e27").unwrap();
        println!("Denominator: {:?}", denominator);
        let price = Rational::from_naturals(numerator, denominator);
        let (res, _) = price.rounding_into(malachite::rounding_modes::RoundingMode::Floor);
        res
    }
}

impl From<SqrtPriceX96> for Ray {
    fn from(value: SqrtPriceX96) -> Self {
        let p: U320 = value.widening_mul(*value);

        let numerator =
            Natural::from_limbs_asc(p.as_limbs()) * Natural::from_sci_string("1e27").unwrap();
        let (res, _) =
            numerator.div_round(const_2_192(), malachite::rounding_modes::RoundingMode::Floor);
        // let res = numerator / const_2_192();
        // let denominator = Natural::power_of_2(192);
        // let price = Rational::from_naturals(numerator, denominator);
        // let (res, _): (Natural, _) =
        //     price.rounding_into(malachite::rounding_modes::RoundingMode::Floor);
        let reslimbs = res.into_limbs_asc();
        let output: U256 = Uint::from_limbs_slice(&reslimbs);
        Self(output)
    }
}

impl Ray {
    /// Uses malachite.rs to approximate this value as a floating point number.
    /// Converts from the internal U256 representation to an approximated f64
    /// representation, which is a change to the value of this number and why
    /// this isn't `From<Ray> for f64`
    pub fn as_f64(&self) -> f64 {
        self.into()
    }

    // pub fn from_sqrtx96(other: &SqrtPriceX96) -> Self {
    //     let bignum = U512::from_limbs(other.as_limbs());
    // }

    // pub fn to_sqrtx96(&self) -> U256 {
    //     let bignum = U512::from(self.0);
    // }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U160;
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn converts_to_and_from_f64() {
        let test_val: f64 = 123456.1234567899;
        let ray = Ray::from(test_val);
        let ray_float = ray.as_f64();
        assert_eq!(test_val, ray_float, "Ray float not equal to original float");

        // let tgt_uint: U256 =
        // Uint::from(123456123456789900000000000000000_u128);
        // Looks like we need to make some decisions about how close to a float
        // we're willing to get
        // assert_eq!(
        //     ray.0, tgt_uint,
        //     "Ray does not properly equal target
        // uint"
        // );
    }

    #[test]
    fn converts_from_sqrtpricex96() {
        let mut rng = thread_rng();
        // Make sure our random number fits in here
        let x: U160 = rng.sample(rand::distributions::Standard);
        // let random: U256 = U256::ZERO;
        // let sp = Ray(random);
        let sp: SqrtPriceX96 = Ray(Uint::from(x)).into();

        let rp: Ray = sp.into();
        let sptwo: SqrtPriceX96 = rp.into();
        let rptwo: Ray = sptwo.into();
        let spthree: SqrtPriceX96 = rptwo.into();
        let rpthree: Ray = spthree.into();

        println!("{:?} - {:?} - {:?}", rp, rptwo, rpthree);
        println!("{:?} - {:?} - {:?}", sp, sptwo, spthree);
        println!("{} - {}", rp.as_f64(), sp.as_f64());
        assert!(rp.as_f64() == sp.as_f64());
        assert!(rp == rptwo);
        assert!(rp == rpthree);
        assert!(sp == sptwo);
        assert!(sp == spthree);
    }
}
