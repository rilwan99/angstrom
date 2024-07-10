use std::collections::HashSet;

use super::{order::OrderCoordinate, BookID};

#[derive(Clone, Debug)]
pub struct XPoolOutcomes {
    live: Vec<OrderCoordinate>,
    dead: Vec<OrderCoordinate>
}

impl XPoolOutcomes {
    pub fn new(live: Vec<OrderCoordinate>, dead: Vec<OrderCoordinate>) -> Self {
        Self { live, dead }
    }

    pub fn live(&self) -> &Vec<OrderCoordinate> {
        &self.live
    }

    pub fn dead(&self) -> &Vec<OrderCoordinate> {
        &self.dead
    }

    pub fn valid_books(&self) -> HashSet<u128> {
        self.live
            .iter()
            .chain(self.dead.iter())
            .map(|c| c.book)
            .collect()
    }

    pub fn for_book(&self, book_id: BookID) -> Self {
        let live = self
            .live
            .iter()
            .filter(|x| x.book == book_id)
            .cloned()
            .collect();
        let dead = self
            .dead
            .iter()
            .filter(|x| x.book == book_id)
            .cloned()
            .collect();
        Self { live, dead }
    }
}