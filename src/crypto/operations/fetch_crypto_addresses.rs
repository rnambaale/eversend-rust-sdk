use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

use crate::{crypto::{Crypto, CryptoAddress}, ApiResponseBody, EversendError, EversendResult};

/// An error returned from [`FetchCryptoAddresses`].
#[derive(Debug, Error)]
pub enum FetchCryptoAddressesError {}

impl From<FetchCryptoAddressesError> for EversendError<FetchCryptoAddressesError> {
    fn from(err: FetchCryptoAddressesError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct FetchCryptoAddressesResponse {
    pub addresses: Vec<CryptoAddress>,
}

/// [Eversend Docs: Fetch Addresses](https://eversend.readme.io/reference/fetch-address)
#[async_trait]
pub trait FetchCryptoAddresses {
    /// Fetch Addresses.
    ///
    /// [Eversend Docs: Fetch Addresses](https://eversend.readme.io/reference/fetch-address)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::crypto::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), FetchCryptoAddressesError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let addresses = eversend
    ///         .crypto()
    ///         .fetch_crypto_addresses()
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn fetch_crypto_addresses(
        &self
    ) -> EversendResult<Vec<CryptoAddress>, FetchCryptoAddressesError>;
}

#[async_trait]
impl<'a> FetchCryptoAddresses for Crypto<'a> {
    async fn fetch_crypto_addresses(
        &self
    ) -> EversendResult<Vec<CryptoAddress>, FetchCryptoAddressesError> {
        let url = format!("{}/crypto/addresses", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<FetchCryptoAddressesResponse>>()
            .await?;

        Ok(result.data.addresses)
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
    async fn it_calls_the_get_addresses_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("GET", "/crypto/addresses")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "addresses": [
                            {
                                "address": "TDqYRYfYfq4fdKXoWXrTEMUHBYErtyhgEf",
                                "coin": "TRX_USDT_S2UZ",
                                "purpose": "Payment for coffee",
                                "ownerName": "Emmanuel Chilaka",
                                "destinationAddressDescription": "emmanuelchilaka@gmail.com",
                                "createdAt": "2023-04-06T07:41:20.867Z",
                                "updatedAt": "2023-04-06T07:41:20.868Z",
                                "transactions": []
                            }
                        ],
                        "total": 1,
                        "limit": 10,
                        "page": 1
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let addresses = eversend
            .crypto()
            .fetch_crypto_addresses()
            .await
            .unwrap();

        assert_eq!(addresses[0].address, "TDqYRYfYfq4fdKXoWXrTEMUHBYErtyhgEf");

        mock.assert();

    }
}
