// loco.rs

//! Defines the application environment.
//! By given the environment you can also load the application configuration
//!
//! # Example:
//!
//! ```rust
//! use std::str::FromStr;
//! use loco_rs::environment::Environment;
//!
//! pub fn load(environment: &str) {
//!  let environment = Environment::from_str(environment).unwrap_or(Environment::Any(environment.to_string()));
//!  let config = environment.load().expect("failed to load environment");
//! }
//! ```
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
// use crate::{config::InsaneConfig, error::Result, hook::Hooks};
// use insane_http::InsaneHTTP;

pub const DEFAULT_ENVIRONMENT: &str = "development";

impl From<String> for Environment {
    fn from(env: String) -> Self {
        Self::from_str(&env).unwrap_or(Self::Any(env))
    }
}

#[must_use]
pub fn resolve_from_env() -> String {
    std::env::var("INSANE_ENV").unwrap_or_else(|_| DEFAULT_ENVIRONMENT.to_string())
}

/// Application environment
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Environment {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "test")]
    Test,
    Any(String),
}

// impl Environment {
//     /// Load environment variables from local configuration
//     ///
//     /// # Errors
//     ///
//     /// Returns error if an error occurs during loading
//     /// configuration file an parse into [`Config`] struct.
//     pub fn load<H: Hooks>(&self) -> Result<InsaneConfig> {
//         InsaneConfig::new(self, H::app_name())
//     }

// }

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any(s) => s.fmt(f),
            _ => to_variant_name(self).expect("only enum supported").fmt(f),
        }
    }
}

impl FromStr for Environment {
    type Err = &'static str;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "production" => Ok(Self::Production),
            "development" => Ok(Self::Development),
            "test" => Ok(Self::Test),
            s => Ok(Self::Any(s.to_string())),
        }
    }
}
