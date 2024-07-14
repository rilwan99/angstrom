use clap::Parser;
use matching_engine::{
    book::{sort::SortStrategy, OrderBook},
    cfmm::uniswap::{math::tick_at_sqrt_price, SqrtPriceX96},
    simulation::{amm::single_position_amm, orders::order_distribution},
    strategy::{MatchingStrategy, SimpleCheckpointStrategy}
};

#[derive(Parser, Debug)]
struct Args {
    /// Number of bid orders to generate
    #[arg(short, long, default_value_t = 1000)]
    bids:            usize,
    /// Number of ask orders to generate
    #[arg(short, long, default_value_t = 1000)]
    asks:            usize,
    /// Price used as the center of our price distributions
    #[arg(short, long, default_value_t = 100_000_000.0)]
    price:           f64,
    /// Scale of bid price distribution
    #[arg(long, default_value_t = 100000.0, help_heading = "Bid Order Statistics")]
    bid_price_scale: f64,
    /// Shape of bid price distribution
    #[arg(long, default_value_t=-2.0, help_heading="Bid Order Statistics")]
    bid_price_shape: f64,
    /// Average bid order volume to use
    #[arg(long, default_value_t = 100.0, help_heading = "Bid Order Statistics")]
    bid_volume_mean: f64,
    /// Standard deviation of bid order volume
    #[arg(long, default_value_t = 1.0, help_heading = "Bid Order Statistics")]
    bid_volume_sd:   f64,
    /// Scale of ask price distribution
    #[arg(long, default_value_t = 100000.0, help_heading = "Ask Order Statistics")]
    ask_price_scale: f64,
    /// Shape of ask price distribution
    #[arg(long, default_value_t = 2.0, help_heading = "Ask Order Statistics")]
    ask_price_shape: f64,
    /// Average ask order volume to use
    #[arg(long, default_value_t = 100.0, help_heading = "Ask Order Statistics")]
    ask_volume_mean: f64,
    /// Standard deviation of ask order volume
    #[arg(long, default_value_t = 1.0, help_heading = "Ask Order Statistics")]
    ask_volume_sd:   f64
}

fn main() {
    let args = Args::parse();

    let asks = order_distribution(
        false,
        10,
        args.price,
        args.ask_price_scale,
        args.ask_price_shape,
        99.0,
        1.0,
        0.0
    )
    .unwrap();

    let bids = order_distribution(
        true,
        10,
        args.price,
        args.bid_price_scale,
        args.bid_price_shape,
        99.0,
        1.0,
        0.0
    )
    .unwrap();

    let middle_tick = tick_at_sqrt_price(SqrtPriceX96::from_float_price(args.price)).unwrap();
    let amm = single_position_amm(middle_tick, 10000, 2e18 as u128).unwrap();

    let book = OrderBook::new(10, Some(amm), bids, asks, Some(SortStrategy::ByPriceByVolume));

    //println!("Orderbook\n{:?}", book.amm());
    //return;

    // We're going to solve using our Simple Checkpoint Strategy
    let solved = SimpleCheckpointStrategy::run(&book).unwrap();

    // Print out some very basic stats
    let results = solved.results();
    println!("{} bids filled", solved.bid_outcomes.iter().filter(|x| x.is_filled()).count());
    println!("{} asks filled", solved.ask_outcomes.iter().filter(|x| x.is_filled()).count());
    println!("{:?}", results);
    // println!("{:?}", solved.crosspool_outcomes())
}
