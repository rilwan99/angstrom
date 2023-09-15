use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Vote {
    /// Type of vote (prevote or precommit)
    pub vote_type: Type,

    /// Block height
    pub height: block::Height,

    /// Round
    pub round: block::Round,

    /// Block ID
    pub block_id: Option<block::Id>,

    /// Timestamp
    pub timestamp: Option<Time>,

    /// Validator address
    pub validator_address: account::Id,

    /// Validator index
    pub validator_index: ValidatorIndex,

    /// Signature
    pub signature: Option<Signature>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CanonicalVote {
    /// Type of vote (prevote or precommit)
    pub vote_type: super::Type,

    /// Block height
    pub height: block::Height,

    /// Round
    pub round: block::Round,

    /// Block ID
    //#[serde(deserialize_with = "serializers::parse_non_empty_block_id")] - moved to try_from
    pub block_id: Option<block::Id>,

    /// Timestamp
    pub timestamp: Option<Time>,

    /// Chain ID
    pub chain_id: ChainId
}

/// SignedVote is the union of a canonicalized vote, the signature on
/// the sign bytes of that vote and the id of the validator who signed it.
pub struct SignedVote {
    vote:              CanonicalVote,
    validator_address: account::Id,
    signature:         Signature
}

impl SignedVote {
    /// Create new `SignedVote` from provided canonicalized vote, validator id,
    /// and the signature of that validator.
    pub fn new(
        vote: Vote,
        chain_id: ChainId,
        validator_address: account::Id,
        signature: Signature
    ) -> SignedVote {
        let canonical_vote = CanonicalVote::new(vote, chain_id);
        SignedVote { vote: canonical_vote, signature, validator_address }
    }

    /// Create a new `SignedVote` from the provided `Vote`, which may or may not
    /// be signed. If the vote is not signed, this function will return
    /// `None`.
    pub fn from_vote(vote: Vote, chain_id: ChainId) -> Option<Self> {
        let validator_address = vote.validator_address;
        vote.signature
            .clone()
            .map(|signature| Self::new(vote, chain_id, validator_address, signature))
    }

    /// Return the id of the validator that signed this vote.
    pub fn validator_id(&self) -> account::Id {
        self.validator_address
    }

    /// Return the bytes (of the canonicalized vote) that were signed.
    pub fn sign_bytes(&self) -> Vec<u8> {
        Protobuf::<RawCanonicalVote>::encode_length_delimited_vec(self.vote.clone())
    }

    /// Return the actual signature on the canonicalized vote.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

/// Types of votes
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// Votes for bundle which validators observe are valid for a given round
    Prevote   = 1,

    /// Votes to commit to a particular bundle for a given round
    Precommit = 2
}

impl TryFrom<i32> for Type {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Type::Prevote),
            2 => Ok(Type::Precommit),
            _ => Err(Error::invalid_message_type())
        }
    }
}

impl From<Type> for i32 {
    fn from(value: Type) -> Self {
        value as i32
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match self {
            Type::Prevote => "Prevote",
            Type::Precommit => "Precommit"
        };
        write!(f, "{id}")
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Prevote" => Ok(Self::Prevote),
            "Precommit" => Ok(Self::Precommit),
            _ => Err(Error::invalid_message_type())
        }
    }
}
