use matching_engine::{
    book::{sort::SortStrategy, OrderBook},
    cfmm::uniswap::{math::tick_at_sqrt_price, SqrtPriceX96},
    matcher::Solution,
    simulation::{amm::single_position_amm, orders::order_distribution},
    strategy::{MatchingStrategy, SimpleCheckpointStrategy}
};

static CENTER_PRICE: f64 = 100_000_000.0;

fn do_solve(book: &OrderBook) -> Solution {
    let solved = SimpleCheckpointStrategy::run(book).unwrap();
    solved.results().clone()
}

fn generate_book() -> OrderBook {
    // Generate our bid and ask distribution
    let bids =
        order_distribution(true, 1000, CENTER_PRICE, 100000.0, -2.0, 100.0, 10.0, 0.0).unwrap();
    let asks =
        order_distribution(false, 1000, CENTER_PRICE, 100000.0, 2.0, 100.0, 10.0, 0.0).unwrap();
    let middle_tick = tick_at_sqrt_price(SqrtPriceX96::from_float_price(CENTER_PRICE)).unwrap();
    let amm = single_position_amm(middle_tick, 10000, 2e18 as u128).unwrap();

    // Create our book
    OrderBook::new(10, Some(amm), bids, asks, Some(SortStrategy::ByPriceByVolume))
}

fn main() {
    divan::main();
}

#[divan::bench]
fn solver_bench(bencher: divan::Bencher) {
    let book = generate_book();

    bencher.bench_local(move || {
        do_solve(divan::black_box(&book));
    })
}
