use async_trait::async_trait;
use thiserror::Error;
use crate::{wallets::{types::Wallet, Wallets}, ApiResponseBody, EversendError, EversendResult};

/// An error returned from [`GetWallets`].
#[derive(Debug, Error)]
pub enum GetWalletsError {}

impl From<GetWalletsError> for EversendError<GetWalletsError> {
    fn from(err: GetWalletsError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: List Wallets](https://eversend.readme.io/reference/get-wallets)
#[async_trait]
pub trait GetWallets {
    /// Retrieves a list of [`Wallet`]s.
    ///
    /// [Eversend Docs: List Wallets](https://eversend.readme.io/reference/get-wallets)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::wallets::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetWalletsError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let wallets = eversend
    ///         .wallets()
    ///         .get_wallets()
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_wallets(
        &self,
    ) -> EversendResult<ApiResponseBody<Vec<Wallet>>, GetWalletsError>;
}

#[async_trait]
impl<'a> GetWallets for Wallets<'a> {
    async fn get_wallets(
        &self,
    ) -> EversendResult<ApiResponseBody<Vec<Wallet>>, GetWalletsError> {
        let url = format!("{}/wallets", self.eversend.base_url());
        let wallets = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<Vec<Wallet>>>()
            .await?;

        Ok(wallets)
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
    async fn it_calls_the_get_wallets_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("GET", "/wallets")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": [
                        {
                            "currency": "UGX",
                            "currencyType": "some type",
                            "amount": 1000,
                            "enabled": true,
                            "name": "Ug Wallet",
                            "icon": "ug-flag",
                            "amountInBaseCurrency": 1000,
                            "isMain": true,
                        },
                        {
                            "currency": "NGN",
                            "currencyType": "some type",
                            "amount": 800,
                            "enabled": true,
                            "name": "Ng Wallet",
                            "icon": "ng-flag",
                            "amountInBaseCurrency": 800,
                            "isMain": false,
                        }
                    ]
                }).to_string(),
            )
            .create();

        let wallets_response = eversend
            .wallets()
            .get_wallets()
            .await
            .unwrap();

        assert_eq!(wallets_response.data[0].currency, "UGX");
        assert_eq!(wallets_response.data[0].amount_in_base_currency, 1000);

        assert_eq!(wallets_response.data[1].currency, "NGN");
        assert_eq!(wallets_response.data[1].amount_in_base_currency, 800);

    }
}
