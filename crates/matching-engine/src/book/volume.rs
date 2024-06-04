use std::cell::Cell;
use malachite::num::basic::traits::Zero;
use crate::cfmm::uniswap::{MarketPrice, SqrtPriceX96};
use super::{order::{Order, OrderDirection, OrderOutcome}, OrderBook, OrderPrice, OrderVolume};

#[derive(Clone)]
enum PartialOrder<'a> {
    Bid(Order<'a>),
    Ask(Order<'a>)
}

#[derive(Clone)]
pub struct VolumeFillBookSolver<'a> {
    book: &'a OrderBook<'a>,
    bid_idx: Cell<usize>,
    pub bid_outcomes: Vec<OrderOutcome>,
    ask_idx: Cell<usize>,
    ask_outcomes: Vec<OrderOutcome>,
    amm_price: MarketPrice<'a>,
    current_partial: Option<PartialOrder<'a>>,
    // A checkpoint should never have a checkpoint stored within itself, otherwise this gets gnarly
    checkpoint: Option<Box<Self>>
}

impl<'a> VolumeFillBookSolver<'a> {
    pub fn new(book: &'a OrderBook) -> Self {
        // Create our outcome tracking for our orders
        let bid_outcomes = vec![OrderOutcome::Unfilled; book.bids.len()];
        let ask_outcomes = vec![OrderOutcome::Unfilled; book.asks.len()];
        let amm_price = book.amm.current_position();
        let mut new_element = Self {
            book,
            bid_idx: Cell::new(0),
            bid_outcomes,
            ask_idx: Cell::new(0), 
            ask_outcomes,
            amm_price,
            current_partial: None,
            checkpoint: None
        };
        // We can checkpoint our initial state as valid
        new_element.save_checkpoint();
        new_element
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
        self.checkpoint.as_ref().map(|cp| *cp.clone())
    }

    /// Restore our checkpoint into this VolumeFillBookSolver - not sure if we ever want to do this but we can!
    #[allow(dead_code)]
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

    pub fn fill(&mut self) -> (Option<OrderPrice>, OrderVolume) {
        {
            // Total quantity filled
            let mut q = f64::ZERO;
            // Price found
            let mut p: Option<OrderPrice> = None;
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
        // Find our next unfilled order
        while index < outcomes.len() {
            if let OrderOutcome::Unfilled = outcomes[index] { break; }
            index += 1;
        }

        // See if we have an AMM order that takes precedence over our book order
        let amm_order = match direction {
            OrderDirection::Bid => {
                order_book.get(index).and_then(|o| if amm_price.as_float() > o.price() { println!("Prioritizing AMM"); Some(Some(o.price())) } else { Some(None) })
                .and_then(|o| o.map(|p| amm_price.sell_to_price(SqrtPriceX96::from_float_price(p))))
                .unwrap_or_else(|| amm_price.sell_to_next_bound())
            },
            OrderDirection::Ask => {
                order_book.get(index).and_then(|o| if amm_price.as_float() < o.price() { Some(Some(o.price())) } else { Some(None) })
                .and_then(|o| o.map(|p| amm_price.buy_to_price(SqrtPriceX96::from_float_price(p))))
                .unwrap_or_else(|| amm_price.buy_to_next_bound())
            }
        }.map(|r| Order::AMM(r));
        
        // If we don't have an AMM order, point our index at the new book order we care about
        // if amm_order.is_none() { println!("It's none!"); index_cell.set(index); }
        // return amm_order;
            
        
        match order_book.get(index).map(|o| o.price()) {
            Some(p) if p >= amm_price.as_float() => {
                println!("It's none!");
                index_cell.set(index);
                None
            },
            e @ _ => {
                if let Some(p) = e {
                    let amm_better = match direction {
                        OrderDirection::Bid => p >= amm_price.as_float(),
                        OrderDirection::Ask => p <= amm_price.as_float()
                    };
                    if amm_better {
                        index_cell.set(index);
                        return None;
                    }
                }
                let amm_order = match direction {
                    OrderDirection::Bid => {
                        e.and_then(|p| amm_price.sell_to_price(SqrtPriceX96::from_float_price(p)))
                            .or_else(|| amm_price.sell_to_next_bound())?
                    },
                    OrderDirection::Ask => {
                        e.and_then(|p| amm_price.buy_to_price(SqrtPriceX96::from_float_price(p)))
                            .or_else(|| amm_price.buy_to_next_bound())?
                    }
                };
                Some(Order::AMM(amm_order))
            },
        }
    }
}