use crate::error::{Error, Result};
use insane_core::{
    config::loader::{Config, ConfigLoader},
    environment::Environment,
};
use serde::{Deserialize, Serialize};

const DEFAULT_SERVER_BINDING: &str = "[::]";

fn default_binding() -> String {
    DEFAULT_SERVER_BINDING.to_string()
}

fn default_port() -> i32 {
    8089
}

/// Server middleware configuration structure.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Middlewares {
    /// Middleware that enable compression for the response.
    pub compression: Option<EnableMiddleware>,
    /// Middleware that enable etag cache headers.
    pub etag: Option<EnableMiddleware>,
    /// Middleware that limit the payload request.
    pub limit_payload: Option<LimitPayloadMiddleware>,
    /// Middleware that improve the tracing logger and adding trace id for each
    /// request.
    pub logger: Option<EnableMiddleware>,
    /// catch any code panic and log the error.
    pub catch_panic: Option<EnableMiddleware>,
    /// Setting a global timeout for the requests
    pub timeout_request: Option<TimeoutRequestMiddleware>,
    /// Setting cors configuration
    pub cors: Option<CorsMiddleware>,
    // /// Serving static assets
    // #[serde(rename = "static")]
    // pub static_assets: Option<StaticAssetsMiddleware>,
}

/// CORS middleware configuration
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CorsMiddleware {
    pub enable: bool,
    /// Allow origins
    pub allow_origins: Option<Vec<String>>,
    /// Allow headers
    pub allow_headers: Option<Vec<String>>,
    /// Allow methods
    pub allow_methods: Option<Vec<String>>,
    /// Max age
    pub max_age: Option<u64>,
}

/// Timeout middleware configuration
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TimeoutRequestMiddleware {
    pub enable: bool,
    // Timeout request in milliseconds
    pub timeout: u64,
}

/// Limit payload size middleware configuration
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LimitPayloadMiddleware {
    pub enable: bool,
    /// Body limit. for example: 5mb
    pub body_limit: String,
}

/// A generic middleware configuration that can be enabled or
/// disabled.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EnableMiddleware {
    pub enable: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct HTTPServerConfig {
    #[serde(default = "default_binding")]
    pub binding: String,
    /// The port on which the server should listen for incoming connections.
    #[serde(default = "default_port")]
    pub port: i32,
    /// Middleware configurations for the server, including payload limits,
    /// logging, and error handling.
    pub middlewares: Middlewares,

    /// Enable the server
    pub enable: bool,
}

impl HTTPServerConfig {
    pub fn new(env: &Environment, app_name: &str) -> Result<Self> {
        let config = HTTPServerConfig::from_key("http", env, app_name)?;
        Ok(config)
    }

    pub fn load_from_env(env: &Environment, app_name: &str) -> Result<Self> {
        HTTPServerConfig::new(env, app_name)
    }
}

#[async_trait::async_trait]
impl Config for HTTPServerConfig {

    fn enable(&self) -> bool {
        self.enable
    }

}

#[async_trait::async_trait]
impl ConfigLoader for HTTPServerConfig {
    type Config = HTTPServerConfig;
    type Error = Error;
}
