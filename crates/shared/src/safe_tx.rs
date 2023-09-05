use bytes::Bytes;
use ethers_core::types::{Address, H256, U256};
use reth_primitives::bytes::BytesMut;
use reth_rlp::{Encodable, RlpDecodable, RlpEncodable};
use revm::primitives::{TransactTo, TxEnv};
use serde::{Deserialize, Serialize};

use crate::ANGSTROM_CONTRACT_ADDR;

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct SafeTx {
    pub value:           U256,
    pub calldata:        Bytes,
    /// 0 for call , 1 for delegate call
    pub operation:       u8,
    pub safe_tx_gas:     U256,
    pub base_gas:        U256,
    pub gas_price:       U256,
    pub gas_token:       Address,
    pub refund_receiver: Address,
    pub nonce:           U256
}

impl SafeTx {
    fn encode_tx_data(&self) -> Bytes {
        //   bytes32 safeTxHash = keccak256(
        //     abi.encode(
        //         SAFE_TX_TYPEHASH,
        //         to,
        //         value,
        //         keccak256(data),
        //         operation,
        //         safeTxGas,
        //         baseGas,
        //         gasPrice,
        //         gasToken,
        //         refundReceiver,
        //         _nonce
        //     )
        // );
        // return abi.encodePacked(bytes1(0x19), bytes1(0x01), domainSeparator(),
        // safeTxHash);
        todo!()
    }

    pub fn tx_hash(&self) -> H256 {
        H256(ethers_core::utils::keccak256(self.encode_tx_data()))
    }
}

impl Into<TxEnv> for SafeTx {
    fn into(self) -> TxEnv {
        let mut data = BytesMut::new();
        self.encode(&mut data);

        TxEnv {
            caller:           Default::default(),
            gas_limit:        self.safe_tx_gas.as_u64(),
            gas_price:        self.gas_price.into(),
            gas_priority_fee: None,
            transact_to:      TransactTo::Call(ANGSTROM_CONTRACT_ADDR.into()),
            value:            self.value.into(),
            data:             data.to_vec().into(),
            chain_id:         Some(1),
            nonce:            None,
            access_list:      Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct SimmedSafeTx {
    pub raw:      SafeTx,
    pub gas_used: U256
}
