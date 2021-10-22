use derive_more::Constructor;
use std::convert::TryFrom;

pub(crate) use self::cluster_metadata_manager::ClusterMetadataManager;
#[cfg(feature = "rust-tls")]
pub use self::config_rustls::{NodeRustlsConfig, NodeRustlsConfigBuilder};
pub use self::config_tcp::{NodeTcpConfig, NodeTcpConfigBuilder};
pub use self::connection_manager::{startup, ConnectionManager};
pub use self::keyspace_holder::KeyspaceHolder;
pub use self::node_address::NodeAddress;
pub use self::node_info::NodeInfo;
pub use self::pager::{ExecPager, PagerState, QueryPager, SessionPager};
#[cfg(feature = "rust-tls")]
pub use self::rustls_connection_manager::RustlsConnectionManager;
pub use self::session::connect_generic;
pub(crate) use self::session_context::SessionContext;
pub use self::tcp_connection_manager::TcpConnectionManager;
pub use self::token_map::TokenMap;
pub use self::topology::cluster_metadata::ClusterMetadata;
use crate::error;
use crate::future::BoxFuture;
use crate::transport::CdrsTransport;

mod cluster_metadata_manager;
#[cfg(feature = "rust-tls")]
mod config_rustls;
mod config_tcp;
pub(crate) mod connection_manager;
mod control_connection;
mod keyspace_holder;
mod metadata_builder;
mod node_address;
mod node_info;
mod pager;
#[cfg(feature = "rust-tls")]
mod rustls_connection_manager;
pub mod session;
mod session_context;
mod tcp_connection_manager;
pub(crate) mod token_factory;
mod token_map;
pub mod topology;

/// Generic connection configuration trait that can be used to create user-supplied
/// connection objects that can be used with the `session::connect()` function.
pub trait GenericClusterConfig<T: CdrsTransport, CM: ConnectionManager<T>>: Send + Sync {
    fn create_manager(&self) -> BoxFuture<error::Result<CM>>;

    /// Returns desired event channel capacity. Take a look at
    /// [`Session`](self::session::Session) builders for more info.
    fn event_channel_capacity(&self) -> usize;
}

/// A token on the ring. Only Murmur3 tokens are supported for now.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Debug, Hash, Constructor)]
pub struct Murmur3Token {
    pub value: i64,
}

impl TryFrom<String> for Murmur3Token {
    type Error = error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .parse()
            .map_err(|error| format!("Error parsing token: {}", error).into())
            .map(Murmur3Token::new)
    }
}