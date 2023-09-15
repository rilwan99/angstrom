use crate::on_chain::Signature;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Proposal {
    proposal_type: ProposalType,
    height:        u64,
    round:         u64,
    pol_round:     u64,
    // TODO: move to type
    timestamp:     u128,
    signature:     Signature
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProposalType {
    Proposal = 32
}
