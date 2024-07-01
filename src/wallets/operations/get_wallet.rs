use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{wallets::{types::{Wallet, WalletId}, Wallets}, ApiResponseBody};

#[derive(Serialize, Deserialize)]
struct WalletResponseData {
    wallet: Wallet
}

/// [Eversend Docs: Get a Wallet](https://eversend.readme.io/reference/get-wallet)
#[async_trait]
pub trait GetWallet {
    /// Retrieves a [`Wallet`] by its ID.
    ///
    /// [Eversend Docs: Get a Wallet](https://eversend.readme.io/reference/get-wallet)
    ///
    /// # Examples
    /// ```
    /// use eversend_rust_sdk::wallets::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let wallet = eversend
    ///         .wallets()
    ///         .get_wallet(&WalletId::from("UGX"))
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_wallet(
        &self,
        wallet_id: &WalletId,
    ) -> Result<Wallet, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> GetWallet for Wallets<'a> {
    async fn get_wallet(
        &self,
        wallet_id: &WalletId,
    ) -> Result<Wallet, Box<dyn std::error::Error>> {
        let url = format!("{}/wallets/{}", self.eversend.base_url(), wallet_id);

        let wallet = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<WalletResponseData>>()
            .await?;

        Ok(wallet.data.wallet)
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
    async fn it_calls_the_get_wallet_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("GET", "/wallets/UGX")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "wallet" : {
                            "currency": "UGX",
                            "currencyType": "some type",
                            "amount": 1000,
                            "enabled": true,
                            "name": "Ug Wallet",
                            "icon": "ug-flag",
                            "amountInBaseCurrency": 1000,
                            "isMain": true,
                        }
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let wallet = eversend
            .wallets()
            .get_wallet(&WalletId::from("UGX"))
            .await
            .unwrap();

        assert_eq!(wallet.currency, WalletId::from("UGX").to_string());
        assert_eq!(wallet.currency_type, "some type");
    }
}
