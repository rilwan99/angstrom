use clap::Parser;
use matching_engine::{book::{order::{LimitOrder, Order}, sort::SortStrategy, OrderBook}, cfmm::uniswap::{math::{sqrt_price_at_tick, tick_at_sqrt_price}, MarketSnapshot, PoolRange, SqrtPriceX96}, strategy::{MatchingStrategy, SimpleCheckpointStrategy}};
use rand_distr::{Distribution, SkewNormal};

#[derive(Parser, Debug)]
struct Args {
    /// Number of bid orders to generate
    #[arg(short, long, default_value_t=1000)]
    bids: usize,
    /// Number of ask orders to generate
    #[arg(short, long, default_value_t=1000)]
    asks: usize,
    /// Price used as the center of our price distributions
    #[arg(short, long, default_value_t=100_000_000.0)]
    price: f64,
    /// Scale of bid price distribution
    #[arg(long, default_value_t=100000.0, help_heading="Bid Order Statistics")]
    bid_price_scale: f64,
    /// Shape of bid price distribution
    #[arg(long, default_value_t=-2.0, help_heading="Bid Order Statistics")]
    bid_price_shape: f64,
    /// Average bid order volume to use
    #[arg(long, default_value_t=100.0, help_heading="Bid Order Statistics")]
    bid_volume_mean: f64,
    /// Standard deviation of bid order volume
    #[arg(long, default_value_t=1.0, help_heading="Bid Order Statistics")]
    bid_volume_sd: f64,
    /// Scale of ask price distribution
    #[arg(long, default_value_t=100000.0, help_heading="Ask Order Statistics")]
    ask_price_scale: f64,
    /// Shape of ask price distribution
    #[arg(long, default_value_t=2.0, help_heading="Ask Order Statistics")]
    ask_price_shape: f64,
    /// Average ask order volume to use
    #[arg(long, default_value_t=100.0, help_heading="Ask Order Statistics")]
    ask_volume_mean: f64,
    /// Standard deviation of ask order volume
    #[arg(long, default_value_t=1.0, help_heading="Ask Order Statistics")]
    ask_volume_sd: f64,
    
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    // Positive skew means prices skew higher, our asks should peek below our central price but generally be higher
    let asks: Vec<Order> = SkewNormal::new(0.0, args.ask_price_scale, args.ask_price_shape).unwrap()
        .map(|v| Order::KillOrFill(LimitOrder::new(args.price + v, 100.0))).sample_iter(&mut rng).take(args.bids).collect();

    // Negative skew means prices skew lower, our bids should peek above our central price but generally be lower
    let bids: Vec<Order> = SkewNormal::new(0.0, args.bid_price_scale, args.bid_price_shape).unwrap()
        .map(|v| Order::KillOrFill(LimitOrder::new(args.price + v, 99.0))).sample_iter(&mut rng).take(args.asks).collect();

    let middle_tick = tick_at_sqrt_price(SqrtPriceX96::from_float_price(args.price)).unwrap();
    let amm_price = sqrt_price_at_tick(middle_tick + 1).unwrap();
    println!("Starting AMM price: {:?}", amm_price.as_float_price());
    let lower_tick = middle_tick - 10000;
    let upper_tick = middle_tick + 10000;
    let ranges = vec![
        PoolRange::new(lower_tick, upper_tick, 2e18 as u128).unwrap(),
    ];
    let amm = MarketSnapshot::new(ranges, amm_price).unwrap();
    
    let book = OrderBook::new(Some(amm), bids, asks, Some(SortStrategy::ByPriceByVolume));

    // We're going to solve using our Simple Checkpoint Strategy
    let solved = SimpleCheckpointStrategy::run(&book).unwrap();

    // Print out some very basic stats
    let results = solved.results();
    println!("{} bids filled", solved.bid_outcomes.iter().filter(|x| x.is_filled()).count());
    println!("{} asks filled", solved.ask_outcomes.iter().filter(|x| x.is_filled()).count());
    println!("{:?}", results);

}