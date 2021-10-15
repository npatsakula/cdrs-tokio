use derive_more::Constructor;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Constructor)]
pub struct NodeInfo {
    pub host_id: Uuid,
    pub broadcast_rpc_address: SocketAddr,
    pub broadcast_address: Option<SocketAddr>,
}
