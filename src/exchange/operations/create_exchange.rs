use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::{exchange::{types::Exchange as ExchangeResult, Exchange}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateExchangeParams {
    /// Token from Create Quotation
    pub token: String
}

/// An error returned from [`CreateExchange`].
#[derive(Debug, Error)]
pub enum CreateExchangeError {}

impl From<CreateExchangeError> for EversendError<CreateExchangeError> {
    fn from(err: CreateExchangeError) -> Self {
        Self::Operation(err)
    }
}

#[async_trait]
pub trait CreateExchange {
    /// Creates an [`Exchange`].
    ///
    /// [Eversend Docs: Create Exchange](https://eversend.readme.io/reference/create-exchange)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::exchange::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> EversendResult<(), CreateExchangeError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let exchange = eversend
    ///         .exchange()
    ///         .create_exchange(&CreateExchangeParams{
    ///             quotation_token: String::from("some-test-quotation-token")
    ///         })
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    ///
    /// ```
    ///
    async fn create_exchange(
        &self,
        params: &CreateExchangeParams
    ) -> EversendResult<ExchangeResult, CreateExchangeError>;
}

#[async_trait]
impl<'a> CreateExchange for Exchange<'a> {
    async fn create_exchange(
        &self,
        params: &CreateExchangeParams
    ) -> EversendResult<ExchangeResult, CreateExchangeError> {
        let url = format!("{}/exchanges", self.eversend.base_url());

        let response = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<ExchangeResult>>()
            .await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientId, eversend::Eversend, wallets::WalletId, ApiToken, ClientSecret};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_create_echange_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("POST", "/exchanges")
            .with_status(200)
            .with_body(
                json!({
                    "code": 201,
                    "data": {
                        "source": {
                            "currency": "UGX",
                            "amount": 100,
                            "balance": {
                                "before": "398.78",
                                "after": "398.78"
                            }
                        },
                        "destination": {
                            "currency": "KES",
                            "amount": 3.1,
                            "balance": {
                                "before": "1783.82",
                                "after": "1783.82"
                            }
                        },
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let exchange = eversend
            .exchange()
            .create_exchange(
                &CreateExchangeParams{
                    token: String::from("some-test-token"),
                }
            )
            .await
            .unwrap();

        // Source ...
        assert_eq!(exchange.source.amount, 100 as f64);
        assert_eq!(exchange.source.currency, WalletId::from("UGX"));
        assert_eq!(exchange.source.balance.before, String::from("398.78"));
        assert_eq!(exchange.source.balance.after, String::from("398.78"));

        // Destination ...
        assert_eq!(exchange.destination.amount, 3.1);
        assert_eq!(exchange.destination.currency, WalletId::from("KES"));
        assert_eq!(exchange.destination.balance.before, String::from("1783.82"));
        assert_eq!(exchange.destination.balance.after, String::from("1783.82"));
    }
}
