use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{transactions::{Transaction, TransactionCurrencyOption, TransactionStatusOption, Transactions, TransactionRangeOption, TransactionTypeOption}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct GetTransactionsParams {
    /// Defaults to UGX
    pub currency: TransactionCurrencyOption,

    /// format YYYY-MM-dd
    pub from: String,

    /// Start from 1. Default value is 10
    pub limit: u32,

    /// Start from 1. Default value is 1
    pub page: u32,

    pub range: TransactionRangeOption,

    /// The transaction reference.
    pub search: String,

    /// Defaults to pending
    #[serde(rename = "status")]
    pub transaction_status: TransactionStatusOption,

    /// format YYYY-MM-dd
    pub to: String,

    /// Defaults to payout
    #[serde(rename = "type")]
    pub transaction_type: TransactionTypeOption,
}

/// An error returned from [`GetTransactions`].
#[derive(Debug, Error)]
pub enum GetTransactionsError {}

impl From<GetTransactionsError> for EversendError<GetTransactionsError> {
    fn from(err: GetTransactionsError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct GetTransactionsResponse {
    pub total_payouts: String,
    pub total_collections: String,
    pub balance: u32,
    pub transactions: Vec<Transaction>,
    pub total: u32,
    pub limit: u32,
    pub page: u32,
}

/// [Eversend Docs: Get Transactions](https://eversend.readme.io/reference/get-transactions)
#[async_trait]
pub trait GetTransactions {
    /// Get Transactions.
    ///
    /// [Eversend Docs: Get Transactions](https://eversend.readme.io/reference/get-transactions)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::transactions::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetTransactionsError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let transactions = eversend
    ///         .transactions()
    ///         .get_transactions(
    ///             &GetTransactionsParams {
    ///                 currency: TransactionCurrencyOption::UGX,
    ///                 from: String::from("2024-01-01"),
    ///                 to: String::from("2024-01-01"),
    ///                 limit: 10,
    ///                 page: 1,
    ///                 range: TransactionRangeOption::MONTH,
    ///                 search: String::from("BE11640235387619"),
    ///                 transaction_status: TransactionStatusOption::PENDING,
    ///                 transaction_type: TransactionTypeOption::PAYOUT,
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_transactions(
        &self,
        params: &GetTransactionsParams
    ) -> EversendResult<Vec<Transaction>, GetTransactionsError>;
}

#[async_trait]
impl<'a> GetTransactions for Transactions<'a> {
    async fn get_transactions(
        &self,
        params: &GetTransactionsParams
    ) -> EversendResult<Vec<Transaction>, GetTransactionsError> {
        let url = format!("{}/transactions", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<GetTransactionsResponse>>()
            .await?;

        Ok(result.data.transactions)
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
    async fn it_calls_the_transactions_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/transactions")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                    "total_payouts": "20000",
                    "total_collections": "19000",
                    "balance": 0,
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
            .transactions()
            .get_transactions(
                &GetTransactionsParams {
                    currency: TransactionCurrencyOption::UGX,
                    from: String::from("2024-01-01"),
                    to: String::from("2024-01-01"),
                    limit: 10,
                    page: 1,
                    range: TransactionRangeOption::MONTH,
                    search: String::from("BE11640235387619"),
                    transaction_status: TransactionStatusOption::PENDING,
                    transaction_type: TransactionTypeOption::PAYOUT,
                }
            )
            .await
            .unwrap();

        assert_eq!(transactions[0].id, 792);

        mock.assert();

    }
}
