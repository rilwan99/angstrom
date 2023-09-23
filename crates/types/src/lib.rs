use reth_primitives::H512;

pub mod consensus;
pub mod contract_bindings;
pub mod database;
pub mod on_chain;

pub fn validate_signature(signature: Signature, message: H256, public_key: H512) -> bool {
    let signature: [u8; 64] = signature.clone().try_into().unwrap();
    let rec_id: [u8; 4] = signature.recovery_id().unwrap().try_into().unwrap();

    let rec_id = i32::from_be_bytes(rec_id);
    let sig = RecoverableSignature::from_compact(&signature, RecoveryId::from_i32(rec_id).unwrap())
        .unwrap();

    let msg: Message = Message::from_slice(message).unwrap();
    let recovered_pub_key = sig.recover(&msg).unwrap();
    let pub_key: [u8; 64] = recovered_pub_key.serialize_uncompressed()[1..]
        .try_into()
        .unwrap();

    H512::from_slice(&pub_key) == pubic_key
}

pub fn get_public_key(signature: Signature, message: H256) -> H512 {
    let signature: [u8; 64] = signature.clone().try_into().unwrap();
    let rec_id: [u8; 4] = signature.recovery_id().unwrap().try_into().unwrap();

    let rec_id = i32::from_be_bytes(rec_id);
    let sig = RecoverableSignature::from_compact(&signature, RecoveryId::from_i32(rec_id).unwrap())
        .unwrap();

    let msg: Message = Message::from_slice(message).unwrap();
    let recovered_pub_key = sig.recover(&msg).unwrap();
    let pub_key: [u8; 64] = recovered_pub_key.serialize_uncompressed()[1..]
        .try_into()
        .unwrap();

    H512::from_slice(&pub_key)
}
