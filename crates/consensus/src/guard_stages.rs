use std::collections::HashMap;

use guard_types::consensus::{RoundStep, Time};
use reth_primitives::H512;

pub struct GuardStages {
    stages: HashMap<H512, GuardRoundState>
}

impl GuardStages {
    pub fn new_step(&mut self, guard: H512, step: RoundStep) {
        let entry = self.stages.entry(guard).or_default();
        if entry.round != step.round {
            entry.round = step.round;
            entry.proposal = false;
        }

        if entry.height != step.height {
            entry.height = step.height;
            entry.proposal = false;
        }

        entry.round_step = step;
    }

    pub fn new_round(&mut self, guard: H512, round: u64) {
        self.stages.entry(guard).or_default().round = round;
    }

    pub fn new_height(&mut self, guard: H512, height: u64) {
        self.stages.entry(guard).or_default().height = height;
    }

    pub fn new_proposal(&mut self, guard: H512, proposal: bool) {
        self.stages.entry(guard).or_default().proposal = proposal;
    }

    pub fn new_start_time(&mut self, guard: H512, time: Time) {
        self.stages.entry(guard).or_default().start_time = time;
    }
}

// Height int64         `json:"height"` // Height peer is at
// 	Round  int32         `json:"round"`  // Round peer is at, -1 if unknown.
// 	Step   RoundStepType `json:"step"`   // Step peer is at
//
// 	// Estimated start of round 0 at this height
// 	StartTime time.Time `json:"start_time"`
//
// 	// True if peer has proposal for this round
// 	Proposal                   bool                `json:"proposal"`
// 	ProposalBlockPartSetHeader types.PartSetHeader
// `json:"proposal_block_part_set_header"` 	ProposalBlockParts
// *bits.BitArray      `json:"proposal_block_parts"` 	// Proposal's POL round. -1
// if none. 	ProposalPOLRound int32 `json:"proposal_pol_round"`
//
// 	// nil until ProposalPOLMessage received.
// 	ProposalPOL     *bits.BitArray `json:"proposal_pol"`
// 	Prevotes        *bits.BitArray `json:"prevotes"`          // All votes peer
// has for this round 	Precommits      *bits.BitArray `json:"precommits"`
// // All precommits peer has for this round 	LastCommitRound int32
// `json:"last_commit_round"` // Round of commit for last height. -1 if none.
// 	LastCommit      *bits.BitArray `json:"last_commit"`       // All commit
// precommits of commit for last height.
//
// 	// Round that we have commit for. Not necessarily unique. -1 if none.
// 	CatchupCommitRound int32 `json:"catchup_commit_round"`
//
// 	// All commit precommits peer has for this height & CatchupCommitRound
// 	CatchupCommit *bits.BitArray `json:"catchup_commit"`
/// fields are currently left out (see above comment)
#[derive(Debug, Default)]
pub struct GuardRoundState {
    pub height:     u64,
    pub round:      u64,
    pub round_step: RoundStep,
    pub start_time: Time,
    pub proposal:   bool
}
