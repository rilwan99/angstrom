use prometheus::IntGauge;

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
