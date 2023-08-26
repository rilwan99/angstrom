use std::{
    collections::{HashMap, HashSet},
    fmt,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr
};

use jsonrpsee::{
    server::{IdProvider, Server, ServerHandle},
    Methods, RpcModule
};
use tower::layer::util::{Identity, Stack};
use tower_http::cors::CorsLayer;
use tracing::{instrument, trace};

pub struct SubmissionServer {
    server: Server<Stack<CorsLayer, Identity>>
}

impl SubmissionServer {
}
