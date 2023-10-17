mod guard;
mod sources;
mod submission_server;

use std::task::Poll;

pub use guard::*;
pub use sources::*;
pub use submission_server::SubmissionServerConfig;

/// COPY from internal sorella tooling
pub trait PollExt<T> {
    /// Analogous to filter on [`Option`].
    /// ```rust
    /// if Poll::Ready(T) && predicate(&T) { return Poll::Ready(T) };
    /// if Poll::Ready(T) && !predicate(&T) { return Poll::Pending };
    /// if Poll::Pending { return Poll::Pending };
    /// ```
    fn filter(self, predicate: impl FnMut(&T) -> bool) -> Poll<T>;

    /// Application of a filter plus a map. Acts exactly like filter_map on a
    /// iterator.
    fn filter_map<U>(self, predicate: impl FnMut(T) -> Option<U>) -> Poll<U>;
}

/// COPY from internal sorella tooling
impl<T> PollExt<T> for Poll<T> {
    fn filter(self, mut predicate: impl FnMut(&T) -> bool) -> Poll<T> {
        let Poll::Ready(val) = self else { return Poll::Pending };

        if predicate(&val) {
            Poll::Ready(val)
        } else {
            Poll::Pending
        }
    }

    fn filter_map<U>(self, mut predicate: impl FnMut(T) -> Option<U>) -> Poll<U> {
        let Poll::Ready(val) = self else { return Poll::Pending };

        if let Some(map) = predicate(val) {
            Poll::Ready(map)
        } else {
            Poll::Pending
        }
    }
}

/// useful for when dealing with [`std::task::Poll`]. This
/// generates the simple return based on a function on a value.
/// This Macro
/// ```rust
/// return_if!(
///     self
///         .guard_net
///         .poll_next_unpin(cx)
///         .filter_map(|poll| poll)
///         .map(|event| Some(SourceMessages::Swarm(event))) => is_ready()
/// );
/// ```
///
/// Gets transformed into this
///
/// ```rust
/// let res = self
///     .guard_net
///     .poll_next_unpin(cx)
///     .filter_map(|poll| poll)
///     .map(|event| Some(SourceMessages::Swarm(event)));
///
/// if res.is_ready() {
///     return res
/// }
/// ```
#[macro_export]
macro_rules! return_if {
    ($value:expr => $($value_expr:tt)*) => {
        let res = $value;
        if res.$($value_expr)* {
            return res
        }
    };
}
