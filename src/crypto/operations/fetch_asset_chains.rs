use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

use crate::{crypto::{AssetChains, Crypto}, ApiResponseBody, EversendError, EversendResult};

pub struct FetchAssetChainsParams {
    /// This should be any of the available crypto asset you have access to.
    /// e.g. USDT or USDC or BTC or ETH, etc
    pub coin: String,
}

/// An error returned from [`FetchAssetChains`].
#[derive(Debug, Error)]
pub enum FetchAssetChainsError {}

impl From<FetchAssetChainsError> for EversendError<FetchAssetChainsError> {
    fn from(err: FetchAssetChainsError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct FetchAssetChainsResponse {
    pub chains: AssetChains,
}

/// [Eversend Docs: Fetch Asset Chains](https://eversend.readme.io/reference/fetch-asset-chains)
#[async_trait]
pub trait FetchAssetChains {
    /// Get Asset Chains.
    ///
    /// [Eversend Docs: Fetch Asset Chains](https://eversend.readme.io/reference/fetch-asset-chains)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::crypto::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), FetchAssetChainsError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let chains = eversend
    ///         .crypto()
    ///         .fetch_asset_chains(
    ///             &FetchAssetChainsParams {
    ///                 coin: String::from("USDT"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn fetch_asset_chains(
        &self,
        params: &FetchAssetChainsParams,
    ) -> EversendResult<AssetChains, FetchAssetChainsError>;
}

#[async_trait]
impl<'a> FetchAssetChains for Crypto<'a> {
    async fn fetch_asset_chains(
        &self,
        params: &FetchAssetChainsParams,
    ) -> EversendResult<AssetChains, FetchAssetChainsError> {
        let url = format!("{}/crypto/assets/{}", self.eversend.base_url(), params.coin);

        let result = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<FetchAssetChainsResponse>>()
            .await?;

        Ok(result.data.chains)
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
    async fn it_calls_the_assets_chains_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let coin = String::from("USDT");

        let mock = mock("GET", format!("/crypto/assets/{}", coin).as_str())
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "chains": {
                            "Binance Smart Chain (BEP20)": "USDT_BSC",
                            "Ethereum (ERC20)": "USDT_ERC20",
                            "TRON (TRC20)": "TRX_USDT_S2UZ"
                        }
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let chains = eversend
            .crypto()
            .fetch_asset_chains(
                &FetchAssetChainsParams {
                    coin,
                }
            )
            .await
            .unwrap();

        assert_eq!(chains.binance_smart_chain, "USDT_BSC");

        mock.assert();

    }
}
