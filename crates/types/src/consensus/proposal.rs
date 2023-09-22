use super::Time;
use crate::on_chain::Signature;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Proposal {
    pub height:        u64,
    pub round:         u64,
    pub timestamp:     Time,
    pub signature:     Signature
}

