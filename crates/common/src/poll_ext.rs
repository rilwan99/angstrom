use std::task::Poll;

pub trait PollExt<T> {
    /// Analogous to filter on [`Option`].
    /// ```ignore
    /// if Poll::Ready(T) && predicate(&T) { return Poll::Ready(T) };
    /// if Poll::Ready(T) && !predicate(&T) { return Poll::Pending };
    /// if Poll::Pending { return Poll::Pending };
    /// ```
    fn filter(self, predicate: impl FnMut(&T) -> bool) -> Poll<T>;

    /// Application of a filter plus a map. Acts exactly like filter_map on a
    /// iterator.
    fn filter_map<U>(self, predicate: impl FnMut(T) -> Option<U>) -> Poll<U>;

    /// Applies the given function as a end of this functional branch and
    /// returns true if the function was called
    fn apply(self, predicate: impl FnMut(T)) -> bool;
}

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

    fn apply(self, mut predicate: impl FnMut(T)) -> bool {
        let Poll::Ready(value) = self else { return false };
        predicate(value);

        return true
    }
}
