use matching_engine::{book::{order::{LimitOrder, Order}, sort::SortStrategy, volume::VolumeFillBookSolver, OrderBook}, cfmm::uniswap::{math::tick_at_sqrt_price, MarketSnapshot, PoolRange, SqrtPriceX96}};
use rand_distr::{Distribution, SkewNormal};

fn main() {

    // Arbitrary central price
    let central_price = 100_000_000.0;
    println!("Order book generator");

    let mut rng = rand::thread_rng();
    // Positive skew means prices skew higher, our asks should peek below our central price but generally be higher
    let asks: Vec<Order> = SkewNormal::new(0.0, 1000.0, 2.0).unwrap()
        .map(|v| Order::KillOrFill(LimitOrder::new(central_price + v, 100.0))).sample_iter(&mut rng).take(100).collect();

    // Negative skew means prices skew lower, our bids should peek above our central price but generally be lower
    let bids: Vec<Order> = SkewNormal::new(0.0, 1000.0, -2.0).unwrap()
        .map(|v| Order::KillOrFill(LimitOrder::new(central_price + v, 100.0))).sample_iter(&mut rng).take(100).collect();

    let amm_price = SqrtPriceX96::from_float_price(central_price);
    let lower_tick = tick_at_sqrt_price(amm_price).unwrap() - 100;
    let upper_tick = tick_at_sqrt_price(amm_price).unwrap() + 100;
    let ranges = vec![
        PoolRange::new(lower_tick, upper_tick, 10000000).unwrap(),
    ];
    let amm = MarketSnapshot::new(ranges, SqrtPriceX96::from_float_price(central_price)).unwrap();
    
    let book = OrderBook::new(amm, bids, asks, Some(SortStrategy::ByPriceByVolume));
    let mut solver = VolumeFillBookSolver::new(&book);
    let (price, volume) = solver.fill();

    println!("{:?}", solver.bid_outcomes);
    println!("{:?} - {:?}", price, volume);

}