use prometheus::{IntGauge, IntGaugeVec};

#[derive(Clone)]
pub struct OrderStorageMetrics {
    // number of vanilla limit orders
    vanilla_limit_orders:        IntGauge,
    // number of composable limit orders
    composable_limit_orders:     IntGauge,
    // number of searcher orders
    searcher_orders:             IntGauge,
    // number of pending finalization orders
    pending_finalization_orders: IntGauge
}

impl Default for OrderStorageMetrics {
    fn default() -> Self {
        let vanilla_limit_orders = prometheus::register_int_gauge!(
            "order_storage_vanilla_limit_orders",
            "number of vanilla limit orders",
        )
        .unwrap();

        let composable_limit_orders = prometheus::register_int_gauge!(
            "order_storage_composable_limit_orders",
            "number of composable limit orders",
        )
        .unwrap();

        let searcher_orders = prometheus::register_int_gauge!(
            "order_storage_searcher_orders",
            "number of searcher orders",
        )
        .unwrap();

        let pending_finalization_orders = prometheus::register_int_gauge!(
            "order_storage_pending_finalization_orders",
            "number of pending finalization orders",
        )
        .unwrap();

        Self {
            vanilla_limit_orders,
            searcher_orders,
            pending_finalization_orders,
            composable_limit_orders
        }
    }
}

impl OrderStorageMetrics {
    pub fn incr_vanilla_limit_orders(&self, count: usize) {
        self.vanilla_limit_orders.add(count as i64);
    }

    pub fn decr_vanilla_limit_orders(&self, count: usize) {
        self.vanilla_limit_orders.sub(count as i64);
    }

    pub fn incr_composable_limit_orders(&self, count: usize) {
        self.composable_limit_orders.add(count as i64);
    }

    pub fn decr_composable_limit_orders(&self, count: usize) {
        self.composable_limit_orders.sub(count as i64);
    }

    pub fn incr_searcher_orders(&self, count: usize) {
        self.searcher_orders.add(count as i64);
    }

    pub fn decr_searcher_orders(&self, count: usize) {
        self.searcher_orders.sub(count as i64);
    }

    pub fn incr_pending_finalization_orders(&self, count: usize) {
        self.pending_finalization_orders.add(count as i64);
    }

    pub fn decr_pending_finalization_orders(&self, count: usize) {
        self.pending_finalization_orders.sub(count as i64);
    }
}

#[derive(Clone)]
pub struct VanillaLimitOrderPoolMetrics {
    // number of vanilla limit orders
    total_orders:         IntGauge,
    // number of pending vanilla limit orders
    total_pending_orders: IntGauge,
    // number of parked vanilla limit orders
    total_parked_orders:  IntGauge,
    // // number of pending pools
    // pending_pools:  IntGauge,
    // // number of parked pools
    // parked_pools:   IntGauge,
    // number of pending orders per pool
    pending_orders:       IntGaugeVec,
    // number of parked orders per pool
    parked_orders:        IntGaugeVec
}

impl Default for VanillaLimitOrderPoolMetrics {
    fn default() -> Self {
        let total_orders = prometheus::register_int_gauge!(
            "vanilla_limit_order_pool_total_orders",
            "number of total vanilla limit orders",
        )
        .unwrap();

        let total_pending_orders = prometheus::register_int_gauge!(
            "vanilla_limit_order_pool_total_pending_orders",
            "number of pending vanilla limit orders",
        )
        .unwrap();

        let total_parked_orders = prometheus::register_int_gauge!(
            "vanilla_limit_order_pool_total_parked_orders",
            "number of parked vanilla limit orders",
        )
        .unwrap();

        // let pending_pools = prometheus::register_int_gauge!(
        //     "vanilla_limit_order_pool_pending_pools",
        //     "number of pending pools",
        // )
        // .unwrap();

        // let parked_pools = prometheus::register_int_gauge!(
        //     "vanilla_limit_order_pool_parked_pools",
        //     "number of parked pools",
        // )
        // .unwrap();

        let pending_orders = prometheus::register_int_gauge_vec!(
            "vanilla_limit_order_pool_pending_orders",
            "number of pending orders per pool",
            &["pool_id"]
        )
        .unwrap();

        let parked_orders = prometheus::register_int_gauge_vec!(
            "vanilla_limit_order_pool_parked_orders",
            "number of parked orders per pool",
            &["pool_id"]
        )
        .unwrap();

        Self {
            total_orders,
            parked_orders,
            pending_orders,
            total_parked_orders,
            total_pending_orders
        }
    }
}

impl VanillaLimitOrderPoolMetrics {
    fn incr_total_orders(&self, count: usize) {
        self.total_orders.add(count as i64);
    }

    fn decr_total_orders(&self, count: usize) {
        self.total_orders.sub(count as i64);
    }

    fn incr_total_parked_orders(&self, count: usize) {
        self.total_parked_orders.add(count as i64);
        self.incr_total_orders(count);
    }

    fn decr_total_parked_orders(&self, count: usize) {
        self.total_parked_orders.sub(count as i64);
        self.decr_total_orders(count);
    }

    fn incr_total_pending_orders(&self, count: usize) {
        self.total_pending_orders.add(count as i64);
        self.incr_total_orders(count);
    }

    fn decr_total_pending_orders(&self, count: usize) {
        self.total_pending_orders.sub(count as i64);
        self.decr_total_orders(count);
    }

    pub fn incr_parked_orders(&self, pool_id: usize, count: usize) {
        self.parked_orders
            .get_metric_with_label_values(&[&pool_id.to_string()])
            .unwrap()
            .add(count as i64);
        self.incr_total_parked_orders(count);
    }

    pub fn decr_parked_orders(&self, pool_id: usize, count: usize) {
        self.parked_orders
            .get_metric_with_label_values(&[&pool_id.to_string()])
            .unwrap()
            .sub(count as i64);
        self.decr_total_parked_orders(count);
    }

    pub fn incr_pending_orders(&self, pool_id: usize, count: usize) {
        self.pending_orders
            .get_metric_with_label_values(&[&pool_id.to_string()])
            .unwrap()
            .add(count as i64);
        self.incr_total_pending_orders(count);
    }

    pub fn decr_pending_orders(&self, pool_id: usize, count: usize) {
        self.pending_orders
            .get_metric_with_label_values(&[&pool_id.to_string()])
            .unwrap()
            .sub(count as i64);
        self.decr_total_pending_orders(count);
    }
}

#[derive(Clone)]
pub struct ComposableLimitOrderPoolMetrics {
    // number of composable limit orders
    total_orders: IntGauge,
    // number of orders per pool
    all_orders:   IntGaugeVec
}

impl Default for ComposableLimitOrderPoolMetrics {
    fn default() -> Self {
        let total_orders = prometheus::register_int_gauge!(
            "composable_limit_order_pool_total_orders",
            "number of composable limit orders",
        )
        .unwrap();

        let all_orders = prometheus::register_int_gauge_vec!(
            "composable_limit_order_pool_all_orders",
            " number of orders per pool",
            &["pool_id"]
        )
        .unwrap();

        Self { total_orders, all_orders }
    }
}

impl ComposableLimitOrderPoolMetrics {
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
