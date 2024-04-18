pub mod backtrace;
pub mod config;
pub mod context;
pub mod environment;
pub mod error;
pub mod hook;
pub mod server;
pub mod traces;

#[cfg(feature = "with-sql")]
pub mod sql;

pub(crate) mod banner;
pub mod boot_loader;

pub mod prelude {
    pub use crate::config::InsaneConfig;
    pub use crate::context::Context;
    pub use crate::environment::Environment;
    pub use crate::error::Result;
    pub use crate::hook::Hooks;
    pub use crate::server::Server;

    #[cfg(feature = "with-sql")]
    pub use sea_orm_migration::MigratorTrait;

    #[cfg(feature = "with-sql")]
    pub use sea_orm::DatabaseConnection;
}
