use std::cell::Cell;

use malachite::num::basic::traits::Zero;
use crate::cfmm::uniswap::{MarketPrice, MarketSnapshot, SqrtPriceX96};
use super::order::{OrderOutcome, Order};

pub struct VolumeFillBook<'a> {
    amm: MarketSnapshot,
    bids: Vec<Order<'a>>,
    asks: Vec<Order<'a>>
}

impl<'a> VolumeFillBook<'a> {
    pub fn new(amm: MarketSnapshot, mut bids: Vec<Order<'a>>, mut asks: Vec<Order<'a>>) -> Self {
        // Sort by price and then by volume - highest price first, highest volume first for same price
        bids.sort_by(|a, b| b.price().partial_cmp(&a.price()).unwrap_or(std::cmp::Ordering::Less)
            .then(b.quantity(0.0).partial_cmp(&a.quantity(0.0)).unwrap_or(std::cmp::Ordering::Less)));
        // Sort by price and then by volume - lowest price first, highest volume first for same price
        asks.sort_by(|a, b| a.price().partial_cmp(&b.price()).unwrap_or(std::cmp::Ordering::Less)
            .then(b.quantity(0.0).partial_cmp(&a.quantity(0.0)).unwrap_or(std::cmp::Ordering::Less)));
        Self { amm, bids, asks }
    }
}

enum OrderDirection {
    Bid,
    Ask
}

#[derive(Clone)]
enum PartialOrder<'a> {
    Bid(Order<'a>),
    Ask(Order<'a>)
}

#[derive(Clone)]
pub struct VolumeFillBookSolver<'a> {
    book: &'a VolumeFillBook<'a>,
    bid_idx: Cell<usize>,
    bid_outcomes: Vec<OrderOutcome>,
    ask_idx: Cell<usize>,
    ask_outcomes: Vec<OrderOutcome>,
    amm_price: MarketPrice<'a>,
    current_partial: Option<PartialOrder<'a>>,
    // A checkpoint should never have a checkpoint stored within itself, otherwise this gets gnarly
    checkpoint: Option<Box<Self>>
}

impl<'a> VolumeFillBookSolver<'a> {
    pub fn new(book: &'a VolumeFillBook) -> Self {
        // Create our outcome tracking for our orders
        let bid_outcomes = vec![OrderOutcome::Unfilled; book.bids.len()];
        let ask_outcomes = vec![OrderOutcome::Unfilled; book.asks.len()];
        let amm_price = book.amm.current_position();
        Self {
            book,
            bid_idx: Cell::new(0),
            bid_outcomes,
            ask_idx: Cell::new(0), 
            ask_outcomes,
            amm_price,
            current_partial: None,
            checkpoint: None
        }
    }

    /// Save our current solve state to an internal checkpoint
    fn save_checkpoint(&mut self) {
        let checkpoint = Self {
            book: self.book,
            bid_idx: self.bid_idx.clone(),
            bid_outcomes: self.bid_outcomes.clone(),
            ask_idx: self.ask_idx.clone(), 
            ask_outcomes: self.ask_outcomes.clone(),
            amm_price: self.amm_price.clone(),
            current_partial: self.current_partial.clone(),
            checkpoint: None
        };
        self.checkpoint = Some(Box::new(checkpoint));
    }

    /// Spawn a new VolumeFillBookSolver from our checkpoint
    pub fn from_checkpoint(&self) -> Option<Self> {
        let Some(cp) = self.checkpoint.as_ref() else { return None; };
        Some(Self {
            book: cp.book,
            bid_idx: cp.bid_idx.clone(),
            bid_outcomes: cp.bid_outcomes.clone(),
            ask_idx: cp.ask_idx.clone(), 
            ask_outcomes: cp.ask_outcomes.clone(),
            amm_price: cp.amm_price.clone(),
            current_partial: cp.current_partial.clone(),
            checkpoint: None
        })
    }

    /// Restore our checkpoint into this VolumeFillBookSolver - not sure if we ever want to do this but we can!
    fn restore_checkpoint(&mut self) -> bool {
        let Some(checkpoint) = self.checkpoint.take() else { return false; };
        let Self { bid_idx, bid_outcomes, ask_idx, ask_outcomes, amm_price, current_partial, ..} = *checkpoint;
        self.bid_idx = bid_idx;
        self.bid_outcomes = bid_outcomes;
        self.ask_idx = ask_idx;
        self.ask_outcomes = ask_outcomes;
        self.amm_price = amm_price;
        self.current_partial = current_partial;
        true
    }

    pub fn fill(&mut self) -> (Option<f64>, f64) {
        {
            // Total quantity filled
            let mut q = f64::ZERO;
            // Price found
            let mut p: Option<f64> = None;
            // Local temporary storage for bid and ask AMM orders since they're generated on the fly
            // and need somewhere to live while we use them
            let mut amm_bid_order: Option<Order>;
            let mut amm_ask_order: Option<Order>;

            loop {
                let bid = if let Some(PartialOrder::Bid(ref o)) = self.current_partial {
                    o
                } else {
                    amm_bid_order = self.try_next_order(OrderDirection::Bid);
                    if let Some(o) = amm_bid_order.as_ref().or_else(|| self.book.bids.get(self.bid_idx.get())) {
                        o
                    } else { break; } // Break if there are no more valid bid orders to work with
                };
                let ask = if let Some(PartialOrder::Ask(ref o)) = self.current_partial {
                    o
                } else { 
                    amm_ask_order = self.try_next_order(OrderDirection::Ask);
                    if let Some(o) = amm_ask_order.as_ref().or_else(|| self.book.asks.get(self.ask_idx.get())) {
                        o
                    } else { break; } // Break if there are no more valid ask orders to work with
                };

                // If we're talking to the AMM on both sides, we're done
                if let (Order::AMM(_), Order::AMM(_)) = (bid, ask) { break; }

                // If our prices no longer cross, we're done
                if ask.price() > bid.price() { break; }

                // Limit to price so that AMM orders will only offer the quantity they can
                // profitably sell.  (Non-AMM orders ignore the provided price)
                let ask_q = ask.quantity(bid.price());
                let bid_q = bid.quantity(ask.price());

                // If either quantity is zero maybe we should break here? (could be a replacement for
                // price cross checking if we implement that)
                if ask_q == f64::ZERO || bid_q == f64::ZERO { break; }

                let matched = ask_q.min(bid_q);
                q += matched;
                let excess = bid_q - ask_q;

                // No matter what happens, we always update our AMM orders with any quantity sold
                if let Order::AMM(o) = ask {
                    // We always update our AMM price with any quantity sold
                    let final_amm_order = o.fill(matched);
                    self.amm_price = final_amm_order.end_bound.clone();
                }
                if let Order::AMM(o) = bid {
                    let final_amm_order = o.fill(matched);
                    self.amm_price = final_amm_order.end_bound.clone();
                }

                // Then we see what else we need to do
                if excess == f64::ZERO {
                    // We annihilated
                    p = Some((ask.price() + bid.price()) / 2.0);
                    // Mark as filled if non-AMM order
                    if let Order::KillOrFill(_) | Order::PartialFill(_) = ask {
                        self.ask_outcomes[self.ask_idx.get()] = OrderOutcome::CompleteFill
                    }
                    if let Order::KillOrFill(_) | Order::PartialFill(_) = bid {
                        self.bid_outcomes[self.bid_idx.get()] = OrderOutcome::CompleteFill
                    }
                    self.current_partial = None;
                    // Take a snapshot as a good solve state
                    self.save_checkpoint();
                    // We're done here, we'll get our next bid and ask on the next round
                } else if excess > f64::ZERO {
                    p = Some(bid.price());
                    // Ask was completely filled, remainder bid
                    if let Order::KillOrFill(_) | Order::PartialFill(_) = ask {
                        self.ask_outcomes[self.ask_idx.get()] = OrderOutcome::CompleteFill
                    }
                    // Create and save our partial bid
                    if let Order::KillOrFill(_) | Order::PartialFill(_) = bid {
                        self.bid_outcomes[self.bid_idx.get()] = self.bid_outcomes[self.bid_idx.get()].partial_fill(matched);
                        self.current_partial = Some(PartialOrder::Bid(bid.fill(ask_q)));
                    } else {
                        self.current_partial = None;
                    }
                } else {
                    p = Some(ask.price());
                    // Bid was completely filled, remainder ask
                    if let Order::KillOrFill(_) | Order::PartialFill(_) = bid {
                        self.bid_outcomes[self.bid_idx.get()] = OrderOutcome::CompleteFill
                    }
                    // Create and save our parital ask
                    if let Order::KillOrFill(_) | Order::PartialFill(_) = ask {
                        self.ask_outcomes[self.ask_idx.get()] = self.ask_outcomes[self.ask_idx.get()].partial_fill(matched);
                        self.current_partial = Some(PartialOrder::Ask(ask.fill(bid_q)));
                    } else {
                        self.current_partial = None;
                    }
                }
                // We can checkpoint if we annihilated (No partial), if we completely filled an order with an AMM order (No partial)
                // or if we have an incomplete order but it's a Partial Fill order which means this is a valid state to stop
                if let None | Some(PartialOrder::Ask(Order::PartialFill(_))) | Some(PartialOrder::Bid(Order::PartialFill(_))) = self.current_partial {
                    self.save_checkpoint();
                }
            }
            (p, q)
        }
    }

    fn try_next_order(&self, direction: OrderDirection) -> Option<Order<'a>> {
        let (index_cell, order_book, outcomes) = match direction {
            OrderDirection::Bid => (&self.bid_idx, &self.book.bids, &self.bid_outcomes),
            OrderDirection::Ask => (&self.ask_idx, &self.book.asks, &self.ask_outcomes)
        };
        let mut index = index_cell.get();
        // Use the price we're passed in or default to the current checkpoint price
        let amm_price = &self.amm_price;
        while let Some(OrderOutcome::Killed) = outcomes.get(index + 1) {
            index += 1;
        }
        match order_book.get(index + 1).map(|o| o.price()) {
            Some(p) if p >= amm_price.as_float() => {
                index_cell.set(index + 1);
                None
            },
            // We do have a target price in our order book to match to
            Some(p) => {
                let target_price = SqrtPriceX96::from_float_price(p);
                match direction {
                    OrderDirection::Bid => Some(Order::AMM(amm_price.sell_to_price(target_price)?)),
                    OrderDirection::Ask => Some(Order::AMM(amm_price.buy_to_price(target_price)?))
                }
            },
            // There's no target price in our order book so just produce an AMM order to the next AMM volume bound
            _ => {
                match direction {
                    OrderDirection::Bid => Some(Order::AMM(amm_price.sell_to_next_bound()?)),
                    OrderDirection::Ask => Some(Order::AMM(amm_price.buy_to_next_bound()?))
                }
            }
        }
    }
}