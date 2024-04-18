use insane_core::config::InsaneConfig;
// use insane_core::context::ServerContext;
use crate::config::HTTPServerConfig;
use insane_core::prelude::*;
// use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HttpContext {
    pub server_config: HTTPServerConfig,
    pub config: InsaneConfig,
    pub environment: Environment,

    #[cfg(feature = "with-sql")]
    pub sql: DatabaseConnection,
}

impl HttpContext {
    pub fn new(server_config: HTTPServerConfig, context: Arc<Box<dyn Context>>) -> Self {
        Self {
            server_config,
            config: context.config().clone(),
            environment: context.environment().clone(),
            sql: context.sql().clone(),
        }
    }
}

// impl Context for HttpContext {
//   fn environment(&self) -> &Environment {
//     &self.environment
//   }

//   fn config(&self) -> &InsaneConfig {
//     &self.config
//   }
// }

// impl ServerContext<HttpContext> for HttpContext {
//   type ContextType = HttpContext;
// }
