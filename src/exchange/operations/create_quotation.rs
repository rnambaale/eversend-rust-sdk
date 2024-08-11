use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{exchange::{types::Quotation, Exchange}, wallets::WalletId, ApiResponseBody, EversendError, EversendResult, ResponseExtension};

#[derive(Serialize)]
pub struct CreateQuotationParams<'a> {
    /// Amount of source currency
    pub amount: String,

    /// Source currency from Get Wallets
    pub from: &'a WalletId,

    /// Destination currency from Get Wallets
    pub to: &'a WalletId,
}

/// An error returned from [`CreateQuotation`].
#[derive(Debug, Error)]
pub enum CreateQuotationError {}

impl From<CreateQuotationError> for EversendError<CreateQuotationError> {
    fn from(err: CreateQuotationError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateQuotationResponse {
    pub expires: String,
    pub token: String,
    pub quotation: Quotation,
}

#[async_trait]
pub trait CreateQuotation {
    /// Creates a [`Quotation`].
    ///
    /// [Eversend Docs: Create a Quotation](https://eversend.readme.io/reference/activate-a-wallet)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::exchange::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> EversendResult<(), CreateQuotationError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let quotation = eversend
    ///         .exchange()
    ///         .create_quotation(&CreateQuotationParams{
    ///             amount: String::from("1000"),
    ///             from: &WalletId::from("UGX"),
    ///             to: &WalletId::from("KES")
    ///         })
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    ///
    /// ```
    ///
    async fn create_quotation(
        &self,
        params: &CreateQuotationParams<'_>
    ) -> EversendResult<CreateQuotationResponse, CreateQuotationError>;
}

#[async_trait]
impl<'a> CreateQuotation for Exchange<'a> {
    async fn create_quotation(
        &self,
        params: &CreateQuotationParams<'_>
    ) -> EversendResult<CreateQuotationResponse, CreateQuotationError> {
        let url = format!("{}/exchanges/quotation", self.eversend.base_url());

        let response = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<ApiResponseBody<CreateQuotationResponse>>()
            .await?;

        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientId, eversend::Eversend, ApiToken, ClientSecret};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_create_quotation_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("POST", "/exchanges/quotation")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data":{
                        "expires":"2022-08-30T16:09:53+00:00",
                        "token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
                        "quotation":{
                            "baseCurrency":"UGX",
                            "baseAmount":100,
                            "baseWalletBefore":498.78,
                            "baseWalletAfter":398.78,
                            "destCurrency":"USD",
                            "destAmount":0.025828573078999998,
                            "destWalletBefore":1.52,
                            "destWalletAfter":null,
                            "rate":0.00025828573079
                        }
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let response = eversend
            .exchange()
            .create_quotation(
                &CreateQuotationParams{
                    amount: String::from("1000"),
                    from: &WalletId::from("UGX"),
                    to: &WalletId::from("KES")
                }
            )
            .await
            .unwrap();

        assert_eq!(response.expires, String::from("2022-08-30T16:09:53+00:00"));
        assert_eq!(response.token, String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."));

        assert_eq!(response.quotation.base_amount, 100);
        assert_eq!(response.quotation.base_currency, WalletId::from("UGX"));
        assert_eq!(response.quotation.dest_currency, WalletId::from("USD"));
    }
}
