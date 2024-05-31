/// This file contains some things I wrote that I don't want to throw away quite yet but that
/// I don't think has a home really anywhere.  Please ignore it for now, this should not be in
/// any final commit or merge

/// Used in MarketSnapshotIter
enum OrderDirection {
    Bid,
    Ask
}


/// Seemed cool when we thought we were going to iterate over tick bounds
struct MarketSnapshotIter<'a> {
    snapshot: &'a MarketSnapshot,
    direction: OrderDirection,
    cur_price: SqrtPriceX96,
    next_tick: i32,
    cur_pool: &'a PoolRange,
    cur_pool_idx: usize,
}

impl<'a> MarketSnapshotIter<'a> {
    fn quantity_for_current_range(&self) -> Option<(SqrtPriceX96, U256)> {
        // Convert into the u256-type we need for this libarary
        let cast_cur_price = ruint_to_u256(Uint::from(self.cur_price.0));
        // Our target price is the price precisely at our target tick
        let target_price = get_sqrt_ratio_at_tick(self.next_tick).unwrap();
        let cast_target_price: SqrtPriceX96 = Uint::from(u256_to_ruint(target_price)).into();
        // Liquidity comes from the current pool
        let liquidity = self.cur_pool.liquidity;
        // Do our calculation
        let quantity_t0 = _get_amount_0_delta(cast_cur_price, target_price, liquidity, true).unwrap();
        // Convert back to our Alloy-type U256
        Some((cast_target_price, u256_to_ruint(quantity_t0)))
    }
}

impl<'a> Iterator for MarketSnapshotIter<'a> {
    type Item = UniswapOrder;

    fn next(&mut self) -> Option<Self::Item> {
        // If the AMM is producing bids, we expect each filled order to lower the price, so we will walk
        // downwards along our tick range

        // If our next tick is out of the tick range, we are complete
        if self.next_tick < MIN_TICK || self.next_tick > MAX_TICK { return None; }

        // Make sure we're operating in the correct pool for our next tick - we're taking advantage
        // of the fact that we know our MarketSnapshot has ensured that all the pools are contiguous
        if self.next_tick > self.cur_pool.upper_tick {
            // Add one but fail at the limits of usize which should never happen but hey it's safe
            self.cur_pool_idx = self.cur_pool_idx.checked_add(1)?;
            self.cur_pool = self.snapshot.get_range(self.cur_pool_idx)?;
        } else if self.next_tick < self.cur_pool.lower_tick {
            // Subtract one but if we're going below zero just return None because we're done
            self.cur_pool_idx = self.cur_pool_idx.checked_sub(1)?;
            self.cur_pool = self.snapshot.get_range(self.cur_pool_idx)?;
        }

        // Get the quantity needed to move between our current price and the price at
        // the next tick
        let (new_price, quantity) = self.quantity_for_current_range()?;

        let start_price = self.cur_price.clone();
        let end_price = new_price.clone();
        
        // Update our current price
        self.cur_price = new_price;

        // Update our ticks
        self.next_tick = match self.direction {
            OrderDirection::Bid => self.next_tick - 1,
            OrderDirection::Ask => self.next_tick + 1
        };

        Some(UniswapOrder { start_price, end_price, quantity })
    }
}

/// A UniswapOrder represents the conversion of one tick interval into a limit order, however it also includes
/// the prices for both ends of that (very small) interval.  This will allow us to ensure that we appropriately
/// match these orders with their counterpart Ask or Bid orders, but also lets us hold on to the data we need
/// in order to deal with situations in which we have a partial price fill.
/// 
/// Note that within this code, a UniswapOrder will always span precisely one Tick interval, which means
/// that it will always have constant Liquidity.
#[derive(Debug, Clone)]
pub struct UniswapOrder {
    /// Starting price of this order before it has been filled at all
    start_price: SqrtPriceX96,
    /// The resultant price after this order has been completely filled
    end_price: SqrtPriceX96,
    /// Total quantity of Token0 required to entirely fill this order
    quantity: U256
}

impl UniswapOrder {
    pub fn quantity(&self) -> f64 { self.quantity.into() }
    pub fn price(&self) -> f64 { self.start_price.as_float_price() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bid_iter_spans_price_range() {
        let ranges = vec![
            PoolRange::new(2000, 2100, 10000000).unwrap(),
            PoolRange::new(2100, 2200, 10000000).unwrap(),
            PoolRange::new(2200, 2300, 10000000).unwrap(),
            PoolRange::new(2300, 2400, 10000000).unwrap(),
            PoolRange::new(2400, 2500, 10000000).unwrap()
        ];
        let start_tick = 2250;
        let start_price_offset = 10;

        let book_price: SqrtPriceX96 = Uint::from(u256_to_ruint(get_sqrt_ratio_at_tick(start_tick).unwrap() + start_price_offset)).into();
        let book = MarketSnapshot::new(ranges, book_price).unwrap();
        let iter = book.bid_iter().unwrap();
        let ticks: Vec<UniswapOrder> = iter.collect();
        assert_eq!(ticks[0].start_price, book_price, "First bid's start price is not equal to our start price");
        let last_idx = ticks.len() - 1;
        let last_tick_price: SqrtPriceX96 = Uint::from(u256_to_ruint(get_sqrt_ratio_at_tick(2000).unwrap())).into();
        assert_eq!(ticks[last_idx].end_price, last_tick_price, "Last bid's end price is not equal to the price at the last available tick");
        assert!(ticks.iter().enumerate().all(|(i, o)| {
            let target_tick = start_tick - i as i32;
            let last_tick_price: SqrtPriceX96 = Uint::from(u256_to_ruint(get_sqrt_ratio_at_tick(target_tick).unwrap())).into();
            o.end_price == last_tick_price
        }), "Not all bids align on the price at a tick boundary")
    }
}