use async_trait::async_trait;
use serde::Serialize;

use crate::{exchange::{types::Quotation, Exchange}, wallets::WalletId, ApiResponseBody};

#[derive(Serialize)]
pub struct CreateQuotationParams<'a> {
    /// Amount of source currency
    pub amount: u32,

    /// Source currency from Get Wallets
    pub from: &'a WalletId,

    /// Destination currency from Get Wallets
    pub to: &'a WalletId,
}

#[async_trait]
pub trait CreateQuotation {
    /// Creates a [`Quotation`].
    ///
    /// [Eversend Docs: Create a Quotation](https://eversend.readme.io/reference/activate-a-wallet)
    ///
    /// # Examples
    /// ```
    /// use eversend_rust_sdk::exchange::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let quotation = eversend
    ///         .exchange()
    ///         .create_quotation(&CreateQuotationParams{
    ///             amount: 1000,
    ///             from: &WalletId::from("UGX"),
    ///             to: &WalletId::from("USD")
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
    ) -> Result<Quotation, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> CreateQuotation for Exchange<'a> {
    async fn create_quotation(
        &self,
        params: &CreateQuotationParams<'_>
    ) -> Result<Quotation, Box<dyn std::error::Error>> {
        let url = format!("{}/exchanges/quotation", self.eversend.base_url());

        let response = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<Quotation>>()
            .await?;

        Ok(response.data)
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
    async fn it_calls_the_create_quotation_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("POST", "/exchanges/quotation")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "amount": 1000,
                        "from": "UGX",
                        "id": 1,
                        "to": "USD",
                        "token": "some-quotation-token"
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let quotation = eversend
            .exchange()
            .create_quotation(
                &CreateQuotationParams{
                    amount: 1000,
                    from: &WalletId::from("UGX"),
                    to: &WalletId::from("UGX")
                }
            )
            .await
            .unwrap();

        assert_eq!(quotation.amount, 1000);
        assert_eq!(quotation.from, WalletId::from("UGX"));
        assert_eq!(quotation.to, WalletId::from("USD"));
        assert_eq!(quotation.token, String::from("some-quotation-token"));
    }
}
