/// Matching strategies are different ways we can look at an order Book and determine
/// what the best possible order matching outome is.  As per our conversations on this
/// topic, there are a lot of different heuristics and algorithms we can use to determine
/// our final clearing price and the orders that will be filled at that price.
/// 
/// The intent is to implement several different strategies here and compare them via
/// a suite of tests that will help us determine what the optimal matching strategy
/// could be.

use crate::book::volume::VolumeFillBookSolver;

mod simplecheckpoint;

/// Basic trait to describe a matching strategy
pub trait MatchingStrategy {
    fn finalize(solver: VolumeFillBookSolver) -> Option<VolumeFillBookSolver>;
}