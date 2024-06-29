use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{wallets::{Wallet, WalletId, Wallets}, ApiResponseBody};

/// The parameters for [`ActivateWallet`].
#[derive(Debug, Serialize)]
pub struct ActivateWalletParams<'a> {
    /// The ID of the wallet.
    pub wallet: &'a WalletId
}

#[derive(Serialize, Deserialize)]
struct WalletResponseData {
    wallet: Wallet
}

#[async_trait]
pub trait ActivateWallet {
    /// Activates an [`Wallet`].
    ///
    /// [Eversend Docs: Activate a Wallet](https://eversend.readme.io/reference/activate-a-wallet)
    ///
    /// # Examples
    /// ```
    /// use eversend_rust_sdk::wallets::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let wallet = eversend
    ///         .wallets()
    ///         .activate_wallet(&ActivateWalletParams{
    ///             wallet: &WalletId::from("USD")
    ///         })
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    ///
    async fn activate_wallet(
        &self,
        params: &ActivateWalletParams<'_>
    ) -> Result<Wallet, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> ActivateWallet for Wallets<'a> {
    async fn activate_wallet(
        &self,
        params: &ActivateWalletParams<'_>
    ) -> Result<Wallet, Box<dyn std::error::Error>> {
        let url = format!("{}/wallets/activate", self.eversend.base_url());

        let wallet = self
            .eversend
            .client()
            .post(url)
            .json(&params)
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
    async fn it_calls_the_activate_wallet_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("POST", "/wallets/activate")
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
            .activate_wallet(
                &ActivateWalletParams{
                    wallet: &WalletId::from("USD")
                }
            )
            .await
            .unwrap();

        assert_eq!(wallet.currency, WalletId::from("UGX").to_string());
        assert_eq!(wallet.currency_type, "some type");
    }
}
