pub mod loader;
pub mod trace;
pub mod sql;

use serde::{Deserialize, Serialize};

#[cfg(feature = "with-sql")]
pub use sql::SqlConfig;

use self::loader::{Config, ConfigLoader};
use crate::{
    error::{Error, Result},
    hook::Hooks,
};
use crate::environment::Environment;
pub use trace::TraceConfig;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct InsaneConfig {
    pub application_name: String,
    #[cfg(feature = "with-sql")]
    pub sql: SqlConfig,

    pub tracing: TraceConfig,
}

impl InsaneConfig {
    pub fn new(env: &Environment, app_name: &str) -> Result<Self> {
        Ok(InsaneConfig::from_folder(env, app_name, None)?)
    }

    pub fn load_from_env<H: Hooks>(environment: &Environment) -> Result<Self> {
        InsaneConfig::new(&environment, H::app_name())
    }
}

#[async_trait::async_trait]
impl Config for InsaneConfig {}

#[async_trait::async_trait]
impl ConfigLoader for InsaneConfig {
    type Config = InsaneConfig;
    type Error = Error;
}
