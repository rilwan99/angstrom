#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderPriorityData {
    pub price:  u128,
    pub volume: u128,
    pub gas:    u128
}

impl PartialOrd for OrderPriorityData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.price.cmp(&other.price).then_with(|| {
            self.volume
                .cmp(&other.volume)
                .then_with(|| self.gas.cmp(&other.gas))
        }))
    }
}

impl Ord for OrderPriorityData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
