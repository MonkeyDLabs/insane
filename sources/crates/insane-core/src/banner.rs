use std::sync::Arc;

use colored::Colorize;

use crate::context::Context;

// use crate::boot_loader::{BootResult, ServerParams};
/// Configuration structure for serving an application.
pub struct ServerParams {
    /// The port number on which the server will listen for incoming
    /// connections.
    pub port: i32,
    /// The network address to which the server will bind. It specifies the
    /// interface to listen on.
    pub binding: String,
  }
  
//   pub struct BootResult {
//     /// Context
//     pub context: Box<dyn Context>,
//     // / Web server routes
//     // pub router: Option<Router>,
//     // /// worker processor
//     // pub processor: Option<Processor>,
//   }
  

pub const BANNER: &str = r"
                      ▄     ▀                     
                                 ▀  ▄             
                  ▄       ▀     ▄  ▄ ▄▀           
                                    ▄ ▀▄▄         
                        ▄     ▀    ▀  ▀▄▀█▄       
                                          ▀█▄     
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█    
 ██████  █████   ███ █████   ███ █████   ███ ▀█   
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄  
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀  
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀    
                https://loco.rs
";

pub fn print_banner(ctx: &Arc<Box<dyn Context>>) {
    println!("{BANNER}");
    let config = ctx.config();

    println!("environment: {}", ctx.environment().to_string().green());

    #[cfg(feature = "with-sql")]
    {
        let mut database = Vec::new();
        if config.sql.enable_logging.unwrap_or(false) {
            database.push("logging".green());
        }
        if config.sql.auto_migrate.unwrap_or(false) {
            database.push("automigrate".yellow());
        }
        if config.sql.dangerously_recreate.unwrap_or(false) {
            database.push("recreate".bright_red());
        }
        if config.sql.dangerously_truncate.unwrap_or(false) {
            database.push("truncate".bright_red());
        }

        if !database.is_empty() {
            println!(
                "   database: {}",
                database
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    println!("     logger: {}", config.tracing.filter.to_string().green());
    if cfg!(debug_assertions) {
        println!("compilation: {}", "debug".bright_red());
    } else {
        println!("compilation: {}", "release".green());
    }

    // let mut modes = Vec::new();
    // let mut servingline = Vec::new();
    // if boot_result.router.is_some() {
    //     modes.push("server".green());
    //     servingline.push(format!(
    //         "listening on {}:{}",
    //         server_config.binding.to_string().green(),
    //         server_config.port.to_string().green()
    //     ));
    // }
    // if boot_result.processor.is_some() {
    //     modes.push("worker".green());
    //     servingline.push(format!("worker is {}", "online".green()));
    // }
    // if !modes.is_empty() {
    //     println!(
    //         "      modes: {}",
    //         modes
    //             .iter()
    //             .map(ToString::to_string)
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     );
    // }

    // println!();
    // println!("{}", servingline.join("\n"));
}
