pub mod routes;

use async_trait::async_trait;
// use insane_core::{config::InsaneConfig, context::Context, error::Result};
use insane_http::{context::HttpContext, hook::HttpHooks, http_routes::HttpRoutes};
// use insane_core::server::Server;
use insane_core::prelude::*;

pub struct HttpApp;

#[async_trait]
impl HttpHooks for HttpApp {
    fn routes(&self, _ctx: &HttpContext, _context: &Box<dyn Context>) -> HttpRoutes {
        HttpRoutes::with_default_routes()
            .prefix("/api")
            .add_route(routes::routes())
    }
}

// #[async_trait::async_trait]
// impl Server for Http {
//   fn name(&self) -> String {
//     "http_server".to_string()
//   }

//   async fn serve(&self, _context: Box<dyn Context>) -> Result<()> {
//     println!("Http server started");
//     let _ = Http::routes(&_context);
//     Ok(())
//   }
// }
