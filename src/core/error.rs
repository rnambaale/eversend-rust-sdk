use thiserror::Error;

/// An Eversend SDK error.
#[derive(Debug, Error)]
pub enum EversendError<E> {
    /// API token was not found on the eversend client.
    #[error("API key could not be found")]
    ApiTokenMissing,

    /// An unauthorized response was received from the Eversend API.
    #[error("unauthorized")]
    Unauthorized,

    #[error("operational error")]
    Operation(E),

    /// An unhandled error occurred with the API request.
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
}

/// A Eversend SDK result.
pub type EversendResult<T, E> = Result<T, EversendError<E>>;
