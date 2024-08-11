use reqwest::{Response, StatusCode};

use crate::EversendError;

use super::EversendResult;

pub trait ResponseExtension
where
    Self: Sized,
{
    /// Handles an unauthorized error from the Eversend API by converting it into a
    /// [`EversendError::Unauthorized`] response.
    fn handle_unauthorized_error<E>(self) -> EversendResult<Self, E>;

    /// Handles a generic error from the Eversend API by converting it into a
    /// [`EversendError::RequestError`] response.
    fn handle_generic_error<E>(self) -> EversendResult<Self, E>;

    /// Handles an unauthorized or generic error from the Eversend API.
    fn handle_unauthorized_or_generic_error<E>(self) -> EversendResult<Self, E>;
}

impl ResponseExtension for Response {
    fn handle_unauthorized_error<E>(self) -> EversendResult<Self, E> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(EversendError::Unauthorized)
        } else {
            Ok(self)
        }
    }

    fn handle_generic_error<E>(self) -> EversendResult<Self, E> {
        match self.error_for_status() {
            Ok(response) => Ok(response),
            Err(err) => Err(EversendError::RequestError(err)),
        }
    }

    fn handle_unauthorized_or_generic_error<E>(self) -> EversendResult<Self, E> {
        self.handle_unauthorized_error()?.handle_generic_error()
    }
}
