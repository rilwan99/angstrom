use alloy_rlp::{Encodable, Decodable};
use bitmaps::Bitmap;
use bytes::BytesMut;
use serde::{de::Visitor, Deserialize, Serialize};
use std::ops::{BitAnd, BitOr};
// use base64::prelude::*;
use blsful::{Bls12381G1Impl, MultiPublicKey, MultiSignature, PublicKey, Signature};

/// BLS Validator ID is just going to be a u8 for now.  However, our bitmap is only 127 wide so be careful
pub type BLSValidatorID = u8;

// We might want to parameterize this to allow for different bitmap sizes but for now 
// we can probably just hardcode it
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BLSSignature {
    /// Bitmap denoting which validators have signed this aggregate signature
    validators_included: Bitmap<128>,
    /// BLS aggregated signature data
    aggregate_signature: MultiSignature<Bls12381G1Impl>
}

impl BLSSignature {
    /// Validates the wrapped signature by using our validator bitmap to pull the appropriate public keys out
    /// of a supplied public key library.
    pub fn validate(&self, public_key_library: &[PublicKey<Bls12381G1Impl>], data: &[u8]) -> bool {
        // Use our bitmap to create a vector of appropriate public keys
        
        let included_public_keys: Vec<PublicKey<Bls12381G1Impl>> = self.validators_included.clone()
            .into_iter().filter_map(|i| public_key_library.get(i)).cloned().collect();  // Might be inefficient but let's not prematurely optimize
        let multikey = MultiPublicKey::from_public_keys(included_public_keys);
        // We're swallowing the error here and might want to record it, but a fail is a fail
        self.aggregate_signature.verify(multikey, data).is_ok()
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
        let Ok(new_sig) = MultiSignature::from_signatures([self.as_signature(), other.as_signature()]) else {
            return false;
        };
        
        self.aggregate_signature = new_sig;
        self.validators_included = self.validators_included.bitor(other.validators_included);
        true
    }

    pub fn as_signature(&self) -> Signature<Bls12381G1Impl> {
        Signature::ProofOfPossession(self.aggregate_signature.as_raw_value().clone())
    }

    pub fn add_signature(&mut self, validator_id: BLSValidatorID, new_signature: Signature<Bls12381G1Impl>) -> bool {
        // If our validator is already included, just return false
        if self.validators_included.get(validator_id.into()) { return false; }
        // Otherwise note that we're included and aggregate the new signature
        let Ok(new_sig) = MultiSignature::from_signatures([self.as_signature(), new_signature]) else {
            return false;
        };
        self.validators_included.set(validator_id.into(), true);
        self.aggregate_signature = new_sig;
        true
    }

    pub fn to_bytes(&self) -> bool {
        true
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::default()
    }
}

impl Serialize for BLSSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        // We're just reusing our `encode` mechanism here for succinctness
        let mut buf = BytesMut::with_capacity(64);
        self.encode(&mut buf);
        serializer.serialize_bytes(&buf.freeze())

        // Serialize_struct way (maybe bad)
        // let mut state = serializer.serialize_struct("BLSSignature", 2)?;
        // state.serialize_field("validators", &self.validators_included.as_value().to_le_bytes())?;
        // state.serialize_field("sig", &self.aggregate_signature)?;
        // state.end()
    }
}

struct BLSSignatureVisitor {}
impl<'de> Visitor<'de> for BLSSignatureVisitor {
    type Value = BLSSignature;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("An n-wide byte array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {
        
        let mut bytes: Vec<u8> = Vec::with_capacity(64);
        loop {
            match seq.next_element() {
                Ok(Some(x)) => bytes.push(x),
                Ok(None) => break,
                Err(_e) => return Err(serde::de::Error::custom("Invalid content in sequence"))
            }
        }
        println!("{}", bytes.len());
        // Very gross skipping of the headers from the RLS encoding, we should not need to do this
        let (bitmap_slice, sig_slice) = bytes.split_at(17);
        let bitmap_bytes: Vec<u8> = bitmap_slice.iter().skip(1).cloned().collect();
        let sig_vec: Vec<u8> = sig_slice.iter().skip(1).cloned().collect();
        let sig_bytes: [u8;48] = sig_vec.try_into().map_err(|_| serde::de::Error::custom("Wrong number of bytes for signature"))?;
        // Parse out the u128 for out bitmap
        let bitmap_value = u128::from_le_bytes(bitmap_bytes.try_into().map_err(|_| serde::de::Error::custom("Error deserializing bitmap"))?);
        let validators_included = Bitmap::from_value(bitmap_value);
        // Parse the remaining bytes into the signature data
        let parsed_g1 = blsful::inner_types::G1Projective::from_compressed(&sig_bytes);
        if parsed_g1.is_none().into() {
            return Err(serde::de::Error::custom("Unable to reconstitute signature from compressed bytes"));
        }
        let aggregate_signature: MultiSignature<Bls12381G1Impl> = MultiSignature::ProofOfPossession(parsed_g1.unwrap());
        Ok(BLSSignature { validators_included, aggregate_signature })
    }

    fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {

        // let bytevec: Vec<u8> = v.iter().cloned().collect();
        Err(serde::de::Error::custom("butts"))
        // BLSSignature::decode(&bytevec).map_err(|e| serde::de::Error::custom(e.to_string()))
        // if 16 > v.len() {
        //     return Err(serde::de::Error::invalid_length(v.len(), &"A byte string at least 16 bytes long"));
        // }
        // let (bitmap_bytes, sig_bytes) = v.split_at(16);
        // // Parse out the u128 for out bitmap
        // let bitmap_value = u128::from_le_bytes(bitmap_bytes.try_into().map_err(serde::de::Error::custom)?);
        // let validators_included = Bitmap::from_value(bitmap_value);
        // // Parse the remaining bytes into the signature data
        // let aggregate_signature = Signature::from_serialized(sig_bytes)
        //     .map_err(|e| serde::de::Error::custom(format!("BLS Signature decode error: {:?}", e)))?;
        // Ok(BLSSignature { validators_included, aggregate_signature })
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
        // Because of the size of our bitmap, this is u128 (16 bytes).  Note that if we change
        // the size of the bitmap we might wind up changing the number of bytes written
        // and expected here!
        let tim: [u8;16] = self.validators_included.as_value().to_le_bytes();
        tim.encode(out);
        // A compressed G1 signature is 48 bytes
        self.aggregate_signature.as_raw_value().to_compressed().encode(out);
    }

    fn length(&self) -> usize { 64 }
}

impl Decodable for BLSSignature {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        //let v = alloy_rlp::Header::decode_bytes(buf, false)?.to_vec();
        // Pull both of our elements back out of the buf
        let bitmap_bytes: [u8;16] = <[u8;16]>::decode(buf)?;
        let sig_bytes: [u8;48] = <[u8;48]>::decode(buf)?;
        
        // Parse out the u128 for out bitmap
        let bitmap_value = u128::from_le_bytes(bitmap_bytes.try_into().map_err(|_| alloy_rlp::Error::Custom("Error deserializing bitmap"))?);
        let validators_included = Bitmap::from_value(bitmap_value);
        // Parse the remaining bytes into the signature data
        let parsed_g1 = blsful::inner_types::G1Projective::from_compressed(&sig_bytes);
        if parsed_g1.is_none().into() {
            return Err(alloy_rlp::Error::Custom("Unable to reconstitute signature from compressed bytes"));
        }
        let aggregate_signature: MultiSignature<Bls12381G1Impl> = MultiSignature::ProofOfPossession(parsed_g1.unwrap());
        Ok(BLSSignature { validators_included, aggregate_signature })
    }
}

#[cfg(test)]
mod tests {
    use blsful::SecretKey;
    use blsful::SignatureSchemes;
    use super::*;
    
    /// Create an aggregate BLSSignature over arbitrary data that uses specific keys out of a larger
    /// list of keys 
    fn sign_data_with(keys: &[SecretKey<Bls12381G1Impl>], idxs: &[usize], data: &[u8]) -> BLSSignature {
        let mut validators_included = Bitmap::new();
        let sigs: Vec<Signature<Bls12381G1Impl>> = idxs.iter().map(|i| {
            validators_included.set(*i, true);
            keys[*i].sign(SignatureSchemes::ProofOfPossession, data).unwrap()
        }).collect();
        let aggregate_signature = MultiSignature::from_signatures(&sigs).unwrap();
        BLSSignature { validators_included, aggregate_signature }
    }

    static FIXED_DATA: &str = "This is an arbitrary chunk of test data";

    #[test]
    fn will_sign_and_verify() {
        let mut sig = BLSSignature::default();
        let my_key = SecretKey::new();
        let my_signature = my_key.sign(SignatureSchemes::ProofOfPossession, &FIXED_DATA.as_bytes()).unwrap();
        sig.add_signature(0, my_signature);
        assert!(sig.validate(&[my_key.public_key()], &FIXED_DATA.as_bytes()), "Unable to validate signature");
    }

    #[test]
    fn will_serialize_and_deserialize() {
        let mut sig = BLSSignature::default();
        let my_key = SecretKey::new();
        let my_signature = my_key.sign(SignatureSchemes::ProofOfPossession, &FIXED_DATA.as_bytes()).unwrap();
        sig.add_signature(0, my_signature);
        // Serialize the structure
        let serialized = serde_json::to_string(&sig).unwrap();
        println!("{}", serialized);
        // Deserialize the structure
        let deserialized: BLSSignature = serde_json::from_str(&serialized).unwrap();
        // Compare that both the before and after are the same
        assert_eq!(sig.validators_included, deserialized.validators_included, "Aggregate signatures don't match");
        assert_eq!(sig.aggregate_signature, deserialized.aggregate_signature, "Aggregate signatures don't match");
        assert!(deserialized.validate(&[my_key.public_key()], &FIXED_DATA.as_bytes()), "Rehydrated signature does not validate")
    }

    #[test]
    fn will_do_rlp_encoding() {
        // Build and validate our sig
        let mut sig = BLSSignature::default();
        let my_key = SecretKey::new();
        let my_signature = my_key.sign(SignatureSchemes::ProofOfPossession, &FIXED_DATA.as_bytes()).unwrap();
        sig.add_signature(0, my_signature);
        assert!(sig.validate(&[my_key.public_key()], &FIXED_DATA.as_bytes()));
        // Encode and decode into a new sig
        let mut buf = BytesMut::with_capacity(64);
        sig.encode(&mut buf);
        let final_bytes = buf.freeze();
        let bytevec: Vec<u8> = final_bytes.iter().cloned().collect();
        let newsig = BLSSignature::decode(&mut bytevec.as_slice()).unwrap();
        // Validate the reconstituted sig
        assert!(newsig.validate(&[my_key.public_key()], &FIXED_DATA.as_bytes()));
    }

    #[test]
    fn will_perform_simple_aggregation() {
        let keys: Vec<SecretKey<Bls12381G1Impl>> = (0..10).map(|_| SecretKey::new()).collect();
        let pubkeys: Vec<PublicKey<Bls12381G1Impl>> = keys.iter().map(|x| x.public_key()).collect();
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