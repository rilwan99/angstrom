use alloy_primitives::U256;
use angstrom_types::sol_bindings::sol::FlashOrder;
use rand_distr::{Distribution, SkewNormal};

use crate::book::order::Order;

pub fn order_distribution<'a>(
    number: usize,
    price_location: f64,
    price_scale: f64,
    price_shape: f64,
    quantity_location: f64,
    quantity_scale: f64,
    quantity_shape: f64
) -> Result<Vec<Order<'a>>, String> {
    let mut rng = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let price_gen = SkewNormal::new(price_location, price_scale, price_shape)
        .map_err(|e| format!("Error creating price distribution: {}", e))?;
    let quantity_gen = SkewNormal::new(quantity_location, quantity_scale, quantity_shape)
        .map_err(|e| format!("Error creating price distribution: {}", e))?;
    Ok(price_gen
        .sample_iter(&mut rng)
        .zip(quantity_gen.sample_iter(&mut rng2))
        .map(|(p, q)| {
            Order::KillOrFill(FlashOrder {
                max_amount_in_or_out: U256::from(q),
                min_price: U256::from(p),
                ..FlashOrder::default()
            })
        })
        .take(number)
        .collect())
}
