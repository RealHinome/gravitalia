use anyhow::Result;
use neo4rs::{ConfigBuilder, Graph};
use std::sync::Arc;

/// Define a structure to manage the Neo4j or Memgraph connection.
#[derive(Clone)]
pub struct Bolt {
    /// Bolt protocol connection as atomic reference.
    pub connection: Arc<Graph>,
}

/// Define a trait for the BoltManager with methods to interact with every database using Bolt protocol (such as Neo4j).
pub trait BoltManager {}

impl BoltManager for Bolt {}

/// Initialize the connection using Bolt protocol.
pub async fn init(config: crate::model::config::Config) -> Result<Arc<Graph>> {
    let bolt_config = ConfigBuilder::default()
        .uri(config.database.bolt.hosts[0].clone())
        .user(config.database.bolt.username.unwrap_or("".to_string()))
        .password(config.database.bolt.password.unwrap_or("".to_string()))
        .max_connections(
            config
                .database
                .bolt
                .pool_size
                .try_into()
                .unwrap_or_default(),
        )
        .build()?;

    Ok(Arc::new(Graph::connect(bolt_config).await?))
}
