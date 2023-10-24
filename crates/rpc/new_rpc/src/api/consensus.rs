use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use reth_primitives::Address;

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "consensus"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "consensus"))]
#[async_trait::async_trait]
pub trait ConsensusApi {
    #[method(name = "state")]
    async fn consensus_state(&self) -> RpcResult<u8>;
}
