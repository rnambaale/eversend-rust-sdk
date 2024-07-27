use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{transactions::{Transaction, Transactions}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct GetTransactionParams {
    /// transactionId from Get Transactions
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

/// An error returned from [`GetTransaction`].
#[derive(Debug, Error)]
pub enum GetTransactionError {
    #[error("could not find transaction in the response")]
    NotFound
}

impl From<GetTransactionError> for EversendError<GetTransactionError> {
    fn from(err: GetTransactionError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct GetTransactionResponse {
    pub transactions: Vec<Transaction>,
}

/// [Eversend Docs: Get Transaction](https://eversend.readme.io/reference/get-transaction)
#[async_trait]
pub trait GetTransaction {
    /// Get Transaction.
    ///
    /// [Eversend Docs: Get Transaction](https://eversend.readme.io/reference/get-transaction)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::transactions::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetTransactionError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let transaction = eversend
    ///         .transactions()
    ///         .get_transaction(
    ///             &GetTransactionParams {
    ///                 transaction_id: String::from("BE11640235387619"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_transaction(
        &self,
        params: &GetTransactionParams
    ) -> EversendResult<Transaction, GetTransactionError>;
}

#[async_trait]
impl<'a> GetTransaction for Transactions<'a> {
    async fn get_transaction(
        &self,
        params: &GetTransactionParams
    ) -> EversendResult<Transaction, GetTransactionError> {
        let url = format!("{}/transactions/{}", self.eversend.base_url(), params.transaction_id);

        let result = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<GetTransactionResponse>>()
            .await?;

        let transaction_result = result.data.transactions.first();

        if let None = transaction_result {
            return Err(EversendError::Operation(GetTransactionError::NotFound));
        }

        Ok(transaction_result.unwrap().clone())
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
    async fn it_calls_the_transactions_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();
        let transaction_id = String::from("BE11640235387619");

        let mock = mock("GET", format!("/transactions/{}", transaction_id).as_str())
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "transactions": [
                            {
                                "id": 792,
                                "transactionId": "BE31661876379861",
                                "transactionRef": null,
                                "type": "exchange",
                                "currency": "UGX",
                                "amount": "100",
                                "fees": null,
                                "balanceBefore": "398.78",
                                "balanceAfter": "398.78",
                                "remitOneId": null,
                                "sourceCurrency": null,
                                "destinationCurrency": "KES",
                                "destinationAmount": "3.1007201981367",
                                "sourceCountry": null,
                                "destinationCountry": null,
                                "pesapotId": null,
                                "pesapotResponse": null,
                                "merchantId": null,
                                "accountId": 3,
                                "userId": null,
                                "beneficiaryId": null,
                                "customer": null,
                                "meta": {
                                    "source": {
                                        "amount": 100,
                                        "balance": {
                                            "after": "398.78",
                                            "before": "398.78"
                                        },
                                        "currency": "UGX"
                                    },
                                    "destination": {
                                        "amount": 3.1,
                                        "balance": {
                                            "after": "1783.82",
                                            "before": "1783.82"
                                        },
                                        "currency": "KES"
                                    }
                                },
                                "reason": null,
                                "isRefunded": false,
                                "status": "successful",
                                "createdAt": "2022-08-30T16:19:39.864Z",
                                "updatedAt": "2022-08-30T16:19:39.864Z",
                                "user": null,
                                "beneficiary": null
                            }
                        ]
                    },
                    "success": true
                  }).to_string(),
            )
            .create();

        let transaction = eversend
            .transactions()
            .get_transaction(
                &GetTransactionParams {
                    transaction_id,
                }
            )
            .await
            .unwrap();

        assert_eq!(transaction.id, 792);

        mock.assert();

    }
}
