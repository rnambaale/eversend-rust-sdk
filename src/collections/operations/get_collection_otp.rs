use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{collections::Collections, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct GetCollectionOtpParams {
    /// Phone number in international format
    #[serde(rename = "phone")]
    pub phone_number: String,
}

/// An error returned from [`GetCollectionOtp`].
#[derive(Debug, Error)]
pub enum GetCollectionOtpError {}

impl From<GetCollectionOtpError> for EversendError<GetCollectionOtpError> {
    fn from(err: GetCollectionOtpError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct GetCollectionOtpResponse {
    #[serde(rename = "pinId")]
    pub pin_id: String,
}

/// [Eversend Docs: Get Collection OTP](https://eversend.readme.io/reference/get-collection-otp)
#[async_trait]
pub trait GetCollectionOtp {
    /// Get Collection OTP.
    ///
    /// [Eversend Docs: Get Collection OTP](https://eversend.readme.io/reference/get-collection-otp)
    ///
    /// This endpoint is used to verify phone numbers before collections. In a case where you already have
    /// a system in place to verify phone numbers in your application(KYC), we can whitelist your account
    /// so you don't have to use this endpoint again
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::collections::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetCollectionOtpError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let otp = eversend
    ///         .collections()
    ///         .get_collection_otp(
    ///             &GetCollectionOtpParams {
    ///                 phone_number: String::from("+256712345678"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_collection_otp(
        &self,
        params: &GetCollectionOtpParams
    ) -> EversendResult<String, GetCollectionOtpError>;
}

#[async_trait]
impl<'a> GetCollectionOtp for Collections<'a> {
    async fn get_collection_otp(
        &self,
        params: &GetCollectionOtpParams
    ) -> EversendResult<String, GetCollectionOtpError> {
        let url = format!("{}/collections/otp", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<GetCollectionOtpResponse>>()
            .await?;

        Ok(result.data.pin_id)
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
    async fn it_calls_the_collection_otp_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/collections/otp")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "pinId": "VE2ae72f9b45cc37603095f7795f03c84d"
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let otp = eversend
            .collections()
            .get_collection_otp(
                &GetCollectionOtpParams {
                    phone_number: String::from("+256712345678"),
                }
            )
            .await
            .unwrap();

        assert_eq!(otp, "VE2ae72f9b45cc37603095f7795f03c84d");

        mock.assert();

    }
}
