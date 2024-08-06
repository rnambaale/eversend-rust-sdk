use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{payouts::{Payouts, Transaction}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateEversendPayoutTransactionBodyParams {
    /// JWT token from quotation
    pub token: String,

    /// Optional unique alphanumeric string set by the client
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
}

/// An error returned from [`CreateEversendPayoutTransaction`].
#[derive(Debug, Error)]
pub enum CreateEversendPayoutTransactionError {}

impl From<CreateEversendPayoutTransactionError> for EversendError<CreateEversendPayoutTransactionError> {
    fn from(err: CreateEversendPayoutTransactionError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateEversendPayoutTransactionResponse {
    transaction: Transaction
}

// [Eversend Docs: Create Payout Transaction Eversend](https://eversend.readme.io/reference/create-payout-transaction-eversend)
#[async_trait]
pub trait CreateEversendPayoutTransaction {
    /// Create a [`Transaction`].
    ///
    /// [Eversend Docs: Create Payout Transaction Eversend](https://eversend.readme.io/reference/create-payout-transaction-eversend)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateEversendPayoutTransactionError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let transaction = eversend
    ///         .payouts()
    ///         .create_eversend_payout_transaction(
    ///             &CreateEversendPayoutTransactionBodyParams {
    ///                 token: String::from("some-token"),
    ///                 transaction_ref: String::from("some-reference"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn create_eversend_payout_transaction(
        &self,
        params: &CreateEversendPayoutTransactionBodyParams
    ) -> EversendResult<Transaction, CreateEversendPayoutTransactionError>;
}

#[async_trait]
impl<'a> CreateEversendPayoutTransaction for Payouts<'a> {
    async fn create_eversend_payout_transaction(
        &self,
        params: &CreateEversendPayoutTransactionBodyParams
    ) -> EversendResult<Transaction, CreateEversendPayoutTransactionError> {
        let url = format!("{}/payouts", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CreateEversendPayoutTransactionResponse>>()
            .await?;

        Ok(result.data.transaction)
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
    async fn it_calls_the_create_payouts_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/payouts")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "transaction": {
                            "transactionId": "BP11678896212253",
                            "transactionRef": "externalRefS",
                            "currency": "UGX",
                            "type": "payout",
                            "amount": 500,
                            "fees": 0,
                            "userId": 3,
                            "balanceBefore": 0,
                            "balanceAfter": 0,
                            "sourceCurrency": "UGX",
                            "destinationCurrency": "UGX",
                            "destinationAmount": "500",
                            "destinationCountry": "UG",
                            "beneficiary": {
                                "firstName": "Stone",
                                "lastName": "Atwine",
                                "phoneNumber": "+2567574747",
                                "country": "UG"
                            },
                            "reason": null,
                            "status": "pending",
                            "createdAt": "2023-03-15T16:03:36.860Z",
                            "updatedAt": "2023-03-15T16:03:36.861Z"
                        }
                    },
                    "success": true
                  }).to_string(),
            )
            .create();

        let transaction = eversend
            .payouts()
            .create_eversend_payout_transaction(
                &CreateEversendPayoutTransactionBodyParams {
                    token: String::from("some-token"),
                    transaction_ref: String::from("some-reference"),
                }
            )
            .await
            .unwrap();

        assert_eq!(transaction.amount, 500);

        mock.assert();

    }
}
