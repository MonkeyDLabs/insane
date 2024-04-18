pub mod hook;
pub mod error;
pub mod context;
pub mod routes;
pub mod http_routes;
pub mod describe;
pub mod middlewares;
pub mod ping;
pub mod health;
pub mod format;
pub mod config;
pub mod server;

use axum::extract::FromRequest;
use axum::response::IntoResponse;
use serde::Serialize;

use error::{Error, Result};

pub mod prelude {
  pub use async_trait::async_trait;
  pub use axum::{
      extract::{Form, Path, State},
      response::{IntoResponse, Response},
      routing::{delete, get, post, put},
  };
  pub use axum_extra::extract::cookie;
}


/// Create an unauthorized error with a specified message.
///
/// This function is used to generate an `Error::Unauthorized` variant with a
/// custom message.
///
/// # Errors
///
/// returns unauthorized enum
///
/// # Example
///
/// ```rust
/// use loco_rs::prelude::*;
///
/// async fn login() -> Result<Response> {
///     let valid = false;
///     if !valid {
///         return unauthorized("unauthorized access");
///     }
///     format::json(())
/// }
/// ````
pub fn unauthorized<T: Into<String>, U>(msg: T) -> Result<U> {
    Err(Error::Unauthorized(msg.into()))
}

/// Return a bad request with a message
///
/// # Errors
///
/// This function will return an error result
pub fn bad_request<T: Into<String>, U>(msg: T) -> Result<U> {
    Err(Error::BadRequest(msg.into()))
}

/// return not found status code
///
/// # Errors
/// Currently this function did't return any error. this is for feature
/// functionality
pub fn not_found<T>() -> Result<T> {
    Err(Error::NotFound)
}

#[derive(Debug, FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

