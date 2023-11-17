pub trait Side: Ord {
    fn is_bid(&self) -> bool;

    fn is_ask(&self) -> bool {
        !self.is_bid()
    }
}
