use angstrom_types::matching::SqrtPriceX96;
use matching_engine::cfmm::uniswap::{MarketSnapshot, PoolRange};
use uniswap_v3_math::tick_math::{get_sqrt_ratio_at_tick, get_tick_at_sqrt_ratio};

#[derive(Debug, Default)]
pub struct AMMSnapshotBuilder {
    price: SqrtPriceX96,
    price_tick: i32,
    lower_tick: i32,
    upper_tick: i32,
    default_position_width: Option<i32>,
    default_position_liquidity: Option<u128>,
    positions: Option<Vec<PoolRange>>
}

impl AMMSnapshotBuilder {
    pub fn new(price: SqrtPriceX96) -> Self {
        let price_tick = get_tick_at_sqrt_ratio(price.into()).unwrap();
        let lower_tick = price_tick;
        let upper_tick = price_tick;
        Self { price, price_tick, lower_tick, upper_tick, ..Self::default() }
    }

    pub fn with_positions(self, positions: Vec<PoolRange>) -> Self {
        let (lower_tick, upper_tick) =
            positions
                .iter()
                .fold((i32::MAX, i32::MIN), |mut state, pos| {
                    if state.0 > pos.lower_tick() {
                        state.0 = pos.lower_tick();
                    }
                    if state.1 < pos.upper_tick() {
                        state.1 = pos.upper_tick();
                    }
                    state
                });
        Self { lower_tick, upper_tick, positions: Some(positions), ..self }
    }

    pub fn with_single_position(self, width: i32, liquidity: u128) -> Self {
        Self {
            default_position_width: Some(width),
            default_position_liquidity: Some(liquidity),
            ..self
        }
    }

    pub fn build(self) -> MarketSnapshot {
        // Maybe we should automatically make sure our price is within our PoolRanges
        let ranges = self.positions.unwrap_or_else(|| {
            let width = self.default_position_width.unwrap_or_default();
            let lower_tick = self.lower_tick.saturating_sub(width);
            let upper_tick = self.upper_tick.saturating_add(width);
            let liquidity = self.default_position_liquidity.unwrap_or_default();
            vec![PoolRange::new(lower_tick, upper_tick, liquidity).unwrap()]
        });
        MarketSnapshot::new(ranges, self.price).unwrap()
    }
}

pub fn generate_single_position_amm_at_tick(
    mid: i32,
    width: i32,
    liquidity: u128
) -> MarketSnapshot {
    let amm_price = SqrtPriceX96::from(get_sqrt_ratio_at_tick(mid + 1).unwrap());
    let lower_tick = mid - width;
    let upper_tick = mid + width;
    let ranges = vec![PoolRange::new(lower_tick, upper_tick, liquidity).unwrap()];
    MarketSnapshot::new(ranges, amm_price).unwrap()
}

pub fn generate_amm_market(target_tick: i32) -> MarketSnapshot {
    let range = PoolRange::new(target_tick - 100, target_tick + 100, 100_000_000).unwrap();
    let ranges = vec![range];
    let sqrt_price_x96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(target_tick).unwrap());
    MarketSnapshot::new(ranges, sqrt_price_x96).unwrap()
}
