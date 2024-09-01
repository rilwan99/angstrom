use prometheus::IntGauge;

#[derive(Clone)]
pub struct FinalizationOrderPoolMetrics {
    // number of blocks tracked
    blocks_tracked: IntGauge,
    // number of finalization orders
    total_orders:   IntGauge
}

impl Default for FinalizationOrderPoolMetrics {
    fn default() -> Self {
        let total_orders = prometheus::register_int_gauge!(
            "finalization_order_pool_total_orders",
            "number of finalization orders",
        )
        .unwrap();

        let blocks_tracked = prometheus::register_int_gauge!(
            "finalization_order_pool_blocks_tracked",
            "number of blocks tracked",
        )
        .unwrap();

        Self { total_orders, blocks_tracked }
    }
}

impl FinalizationOrderPoolMetrics {
    pub fn incr_total_orders(&self) {
        self.total_orders.add(1);
    }

    pub fn decr_total_orders(&self) {
        self.total_orders.sub(1);
    }

    pub fn incr_blocks_tracked(&self) {
        self.blocks_tracked.add(1);
    }

    pub fn decr_blocks_tracked(&self) {
        self.blocks_tracked.sub(1);
    }
}
