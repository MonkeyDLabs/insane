use async_trait::async_trait;
// use insane_core::context::Context;
// use insane_core::error::Result;
// use insane_core::hook::Hooks;
// use insane_http::context::HttpContext;
// use insane_core::server::Server;
// use insane_http::hook::HttpHooks;

use insane_core::prelude::*;
use std::path::Path;

use crate::http::HttpApp;
use insane_http::server::HttpServer;
use std::sync::Arc;

pub struct App;

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn truncate(_db: &DatabaseConnection) -> Result<()> {
        Ok(())
    }

    async fn seed(_db: &DatabaseConnection, _path: &Path) -> Result<()> {
        Ok(())
    }

    async fn servers(_ctx: Arc<Box<dyn Context>>) -> Result<Vec<Box<dyn Server>>> {
        let http_server = HttpServer::<HttpApp>::new(HttpApp);
        let http_server = Box::new(http_server);
        // You can add more servers here
        Ok(vec![http_server])
    }
}
