#[derive(Debug, Clone, Copy, Default)]
pub struct AngstromTestnetConfig {
    pub intial_node_count:       u64,
    pub initial_rpc_port:        u16,
    pub testnet_block_time_secs: u64
}

impl AngstromTestnetConfig {
    pub fn new(
        intial_node_count: u64,
        initial_rpc_port: u16,
        testnet_block_time_secs: u64
    ) -> Self {
        Self { intial_node_count, initial_rpc_port, testnet_block_time_secs }
    }

    pub fn rpc_port_with_node_id(&self, node_id: u64) -> u64 {
        self.initial_rpc_port as u64 + node_id
    }
}
