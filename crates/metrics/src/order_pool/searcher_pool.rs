use prometheus::{IntGauge, IntGaugeVec};

#[derive(Clone)]
pub struct SearcherOrderPoolMetrics {
    // number of searcher orders
    total_orders: IntGauge,
    // number of orders per pool
    all_orders:   IntGaugeVec
}

impl Default for SearcherOrderPoolMetrics {
    fn default() -> Self {
        let total_orders = prometheus::register_int_gauge!(
            "searcher_order_pool_total_orders",
            "number of searcher orders",
        )
        .unwrap();

        let all_orders = prometheus::register_int_gauge_vec!(
            "searcher_order_pool_all_orders",
            " number of orders per pool",
            &["pool_id"]
        )
        .unwrap();

        Self { total_orders, all_orders }
    }
}

impl SearcherOrderPoolMetrics {
    fn incr_total_orders(&self, count: usize) {
        self.total_orders.add(count as i64);
    }

    fn decr_total_orders(&self, count: usize) {
        self.total_orders.sub(count as i64);
    }

    pub fn incr_all_orders(&self, pool_id: usize, count: usize) {
        self.all_orders
            .get_metric_with_label_values(&[&pool_id.to_string()])
            .unwrap()
            .add(count as i64);

        self.incr_total_orders(count);
    }

    pub fn decr_all_orders(&self, pool_id: usize, count: usize) {
        self.all_orders
            .get_metric_with_label_values(&[&pool_id.to_string()])
            .unwrap()
            .sub(count as i64);

        self.decr_total_orders(count);
    }
}
