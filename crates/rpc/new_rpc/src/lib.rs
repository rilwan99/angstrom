pub mod api;
pub mod impls;

pub type RpcResult<T> = Result<T, RpcError>;

pub enum RpcError {}
