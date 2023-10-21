use bytes::{Bytes, BytesMut};
use ethers_core::{types::Address, utils::keccak256};
use reth_rlp::Encodable;

use super::SubmittedOrder;

pub type ExternalStateCall = (Address, Bytes);

#[derive(Debug)]
pub struct ExternalStateSim {
    pub tx:               SubmittedOrder,
    // the address of the user.
    pub addr:             Address,
    // gas in
    pub pre_hook:         Bytes,
    pub amount_in_req:    u128,
    pub amount_in_token:  Address,
    // gas out
    pub post_hook:        Bytes,
    pub amount_out_lim:   u128,
    pub amount_out_token: Address
}

impl ExternalStateSim {
    pub fn pre_hook(&self) -> ExternalStateCall {
        let addr = Address::from_slice(&self.pre_hook[0..20]);

        (addr, Bytes::copy_from_slice(&self.pre_hook[21..]))
    }

    pub fn post_hook(&self) -> ExternalStateCall {
        let addr = Address::from_slice(&self.post_hook[0..20]);

        (addr, Bytes::copy_from_slice(&self.post_hook[21..]))
    }
}

impl TryInto<ExternalStateSim> for SubmittedOrder {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<ExternalStateSim, Self::Error> {
        let addr = self.get_ethereum_address();

        Ok(ExternalStateSim {
            tx: self,
            pre_hook: self.order.pre_hook,
            amount_in_req: self.order.amount_in,
            amount_in_token: self.order.currency_in,
            post_hook: self.order.post_hook,
            amount_out_lim: self.order.amount_out_min,
            amount_out_token: self.order.currency_out,
            addr
        })
    }
}
