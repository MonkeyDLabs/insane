use crate::{config::InsaneConfig, environment::Environment};

#[cfg(feature = "with-sql")]
use sea_orm::DatabaseConnection;

#[async_trait::async_trait]
pub trait Context: Sync + Send {
    /// Get the environment of the context.
    fn environment(&self) -> &Environment;

    /// Get the configuration of the context.
    fn config(&self) -> &InsaneConfig;

    /// Get the configuration of the context.
    #[cfg(feature = "with-sql")]
    fn sql(&self) -> &DatabaseConnection;
}

// pub trait ServerContext<T> {
//     type ContextType: T;
// }

#[derive(Clone)]
pub struct DefaultContext {
    pub environment: Environment,
    pub config: InsaneConfig,

    #[cfg(feature = "with-sql")]
    /// A database connection used by the application.    
    pub sql: DatabaseConnection,

    // #[cfg(feature = "with-redis")]
    // /// An optional connection pool for Redis, for worker tasks
    // pub redis: Option<Pool<RedisConnectionManager>>,
}

#[async_trait::async_trait]
impl Context for DefaultContext {
    fn environment(&self) -> &Environment {
        &self.environment
    }

    fn config(&self) -> &InsaneConfig {
        &self.config
    }

    #[cfg(feature = "with-sql")]
    fn sql(&self) -> &DatabaseConnection {
        &self.sql
    }
}
