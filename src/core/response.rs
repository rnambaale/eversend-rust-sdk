use reqwest::{Response, StatusCode};

use crate::{ClientError, EversendError};

pub trait ResponseExtension
where
    Self: Sized,
{
    /// Handles an unauthorized error from the Eversend API by converting it into a
    /// [`EversendError::Unauthorized`] response.
    fn handle_unauthorized_error(self) -> Result<Self, ClientError>;

    /// Handles a generic error from the Eversend API by converting it into a
    /// [`EversendError::RequestError`] response.
    fn handle_generic_error(self) -> Result<Self, ClientError>;

    /// Handles an unauthorized or generic error from the Eversend API.
    fn handle_unauthorized_or_generic_error(self) -> Result<Self, ClientError>;
}

impl ResponseExtension for Response {
    fn handle_unauthorized_error(self) -> Result<Self, ClientError> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(
                ClientError {
                    kind: EversendError::Unauthorized,
                    message: "Unauthorized".to_owned(),
                }
            )
        } else {
            Ok(self)
        }
    }

    fn handle_generic_error(self) -> Result<Self, ClientError> {
        match self.error_for_status() {
            Ok(response) => Ok(response),
            Err(err) => Err(
                ClientError {
                    kind: EversendError::RequestError,
                    message: err.to_string()
                }
            ),
        }
    }

    fn handle_unauthorized_or_generic_error(self) -> Result<Self, ClientError> {
        self.handle_unauthorized_error()?.handle_generic_error()
    }
}
