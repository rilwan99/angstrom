use crate::matcher::VolumeFillBookSolver;
use super::MatchingStrategy;

/// Very simple strategy where we just roll the solver back to the last "good solve"
/// checkpoint and presume we're done there.
pub struct SimpleCheckpointStrategy {}

impl<'a> MatchingStrategy<'a> for SimpleCheckpointStrategy {
    fn finalize(solver: VolumeFillBookSolver) -> Option<VolumeFillBookSolver> {
        solver.from_checkpoint()
    }
}