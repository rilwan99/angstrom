use bytes::{Bytes, BytesMut};
use ethers_core::{
    types::{Address, U256},
    utils::keccak256
};

use super::OrderDetails;

pub type ExternalStateCall = (Address, Bytes);

#[derive(Debug)]
pub struct ExternalStateSim {
    pub tx:               OrderDetails,
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
        let addr = Address::from_slice(&self.pre_hook[0..19]);

        (addr, Bytes::copy_from_slice(&self.pre_hook[20..]))
    }

    pub fn post_hook(&self) -> ExternalStateCall {
        let addr = Address::from_slice(&self.post_hook[0..19]);

        (addr, Bytes::copy_from_slice(&self.post_hook[20..]))
    }
}

impl TryInto<ExternalStateSim> for OrderDetails {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<ExternalStateSim, Self::Error> {
        let mut msg = Vec::new();
        msg.extend(SEARCHER_TYPE_HASH.to_fixed_bytes());
        msg.extend(self.order.pool);
        msg.extend(self.order.token_in.to_fixed_bytes());
        msg.extend(self.order.token_out.to_fixed_bytes());
        msg.extend(self.order.amount_in.to_be_bytes());
        msg.extend(self.order.amount_out_min.to_be_bytes());

        let mut deadbuf = BytesMut::new();
        self.order.deadline.to_big_endian(&mut deadbuf);
        msg.extend(deadbuf.to_vec());
        let mut bribe = BytesMut::new();
        self.order.bribe.to_big_endian(&mut bribe);
        msg.extend(bribe.to_vec());
        msg.extend(keccak256(&self.order.pre_hook));
        msg.extend(keccak256(&self.order.post_hook));

        let digest = keccak256(msg);
        let addr = self.signature.recover(digest)?;

        Ok(ExternalStateSim {
            tx: ::SearcherOrUser::Searcher(self.clone()),
            pre_hook: self.order.pre_hook,
            amount_in_req: self.order.amount_in,
            amount_in_token: self.order.token_in,
            post_hook: self.order.post_hook,
            amount_out_lim: self.order.amount_out_min,
            amount_out_token: self.order.token_out,
            addr
        })
    }
}
