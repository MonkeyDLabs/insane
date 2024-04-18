use std::sync::Arc;

use crate::{
    banner::print_banner, config::InsaneConfig, context::{Context, DefaultContext}, environment::Environment, error::Result, hook::Hooks
};
#[cfg(feature = "with-sql")]
use crate::sql;

/// Initializes the application context by loading configuration and
/// establishing connections.
///
/// # Errors
/// When has an error to create DB connection.
pub async fn create_context<H: Hooks>(
    environment: &Environment,
    config: &InsaneConfig,
) -> Result<Arc<Box<dyn Context>>> {
    if config.tracing.pretty_backtrace {
        std::env::set_var("RUST_BACKTRACE", "1");
        tracing::warn!(
            "pretty backtraces are enabled (this is great for development but has a runtime cost \
         for production. disable with `logger.pretty_backtrace` in your config yaml)"
        );
    }

    #[cfg(feature = "with-sql")]
    let sql: sea_orm::prelude::DatabaseConnection = sql::connect(&config.sql).await?;

    // #[cfg(feature = "with-redis")]
    // let redis = connect_redis(&config).await;

    let context: Arc<Box<dyn Context>> = Arc::new(Box::new(DefaultContext {
        environment: environment.clone(),
        config: config.clone(),

        #[cfg(feature = "with-sql")]
        sql,
        // #[cfg(feature = "with-redis")]
        // redis,
        // storage: C::storage(&config, environment).await?.map(Arc::new),
    }));

    Ok(context)
}

/// Boots the application based on the specified mode.
pub async fn boot_app<H: Hooks>(context: Arc<Box<dyn Context>>) -> Result<()> {
    // Global app lifecycle hooks
    H::before_run(context.clone()).await?;
    let initializers = H::initializers(context.clone()).await?;
    tracing::info!(initializers = ?initializers.iter().map(|init| init.name()).collect::<Vec<_>>().join(","), "initializers loaded");
    for initializer in &initializers {
        initializer.before_run(context.clone(), None).await?;
    }

    print_banner(&context);

    let servers = H::servers(context.clone()).await?;
    let serve_futures = servers.into_iter().map(|server| {
        let context = context.clone(); // Clone the context for each server
        tokio::spawn(async move {
            if server.enable(context.clone()).await? {
                server.serve(context).await // Serve the server
            } else {
                tracing::info!(server = server.name(), "is disabled. skipping...");
                Ok(()) // Server is disabled, return Ok(())
            }
        })
    });

    for serve_future in serve_futures {
        // Spawn each serve future concurrently
        if let Err(err) = serve_future.await {
            // Use the cloned context inside the closure
            tracing::error!("Error in processing: {:?}", err);
        }
    }

    Ok(())
}
