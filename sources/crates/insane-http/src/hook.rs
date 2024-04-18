
use crate::error::Result;
use crate::context::HttpContext;
use crate::http_routes::HttpRoutes;
// use crate::server::HttpServer;
use axum::Router as AxumRouter;
// use insane_core::{context::Context, error::Result as CoreResult, server::Server};

use insane_core::prelude::*;

#[async_trait::async_trait]
pub trait HttpHooks: Sync + Send {
    /// Defines the application's routing configuration.
    fn routes(&self, _ctx: &HttpContext, _context: &Box<dyn Context>) -> HttpRoutes;
    // fn routes(&self, _context: Box<dyn Context>) -> HttpRoutes;

    /// Invoke this function after the Loco routers have been constructed. This
    /// function enables you to configure custom Axum logics, such as layers,
    /// that are compatible with Axum.
    ///
    /// # Errors
    /// Axum router error
    async fn after_routes(&self, router: AxumRouter, _ctx: &HttpContext) -> Result<AxumRouter> {
        Ok(router)
    }

    // /// Create a new instance of the server
    // async fn new(hooks: Self) -> CoreResult<Box<dyn Server>> {
    //     Ok(Box::new(HttpServer{ hooks: Self }))
    // }
}
