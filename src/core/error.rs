use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error kind that represents failures reported by the [`crate::Client`].
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EversendError {
    /// API token was not found on the eversend client.
    ApiTokenMissing,
    OperationError
}

/// Error struct that holds the [`EversendError`] and message of the reported failure.
#[derive(Debug, PartialEq)]
pub struct ClientError {
    /// Error kind that represents failures reported by the [`crate::Client`].
    pub kind: EversendError,
    /// The text representation of the failure.
    pub message: String,
}

impl ClientError {
    pub(crate) fn new(kind: EversendError, message: String) -> Self {
        Self { message, kind }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for ClientError {}
