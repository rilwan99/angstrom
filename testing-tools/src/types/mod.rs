mod events;
use std::sync::OnceLock;

pub use events::*;
mod handles;
pub use handles::*;
mod hooks;
pub use hooks::*;

static INITIAL_RPC_PORT: OnceLock<u16> = OnceLock::new();

pub fn set_initial_rpc_port(port: u16) {
    INITIAL_RPC_PORT
        .set(port)
        .expect("INITIAL_RPC_PORT has already been set");
}

pub fn get_rpc_port_with_node_id(node_id: u64) -> u64 {
    *INITIAL_RPC_PORT
        .get()
        .expect("INITIAL_RPC_PORT has not been set") as u64
        + node_id
}
