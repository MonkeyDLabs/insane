use crate::{
    config::HTTPServerConfig,
    context::HttpContext,
    error::{Error, Result},
    hook::HttpHooks,
};
use axum::Router as AxumRouter;
use insane_core::{
    context::Context,
    error::{Error as CoreError, Result as CoreResult},
    hook::Initializer,
    server::{Server, ServerLifeCycle},
};
use std::sync::Arc;
use tokio::sync::Mutex;

// /// Configuration structure for serving an application.
// pub struct ServerParams {
//     /// The port number on which the server will listen for incoming
//     /// connections.
//     pub port: i32,
//     /// The network address to which the server will bind. It specifies the
//     /// interface to listen on.
//     pub binding: String,
// }

// use crate::context::HttpContext;

// #[derive(Clone)]
pub struct HttpServer<H: HttpHooks> {
    pub hooks: H,
    pub config: Arc<Mutex<Option<HTTPServerConfig>>>,
}

impl<H: HttpHooks> HttpServer<H> {
    /// Create a new instance of the server
    pub fn new(hooks: H) -> Self {
        HttpServer {
            hooks,
            config: Arc::new(Mutex::new(None)),
        }
    }
}

impl<H: HttpHooks> HttpServer<H> {
    /// Start serving the Axum web application on the specified address and
    /// port.
    ///
    /// # Returns
    /// A Result indicating success () or an error if the server fails to start.
    async fn start(http: AxumRouter, http_config: HTTPServerConfig) -> Result<()> {
        let listener =
            tokio::net::TcpListener::bind(&format!("{}:{}", http_config.binding, http_config.port))
                .await?;

        axum::serve(listener, http).await?;

        Ok(())
    }

    /// Asynchronously loads and provides access to the HTTP server configuration.
    ///
    /// This function checks if the HTTP server configuration has already been loaded. If it has,
    /// it returns a clone of the cached configuration. If not, it loads the configuration using
    /// the provided context, caches it for future use, and returns it.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the current instance of the struct.
    /// * `context` - An Arc<Box<dyn Context>> containing the context necessary for loading the configuration.
    ///
    /// # Returns
    ///
    /// A Result containing the loaded HTTP server configuration on success, or an Error if loading fails.
    /// If the configuration has already been loaded, the cached configuration is returned without reloading.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::{Arc, Mutex};
    /// # use your_module::{Context, HTTPServerConfig};
    /// # async fn example_function(server: &Mutex<YourServer>, context: Arc<Box<dyn Context>>) {
    /// let config = server.lock().await.config(context).await;
    /// match config {
    ///     Ok(config) => println!("Loaded HTTP server config: {:?}", config),
    ///     Err(err) => eprintln!("Failed to load HTTP server config: {}", err),
    /// }
    /// # }
    /// ```
    async fn config(&self, context: Arc<Box<dyn Context>>) -> Result<HTTPServerConfig> {
        // Acquire the lock once
        let mut guard = self.config.lock().await;

        // Check if config is already loaded
        if let Some(config) = &*guard {
            return Ok(config.clone());
        }

        // Load config if not already loaded
        let config = HTTPServerConfig::load_from_env(
            &context.environment(),
            &context.config().application_name,
        )
        .map_err(|e| Error::msg(e).bt())?;

        // Update config and return
        *guard = Some(config.clone());
        Ok(config)
    }
}

#[async_trait::async_trait]
impl<H: HttpHooks> Server for HttpServer<H> {
    fn name(&self) -> String {
        "http_server".to_string()
    }

    async fn enable(&self, context: Arc<Box<dyn Context>>) -> CoreResult<bool> {
        let http_config = self
            .config(context.clone())
            .await
            .map_err(|e| CoreError::msg(e).bt())?;

        Ok(http_config.enable)
    }

    async fn serve(&self, context: Arc<Box<dyn Context>>) -> CoreResult<()> {
        let http_config = self
            .config(context.clone())
            .await
            .map_err(|e| CoreError::msg(e).bt())?;

        let http_context = HttpContext::new(http_config.clone(), context.clone());
        let htt_context_boxed = Arc::new(Box::new(http_context.clone()));

        self.before_run(context.clone(), htt_context_boxed.clone())
            .await?;

        let initializers = self
            .initializers(context.clone(), htt_context_boxed.clone())
            .await?;
        tracing::info!(initializers = ?initializers.iter().map(|init| init.name()).collect::<Vec<_>>().join(","), "server initializers loaded");
        for initializer in &initializers {
            initializer
                .before_run(context.clone(), Some(htt_context_boxed.clone()))
                .await?;
        }

        let app = self
            .hooks
            .routes(&http_context, &context)
            .to_router(http_context.clone(), context.clone()) // Unwrap the Arc<Box<HttpContext>> to get the inner HttpContext object
            .map_err(|e| CoreError::msg(e).bt())?;

        let mut router = self
            .hooks
            .after_routes(app, &http_context)
            .await
            .map_err(|e| CoreError::msg(e).bt())?;

        router = self
            .hooks
            .after_routes(router, &http_context)
            .await
            .map_err(|e| CoreError::msg(e).bt())?;

        tracing::info!("{} listening on {}:{}", self.name(), http_config.binding, http_config.port);
        HttpServer::<H>::start(router, http_config)
            .await
            .map_err(|e| CoreError::msg(e).bt())?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<H: HttpHooks> ServerLifeCycle for HttpServer<H> {
    type ServerContext = Arc<Box<HttpContext>>;

    async fn initializers(
        &self,
        _app_context: Arc<Box<dyn Context>>,
        _context: Self::ServerContext,
    ) -> CoreResult<Vec<Box<dyn Initializer<Context = Self::ServerContext>>>> {
        Ok(vec![])
    }

    async fn before_run(
        &self,
        _app_context: Arc<Box<dyn Context>>,
        _context: Self::ServerContext,
    ) -> CoreResult<()> {
        Ok(())
    }
}
