use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{wallets::{Wallet, WalletId, Wallets}, ApiResponseBody, EversendError, EversendResult, ResponseExtension};

/// The parameters for [`DeactivateWallet`].
#[derive(Debug, Serialize)]
pub struct DeActivateWalletParams<'a> {
    /// The ID of the wallet e.g. UGX, NGN, etc
    pub wallet: &'a WalletId
}

/// An error returned from [`DeactivateWallet`].
#[derive(Debug, Error)]
pub enum DeactivateWalletError {}

impl From<DeactivateWalletError> for EversendError<DeactivateWalletError> {
    fn from(err: DeactivateWalletError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Serialize, Deserialize)]
struct WalletResponseData {
    wallet: Wallet
}

/// [Eversend Docs: Deactivate a Wallet](https://eversend.readme.io/reference/deactivate-a-wallet)
#[async_trait]
pub trait DeactivateWallet {
    /// Deactivates an [`Wallet`].
    ///
    /// [Eversend Docs: Deactivate a Wallet](https://eversend.readme.io/reference/deactivate-a-wallet)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::wallets::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> EversendResult<(), DeactivateWalletError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let wallet = eversend
    ///         .wallets()
    ///         .deactivate_wallet(&DeActivateWalletParams{
    ///             wallet: &WalletId::from("UGX")
    ///         })
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    ///
    async fn deactivate_wallet(
        &self,
        params: &DeActivateWalletParams<'_>
    ) -> EversendResult<Wallet, DeactivateWalletError>;
}

#[async_trait]
impl<'a> DeactivateWallet for Wallets<'a> {
    async fn deactivate_wallet(
        &self,
        params: &DeActivateWalletParams<'_>
    ) -> EversendResult<Wallet, DeactivateWalletError> {
        let url = format!("{}/wallets/deactivate", self.eversend.base_url());

        let wallet = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<ApiResponseBody<WalletResponseData>>()
            .await?;

        Ok(wallet.data.wallet)
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
    async fn it_calls_the_activate_wallet_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/wallets/deactivate")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "wallet" : {
                            "currency": "UGX",
                            "currencyType": "fiat",
                            "amount": 500,
                            "enabled": false,
                            "name": "Ugandan Shilling",
                            "icon": "https://source.unsplash.com/user/c_v_r/1900x800",
                            "amountInBaseCurrency": 500,
                            "isMain": false,
                        }
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let wallet = eversend
            .wallets()
            .deactivate_wallet(
                &DeActivateWalletParams{
                    wallet: &WalletId::from("UGX")
                }
            )
            .await
            .unwrap();

        assert_eq!(wallet.currency, WalletId::from("UGX").to_string());
        assert_eq!(wallet.currency_type, "fiat");
        mock.assert();
    }
}
