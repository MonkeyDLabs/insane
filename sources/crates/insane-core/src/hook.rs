use crate::{config::InsaneConfig, error::Result};
use crate::{context::Context, environment::Environment, server::Server};
use std::sync::Arc;

#[cfg(feature = "with-sql")]
use sea_orm::DatabaseConnection;
#[cfg(feature = "with-sql")]
use std::path::Path;


#[async_trait::async_trait]
pub trait Hooks: Sync + Send {
    /// Defines the composite app version
    #[must_use]
    fn app_version() -> String {
        "dev".to_string()
    }
    /// Defines the crate name
    ///
    /// Example
    /// ```rust
    /// fn app_name() -> &'static str {
    ///     env!("CARGO_CRATE_NAME")
    /// }
    /// ```
    fn app_name() -> &'static str;

    /// Override and return `Ok(true)` to provide an alternative logging and
    /// tracing stack of your own.
    /// When returning `Ok(true)`, Loco will *not* initialize its own logger,
    /// so you should set up a complete tracing and logging stack.
    ///
    /// # Errors
    /// If fails returns an error
    fn init_logger(_config: &InsaneConfig, _env: &Environment) -> Result<bool> {
        Ok(false)
    }

    /// Calling the function before run the app
    /// You can now code some custom loading of resources or other things before
    /// the app runs
    async fn before_run(_app_context: Arc<Box<dyn Context>>) -> Result<()> {
        Ok(())
    }

    /// Provide a list of initializers
    /// An initializer can be used to seamlessly add functionality to your app
    /// or to initialize some aspects of it.
    async fn initializers(
        _app_context: Arc<Box<dyn Context>>,
    ) -> Result<Vec<Box<dyn Initializer<Context = Arc<Box<dyn Context>>>>>> {
        Ok(vec![])
    }

    async fn servers(_app_context: Arc<Box<dyn Context>>) -> Result<Vec<Box<dyn Server>>>;

    /// Truncates the database as required. Users should implement this
    /// function. The truncate controlled from the [`crate::config::Database`]
    /// by changing dangerously_truncate to true (default false).
    /// Truncate can be useful when you want to truncate the database before any
    /// test.
    #[cfg(feature = "with-sql")]     
    async fn truncate(db: &DatabaseConnection) -> Result<()>;

    /// Seeds the database with initial data.    
    #[cfg(feature = "with-sql")]
    async fn seed(db: &DatabaseConnection, path: &Path) -> Result<()>;
}

/// An initializer.
/// Initializers should be kept in `src/initializers/`
#[async_trait::async_trait]
pub trait Initializer: Sync + Send {
    /// Associated type for the context
    type Context: Send + Sync;

    /// The initializer name or identifier
    fn name(&self) -> String;

    /// Occurs after the app's `before_run`.
    /// Use this to for one-time initializations, load caches, perform web
    /// hooks, etc.
    async fn before_run(
        &self,
        _app_context: Arc<Box<dyn Context>>,
        _context: Option<Self::Context>,
    ) -> Result<()> {
        Ok(())
    }
}
