//! # Application Error Handling

/// Application results options list
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    JSON(serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}