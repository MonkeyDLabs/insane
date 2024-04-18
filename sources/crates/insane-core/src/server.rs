use std::sync::Arc;

// use crate::context::ServerContext;
use crate::{context::Context, error::Result, hook::Initializer};

#[async_trait::async_trait]
pub trait Server: Sync + Send {
    /// The initializer name or identifier
    fn name(&self) -> String;

    /// Occurs after the app's `before_run`.
    /// Use this to for one-time initializations, load caches, perform web
    /// hooks, etc.
    async fn serve(&self, _app_context: Arc<Box<dyn Context>>) -> Result<()>;


    async fn enable(&self, _context: Arc<Box<dyn Context>>) -> Result<bool> {
        Ok(false)
    }
}

#[async_trait::async_trait]
pub trait ServerLifeCycle: Sync + Send {
    /// Associated type for the context
    type ServerContext: Send + Sync;

    /// Provide a list of initializers
    /// An initializer can be used to seamlessly add functionality to your app
    /// or to initialize some aspects of it.
    async fn initializers(
        &self,
        _app_context: Arc<Box<dyn Context>>,
        _context: Self::ServerContext,
    ) -> Result<Vec<Box<dyn Initializer<Context = Self::ServerContext>>>> {
        Ok(vec![])
    }

    /// Calling the function before run the app
    /// You can now code some custom loading of resources or other things before
    /// the app runs
    async fn before_run(
        &self,
        _app_context: Arc<Box<dyn Context>>,
        _context: Self::ServerContext,
    ) -> Result<()> {
        Ok(())
    }
}
