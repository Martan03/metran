use std::borrow::Cow;

use thiserror::Error;

/// Wrapper for all the error types.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error("{0}")]
    String(Cow<'static, str>),
}
