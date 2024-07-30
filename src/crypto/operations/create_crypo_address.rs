use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{crypto::{Crypto, CryptoAddress}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateCryptoAddressParams {
    /// Valid asset from Fetch Asset Chains
    #[serde(rename = "assetId")]
    pub asset_id: String,

    /// Client email or unique identifier
    #[serde(rename = "destinationAddressDescription")]
    pub destination_address_description: String,

    /// Client name
    #[serde(rename = "ownerName")]
    pub owner_name: String,

    /// Purpose of transaction
    pub purpose: Option<String>,
}

/// An error returned from [`CreateCryptoAddress`].
#[derive(Debug, Error)]
pub enum CreateCryptoAddressError {}

impl From<CreateCryptoAddressError> for EversendError<CreateCryptoAddressError> {
    fn from(err: CreateCryptoAddressError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateCryptoAddressResponse {
    pub address: CryptoAddress,
}

/// [Eversend Docs: Create Crypto Address](https://eversend.readme.io/reference/create-crypto-address)
#[async_trait]
pub trait CreateCryptoAddress {
    /// Create Crypto Address.
    ///
    /// [Eversend Docs: Create Crypto Address](https://eversend.readme.io/reference/create-crypto-address)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::crypto::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateCryptoAddressError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let address = eversend
    ///         .crypto()
    ///         .create_crypto_address(
    ///             &CreateCryptoAddressParams {
    ///                 asset_id: String::from("TRX_USDT_S2UZ"),
    ///                 destination_address_description: String::from("emmanuelchilaka779@gmail.com"),
    ///                 owner_name: String::from("Emmanuel Chilaka"),
    ///                 purpose: Some(String::from("payment for coffee")),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn create_crypto_address(
        &self,
        params: &CreateCryptoAddressParams,
    ) -> EversendResult<CryptoAddress, CreateCryptoAddressError>;
}

#[async_trait]
impl<'a> CreateCryptoAddress for Crypto<'a> {
    async fn create_crypto_address(
        &self,
        params: &CreateCryptoAddressParams,
    ) -> EversendResult<CryptoAddress, CreateCryptoAddressError> {
        let url = format!("{}/crypto/addresses", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CreateCryptoAddressResponse>>()
            .await?;

        Ok(result.data.address)
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
    async fn it_calls_the_create_address_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/crypto/addresses")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "address": {
                            "address": "TDqYRYfYfq4fdKXoWXrTEMUHBYErtyhgEf",
                            "coin": "TRX_USDT_S2UZ",
                            "purpose": "Payment for coffee",
                            "ownerName": "Emmanuel Chilaka",
                            "destinationAddressDescription": "emmanuelchilaka@gmail.com",
                            "createdAt": "2023-04-06T07:41:20.867Z",
                            "updatedAt": "2023-04-06T07:41:20.868Z"
                        }
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let address = eversend
            .crypto()
            .create_crypto_address(
                &CreateCryptoAddressParams {
                    asset_id: String::from("TRX_USDT_S2UZ"),
                    destination_address_description: String::from("emmanuelchilaka779@gmail.com"),
                    owner_name: String::from("Emmanuel Chilaka"),
                    purpose: Some(String::from("payment for coffee")),
                }
            )
            .await
            .unwrap();

        assert_eq!(address.address, "TDqYRYfYfq4fdKXoWXrTEMUHBYErtyhgEf");

        mock.assert();

    }
}
