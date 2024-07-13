use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{beneficiaries::Beneficiaries, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CheckAccountParams {
    /// Email address of user. Optional if phone is provided
    pub email: Option<String>,

    /// Phone number in international format. Optional if email is provided
    pub phone: Option<String>,
}

/// An error returned from [`CheckEversendAccount`].
#[derive(Debug, Error)]
pub enum CheckEversendAccountError {}

impl From<CheckEversendAccountError> for EversendError<CheckEversendAccountError> {
    fn from(err: CheckEversendAccountError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CheckEversendAccountStatus {
    #[serde(rename = "accountExists")]
    pub account_exists: bool
}

/// [Eversend Docs: Check Eversend Account](https://eversend.readme.io/reference/check-eversend-account)
#[async_trait]
pub trait CheckEversendAccount {
    /// Check if Eversend Account exists.
    ///
    /// [Eversend Docs: Check Eversend Account](https://eversend.readme.io/reference/check-eversend-account)
    ///
    /// [!NOTE]
    /// Please note that only one of email or phone should be passed. If both are filled in, only phone will be used.
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CheckEversendAccountError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let _response = eversend
    ///         .beneficiaries()
    ///         .check_eversend_account(
    ///             &CheckAccountParams {
    ///                 email: None,
    ///                 phone: Some(String::from("0789098123")),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn check_eversend_account(
        &self,
        params: &CheckAccountParams
    ) -> EversendResult<bool, CheckEversendAccountError>;
}

#[async_trait]
impl<'a> CheckEversendAccount for Beneficiaries<'a> {
    async fn check_eversend_account(
        &self,
        params: &CheckAccountParams
    ) -> EversendResult<bool, CheckEversendAccountError> {
        let url = format!("{}/beneficiaries/accounts/eversend", self.eversend.base_url());

        let response = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CheckEversendAccountStatus>>()
            .await?;

        Ok(response.data.account_exists)
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::ClientId, eversend::Eversend, ApiToken};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_check_eversend_account_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/beneficiaries/accounts/eversend")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "accountExists": true,
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let account_status = eversend
            .beneficiaries()
            .check_eversend_account(
                &CheckAccountParams {
                    email: None,
                    phone: Some(String::from("0789098123")),
                }
            )
            .await
            .unwrap();

        mock.assert();
        assert_eq!(account_status, true);
    }
}
