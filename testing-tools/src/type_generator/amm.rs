use matching_engine::cfmm::uniswap::{math::sqrt_price_at_tick, MarketSnapshot, PoolRange};

pub fn generate_amm_market(target_tick: i32) -> MarketSnapshot {
    let range = PoolRange::new(target_tick - 100, target_tick + 100, 100_000_000).unwrap();
    let ranges = vec![range];
    let sqrt_price_x96 = sqrt_price_at_tick(target_tick).unwrap();
    MarketSnapshot::new(ranges, sqrt_price_x96).unwrap()
}
