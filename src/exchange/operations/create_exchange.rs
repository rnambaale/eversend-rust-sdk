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
    /// use eversend_rust_sdk::{ClientId,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> EversendResult<(), CreateExchangeError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let exchange = eversend
    ///         .exchange()
    ///         .create_exchange(&CreateExchangeParams{
    ///             token: String::from("some-test-token")
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
    use crate::{core::ClientId, eversend::Eversend, wallets::WalletId, ApiToken};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_create_echange_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("POST", "/exchanges")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "source": {
                            "amount": 1000,
                            "currency": "UGX",
                            "balance": {
                                "before": "2000",
                                "after": "2000"
                            }
                        },
                        "destination": {
                            "amount": 10000,
                            "currency": "USD",
                            "balance": {
                                "before": "2000",
                                "after": "2000"
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
        assert_eq!(exchange.source.amount, 1000);
        assert_eq!(exchange.source.currency, WalletId::from("UGX"));
        assert_eq!(exchange.source.balance.before, String::from("2000"));
        assert_eq!(exchange.source.balance.after, String::from("2000"));

        // Destination ...
        assert_eq!(exchange.destination.amount, 10000);
        assert_eq!(exchange.destination.currency, WalletId::from("USD"));
        assert_eq!(exchange.destination.balance.before, String::from("2000"));
        assert_eq!(exchange.destination.balance.after, String::from("2000"));
    }
}
