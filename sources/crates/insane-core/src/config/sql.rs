use serde::{Deserialize, Serialize};

/// Database configuration
///
/// Configures the [SeaORM](https://www.sea-ql.org/SeaORM/) connection and pool, as well as Loco's additional DB
/// management utils such as `auto_migrate`, `truncate` and `recreate`.
///
/// Example (development):
/// ```yaml
/// # config/development.yaml
/// database:
///   uri: {{ get_env(name="DATABASE_URL", default="...") }}
///   enable_logging: true
///   connect_timeout: 500
///   idle_timeout: 500
///   min_connections: 1
///   max_connections: 1
///   auto_migrate: true
///   dangerously_truncate: false
///   dangerously_recreate: false
/// ```
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct SqlConfig {
    /// The URI for connecting to the database. For example:
    /// * Postgres: `postgres://root:12341234@localhost:5432/myapp_development`
    /// * Sqlite: `sqlite://db.sqlite?mode=rwc`
    pub uri: String,

    /// Enable SQLx statement logging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,

    /// Minimum number of connections for a pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_connections: Option<u32>,

    /// Maximum number of connections for a pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,

    /// Set the timeout duration when acquiring a connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<u64>,

    /// Set the idle duration before closing a connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lifetime: Option<u64>,

    /// Run migration up when application loads. It is recommended to turn it on
    /// in development. In production keep it off, and explicitly migrate your
    /// database every time you need.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_migrate: Option<bool>,

    /// Truncate database when application loads. It will delete data from your
    /// tables. Commonly used in `test`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_truncate: Option<bool>,

    /// Recreate schema when application loads. Use it when you want to reset
    /// your database *and* structure (drop), this also deletes all of the data.
    /// Useful when you're just sketching out your project and trying out
    /// various things in development.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_recreate: Option<bool>,
}
