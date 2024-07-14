use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{payouts::{Payouts, Transaction}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateMomoPayoutTransactionParams {
    /// Recipient Country Code e.g. Nigeria should be NG, Uganda should be UG, etc
    pub country: String,

    /// Recipient First Name
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// Recipient Last Name
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// Recipient Phone Number
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    /// JWT token from quotation
    pub token: String,

    /// Optional unique alphanumeric string set by the client
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
}

/// An error returned from [`CreateMomoPayoutTransaction`].
#[derive(Debug, Error)]
pub enum CreateMomoPayoutTransactionError {}

impl From<CreateMomoPayoutTransactionError> for EversendError<CreateMomoPayoutTransactionError> {
    fn from(err: CreateMomoPayoutTransactionError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateMomoPayoutResponse {
    transaction: Transaction
}

/// [Eversend Docs: Create Payout Transaction Non Beneficiary - Momo](https://eversend.readme.io/reference/create-payout-transaction-non-beneficiary-momo)
#[async_trait]
pub trait CreateMomoPayoutTransaction {
    /// Create a [`Transaction`].
    ///
    /// [Eversend Docs: Create Payout Transaction Non Beneficiary - Momo](https://eversend.readme.io/reference/create-payout-transaction-non-beneficiary-momo)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateMomoPayoutTransactionError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let transaction = eversend
    ///         .payouts()
    ///         .create_momo_payout_transaction(
    ///             &CreateMomoPayoutTransactionParams {
    ///                 country: String::from("UG"),
    ///                 first_name: String::from("John"),
    ///                 last_name: String::from("Doe"),
    ///                 phone_number: String::from("+256789123456"),
    ///                 token: String::from("some-token"),
    ///                 transaction_ref: String::from("some-reference")
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn create_momo_payout_transaction(
        &self,
        params: &CreateMomoPayoutTransactionParams
    ) -> EversendResult<Transaction, CreateMomoPayoutTransactionError>;
}

#[async_trait]
impl<'a> CreateMomoPayoutTransaction for Payouts<'a> {
    async fn create_momo_payout_transaction(
        &self,
        params: &CreateMomoPayoutTransactionParams
    ) -> EversendResult<Transaction, CreateMomoPayoutTransactionError> {
        let url = format!("{}/payouts", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CreateMomoPayoutResponse>>()
            .await?;

        Ok(result.data.transaction)
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
    async fn it_calls_the_create_payouts_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
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
                            "transactionId": "BP11678735362605",
                            "currency": "UGX",
                            "type": "payout",
                            "amount": 1000,
                            "fees": 0,
                            "userId": 3,
                            "balanceBefore": 0,
                            "balanceAfter": 0,
                            "sourceCurrency": "UGX",
                            "destinationCurrency": "NGN",
                            "destinationAmount": "191.16",
                            "destinationCountry": "NG",
                            "beneficiary": {
                                "id": 272,
                                "firstName": "TOCHUKWU ALPHONSUS",
                                "lastName": "OGUGUA",
                                "phoneNumber": "+2348038385263",
                                "createdAt": "2023-03-13T19:22:43.538Z",
                                "updatedAt": "2023-03-13T19:22:44.986Z"
                            },
                            "reason": null,
                            "status": "pending",
                            "createdAt": "2023-03-13T19:22:46.070Z",
                            "updatedAt": "2023-03-13T19:22:46.071Z"
                        }
                    },
                    "success": true
                  }).to_string(),
            )
            .create();

        let transaction = eversend
            .payouts()
            .create_momo_payout_transaction(
                &CreateMomoPayoutTransactionParams {
                    country: String::from("UG"),
                    first_name: String::from("John"),
                    last_name: String::from("Doe"),
                    phone_number: String::from("+256789123456"),
                    token: String::from("some-token"),
                    transaction_ref: String::from("some-reference")
                }
            )
            .await
            .unwrap();

        assert_eq!(transaction.amount, 1000);

        mock.assert();
    }
}
