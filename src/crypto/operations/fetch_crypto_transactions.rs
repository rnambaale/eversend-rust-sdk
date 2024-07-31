use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

use crate::{crypto::{Crypto, CryptoTransaction}, ApiResponseBody, EversendError, EversendResult};

/// An error returned from [`FetchCryptoTransactions`].
#[derive(Debug, Error)]
pub enum FetchCryptoTransactionsError {}

impl From<FetchCryptoTransactionsError> for EversendError<FetchCryptoTransactionsError> {
    fn from(err: FetchCryptoTransactionsError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct FetchCryptoTransactionsResponse {
    pub transactions: Vec<CryptoTransaction>,
}

/// [Eversend Docs: Fetch Transactions](https://eversend.readme.io/reference/fetch-transactions)
#[async_trait]
pub trait FetchCryptoTransactions {
    /// Fetch Transactions.
    ///
    /// [Eversend Docs: Fetch Transactions](https://eversend.readme.io/reference/fetch-transactions)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::crypto::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), FetchCryptoTransactionsError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let transactions = eversend
    ///         .crypto()
    ///         .fetch_crypto_transactions()
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn fetch_crypto_transactions(
        &self
    ) -> EversendResult<Vec<CryptoTransaction>, FetchCryptoTransactionsError>;
}

#[async_trait]
impl<'a> FetchCryptoTransactions for Crypto<'a> {
    async fn fetch_crypto_transactions(
        &self
    ) -> EversendResult<Vec<CryptoTransaction>, FetchCryptoTransactionsError> {
        let url = format!("{}/crypto/transactions", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<FetchCryptoTransactionsResponse>>()
            .await?;

        Ok(result.data.transactions)
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
    async fn it_calls_the_get_transactions_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("GET", "/crypto/transactions")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "transactions": [
                            {
                                "id": 2,
                                "transactionId": "BP11666178904722",
                                "accountId": 1,
                                "addressId": 11,
                                "amount": "10",
                                "meta": {
                                    "date": "2022-10-18",
                                    "fees": "0.00",
                                    "type": "Blockchain",
                                    "amount": 10,
                                    "source": "TEduPZAcEA3GB8UGP3xuEE7i4ZvytK49PQ",
                                    "charges": 0,
                                    "country": "NG",
                                    "currency": "XDC",
                                    "toppedUp": true,
                                    "username": "eversendb2btest@gmail.com",
                                    "processor": "Fireblocks",
                                    "actualCoin": "TRX_USDC_6NU3",
                                    "totalToPay": 10,
                                    "eversendRef": "3557276",
                                    "creationDate": "2022-10-18T13:13:47+00:00",
                                    "fireblocksId": "6f014008-47bd-422a-9a26-25f32f79dc60",
                                    "toppedUpDate": "2022-10-18T13:13:48+00:00",
                                    "blockchainHash": "972cca5c28496d8f82c3a5d0fdc02110a762a25c1823d76275901554f8752726",
                                    "blockchainStatus": "CONFIRMING",
                                    "blockchainSubStatus": "PENDING_BLOCKCHAIN_CONFIRMATIONS"
                                },
                                "status": "CONFIRMING",
                                "subStatus": "PENDING_BLOCKCHAIN_CONFIRMATIONS",
                                "createdAt": "2022-10-19T11:28:26.480Z",
                                "updatedAt": "2022-10-19T11:28:26.481Z",
                                "address": {
                                    "address": "TEduPZAcEA3GB8UGP3xuEE7i4ZvytK49PQ",
                                    "coin": "TRX_USDC_6NU3",
                                    "purpose": "Payment for coffee",
                                    "ownerName": "Emmanuel Chilaka",
                                    "destinationAddressDescription": "emmanuel@eversend.co",
                                    "createdAt": "2022-10-18T12:58:51.093Z",
                                    "updatedAt": "2022-10-18T12:58:51.093Z",
                                }
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

        let transactions = eversend
            .crypto()
            .fetch_crypto_transactions()
            .await
            .unwrap();

        assert_eq!(transactions[0].transaction_id, "BP11666178904722");

        mock.assert();

    }
}
