use alloy_rlp::{Encodable, Decodable};
use bitmaps::Bitmap;
use bytes::BytesMut;
use bls_eth_rust::{PublicKey, Signature};
use serde::{de::Visitor, Deserialize, Serialize};
use std::ops::{BitAnd, BitOr};
// use base64::prelude::*;

/// BLS Validator ID is just going to be a u8 for now.  However, our bitmap is only 127 wide so be careful
pub type BLSValidatorID = u8;

// We might want to parameterize this to allow for different bitmap sizes but for now 
// we can probably just hardcode it
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BLSSignature {
    /// Bitmap denoting which validators have signed this aggregate signature
    validators_included: Bitmap<128>,
    /// BLS aggregated signature data
    aggregate_signature: Signature
}

impl BLSSignature {
    /// Validates the wrapped signature by using our validator bitmap to pull the appropriate public keys out
    /// of a supplied public key library.
    pub fn validate(&self, public_key_library: &[PublicKey], data: &[u8]) -> bool {
        // Use our bitmap to create a vector of appropriate public keys
        let included_public_keys: Vec<PublicKey> = self.validators_included.clone()
            .into_iter().filter_map(|i| public_key_library.get(i)).cloned().collect();  // Might be inefficient but let's not prematurely optimize
        self.aggregate_signature.fast_aggregate_verify(&included_public_keys, data)
    }

    /// Aggregate another `BLSSignature` into this one, modifying this signature in place.  This aggegration
    /// function will succeed and return `true` if both signatures are unique, i.e. no validator is included
    /// in both signatures.  If a validator is included in both signatures, the validator will return `false`
    /// and this signature will remain unaltered.
    pub fn simple_aggregate(&mut self, other: &Self) -> bool{
        // Make sure that we aren't doubling up on public keys
        let check = self.validators_included.bitand(other.validators_included);
        if check.first_index().is_some() { return false; }
        
        // Merge the two signatures
        self.aggregate_signature.add_assign(&other.aggregate_signature);
        self.validators_included = self.validators_included.bitor(other.validators_included);
        true
    }

    pub fn add_signature(&mut self, validator_id: BLSValidatorID, signature: Signature) -> bool {
        // If our validator is already included, just return false
        if self.validators_included.get(validator_id.into()) { return false; }
        // Otherwise note that we're included and aggregate the new signature
        self.validators_included.set(validator_id.into(), true);
        self.aggregate_signature.add_assign(&signature);
        true
    }
}

impl Serialize for BLSSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut buf = BytesMut::new();
        // Because of the size of our bitmap, this is u128.  Note that if we change
        // the size of the bitmap we might wind up changing the number of bytes written
        // and expected here!
        buf.extend_from_slice(&self.validators_included.as_value().to_le_bytes());
        buf.extend_from_slice(&self.aggregate_signature.serialize());
        //let encoded = BASE64_STANDARD.encode(buf.freeze());
        //serializer.serialize_str(&encoded)
        serializer.serialize_bytes(&buf.freeze())
    }
}

struct BLSSignatureVisitor {}
impl<'de> Visitor<'de> for BLSSignatureVisitor {
    type Value = BLSSignature;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("An n-wide byte array")
    }
    
    // fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    //     where
    //         E: serde::de::Error, {
    //     let decoded_bytes = BASE64_STANDARD.decode(v)
    //         .map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &"A base64-decodeable string"))?;
    //     if 16 > decoded_bytes.len() {
    //         return Err(serde::de::Error::invalid_length(decoded_bytes.len(), &"A byte string at least 16 bytes long"));
    //     }
    //     let (bitmap_bytes, sig_bytes) = decoded_bytes.split_at(16);
    //     // Parse out the u128 for out bitmap
    //     let bitmap_value = u128::from_le_bytes(bitmap_bytes.try_into().map_err(serde::de::Error::custom)?);
    //     let validators_included = Bitmap::from_value(bitmap_value);
    //     // Parse the remaining bytes into the signature data
    //     let aggregate_signature = Signature::from_serialized(sig_bytes)
    //         .map_err(|e| serde::de::Error::custom(format!("BLS Signature decode error: {:?}", e)))?;
    //     Ok(BLSSignature { validators_included, aggregate_signature })
    // }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        if 16 > v.len() {
            return Err(serde::de::Error::invalid_length(v.len(), &"A byte string at least 16 bytes long"));
        }
        let (bitmap_bytes, sig_bytes) = v.split_at(16);
        // Parse out the u128 for out bitmap
        let bitmap_value = u128::from_le_bytes(bitmap_bytes.try_into().map_err(serde::de::Error::custom)?);
        let validators_included = Bitmap::from_value(bitmap_value);
        // Parse the remaining bytes into the signature data
        let aggregate_signature = Signature::from_serialized(sig_bytes)
            .map_err(|e| serde::de::Error::custom(format!("BLS Signature decode error: {:?}", e)))?;
        Ok(BLSSignature { validators_included, aggregate_signature })
    }
}

impl<'de> Deserialize<'de> for BLSSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_bytes(BLSSignatureVisitor {})
    }
}

impl Encodable for BLSSignature {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.validators_included.as_value().to_le_bytes().encode(out);
        self.aggregate_signature.serialize().encode(out);
    }

    fn length(&self) -> usize {
        16 + self.aggregate_signature.serialize().len()
    }
}

impl Decodable for BLSSignature {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let v = buf.to_vec();
        let (bitmap_bytes, sig_bytes) = v.split_at(16);
        // Parse out the u128 for out bitmap
        let bitmap_value = u128::from_le_bytes(bitmap_bytes.try_into().map_err(|_| alloy_rlp::Error::Custom("Error deserializing bitmap"))?);
        let validators_included = Bitmap::from_value(bitmap_value);
        // Parse the remaining bytes into the signature data
        let aggregate_signature = Signature::from_serialized(sig_bytes)
            .map_err(|_| alloy_rlp::Error::Custom("Error deserializing signature - {:?}"))?;
        Ok(BLSSignature { validators_included, aggregate_signature })
    }
}

#[cfg(test)]
mod tests {
    use bls_eth_rust::SecretKey;
    use super::*;

    fn new_random_key() -> (PublicKey, SecretKey) {
        let mut key = SecretKey::zero();
        key.set_by_csprng();
        let pubkey = key.get_publickey();
        (pubkey, key)
    }
    
    /// Create an aggregate BLSSignature over arbitrary data that uses specific keys out of a larger
    /// list of keys 
    fn sign_data_with(keys: &[(PublicKey, SecretKey)], idxs: &[usize], data: &[u8]) -> BLSSignature {
        let mut validators_included = Bitmap::new();
        let sigs: Vec<Signature> = idxs.iter().map(|i| {
            validators_included.set(*i, true);
            keys[*i].1.sign(data)
        }).collect();
        let mut aggregate_signature = Signature::zero();
        aggregate_signature.aggregate(&sigs);
        BLSSignature { validators_included, aggregate_signature }
    }

    static FIXED_DATA: &str = "This is an arbitrary chunk of test data";

    #[test]
    fn will_sign_and_verify() {
        let mut sig = BLSSignature::default();
        let my_key = new_random_key();
        let my_signature = my_key.1.sign(&FIXED_DATA.as_bytes());
        sig.add_signature(0, my_signature);
        assert!(sig.validate(&[my_key.0], &FIXED_DATA.as_bytes()), "Unable to validate signature");
    }
    #[test]
    fn will_serialize_and_deserialize() {
        let mut sig = BLSSignature::default();
        let my_key = new_random_key();
        let my_signature = my_key.1.sign(&FIXED_DATA.as_bytes());
        sig.add_signature(0, my_signature);
        // Serialize the structure
        let serialized = serde_json::to_string(&sig).unwrap();
        println!("{}", serialized);
        // Deserialize the structure
        let deserialized: BLSSignature = serde_json::from_str(&serialized).unwrap();
        // Compare that both the before and after are the same
        assert_eq!(sig.validators_included, deserialized.validators_included, "Aggregate signatures don't match");
        assert_eq!(sig.aggregate_signature, deserialized.aggregate_signature, "Aggregate signatures don't match");
    }
    #[test]
    fn will_perform_simple_aggregation() {
        let keys: Vec<(PublicKey, SecretKey)> = (0..10).map(|_| new_random_key()).collect();
        let pubkeys: Vec<PublicKey> = keys.iter().map(|x| &x.0).cloned().collect();
        let mut sig = sign_data_with(&keys, &[1,3,5], &FIXED_DATA.as_bytes());
        let overlapping_sig = sign_data_with(&keys, &[2,4,5], &FIXED_DATA.as_bytes());
        let nonoverlapping_sig = sign_data_with(&keys, &[2,4,6], &FIXED_DATA.as_bytes());
        // Perform an overlapping aggregation, which should fail
        assert_eq!(sig.simple_aggregate(&overlapping_sig), false, "Overlapping aggregation permitted");
        // Assert that sig remains unchanged
        assert!(sig.validate(&pubkeys, &FIXED_DATA.as_bytes()), "Unable to validate signature");
        // Only bytes 1, 3, 5 should be set and no others
        assert!(sig.validators_included.next_index(0).unwrap() == 1);
        assert!(sig.validators_included.next_index(1).unwrap() == 3);
        assert!(sig.validators_included.next_index(3).unwrap() == 5);
        assert!(sig.validators_included.next_index(5).is_none());
        // Perform a valid nonoverlapping aggregation
        assert_eq!(sig.simple_aggregate(&nonoverlapping_sig), true, "Nonoverlapping aggregation denied");
        // Validate that sig has changed
        assert!(sig.validate(&pubkeys, &FIXED_DATA.as_bytes()), "Unable to validate signature");
        // All 6 bytes should be set but no others
        assert!(sig.validators_included.next_index(0).unwrap() == 1);
        assert!(sig.validators_included.next_index(1).unwrap() == 2);
        assert!(sig.validators_included.next_index(2).unwrap() == 3);
        assert!(sig.validators_included.next_index(3).unwrap() == 4);
        assert!(sig.validators_included.next_index(4).unwrap() == 5);
        assert!(sig.validators_included.next_index(5).unwrap() == 6);
        assert!(sig.validators_included.next_index(6).is_none());
    }
}