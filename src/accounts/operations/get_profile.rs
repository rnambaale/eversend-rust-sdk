use async_trait::async_trait;
use thiserror::Error;

use crate::{accounts::{types::Account, Accounts}, ApiResponseBody, EversendError, EversendResult, ResponseExtension};

/// An error returned from [`GetProfile`].
#[derive(Debug, Error)]
pub enum GetProfileError {}

impl From<GetProfileError> for EversendError<GetProfileError> {
    fn from(err: GetProfileError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: Get Account Profile](https://eversend.readme.io/reference/get-account-profile)
#[async_trait]
pub trait GetProfile {
    /// Gets Account Profile.
    ///
    /// [Eversend Docs: Get Account Profile](https://eversend.readme.io/reference/get-account-profile)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::accounts::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetProfileError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let wallet = eversend
    ///         .accounts()
    ///         .get_profile()
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_profile(&self) -> EversendResult<Account, GetProfileError>;
}


#[async_trait]
impl<'a> GetProfile for Accounts<'a> {
    async fn get_profile(&self) -> EversendResult<Account, GetProfileError> {
        let url = format!("{}/account", self.eversend.base_url());

        let account_response = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<ApiResponseBody<Account>>()
            .await?;

        Ok(account_response.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApiToken, ClientId, ClientSecret, eversend::Eversend};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_get_profile_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("GET", "/account")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "id": 3,
                        "name": "Eversend",
                        "email": "frank@eversend.co",
                        "phone": "256759429721",
                        "address": "14-18 Cooper Road",
                        "town": "Kampala",
                        "country": "UG",
                        "logo": null,
                        "website": "http://eversend.co",
                        "isVerified": true
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let account = eversend
            .accounts()
            .get_profile()
            .await
            .unwrap();

        assert_eq!(account.name, "Eversend");
    }
}
