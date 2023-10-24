use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "order"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "order"))]
#[async_trait::async_trait]
pub trait OrderApi {
    #[method(name = "submit_order")]
    async fn submit_order(&self) -> RpcResult<bool>;
}
