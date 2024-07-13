use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{payouts::{Payouts, Transaction}, ApiResponseBody};

#[derive(Serialize)]
pub struct CreateEversendPayoutTransactionBodyParams {
    /// JWT token from quotation
    pub token: String,

    /// Optional unique alphanumeric string set by the client
    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,
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
    /// use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
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
    ) -> Result<Transaction, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> CreateEversendPayoutTransaction for Payouts<'a> {
    async fn create_eversend_payout_transaction(
        &self,
        params: &CreateEversendPayoutTransactionBodyParams
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
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
                            "transactionId": "123",
                            "currency": "UG",
                            "type": "Deposit",
                            "amount": 1500,
                            "fees": 0,
                            "userId": 0,
                            "balanceBefore": 0,
                            "balanceAfter": 1500,
                            "beneficiary": {
                                "country": "UG",
                                "createdAt": "2024-07-10 00:00:00",
                                "id": 0,
                                "firstName": "John",
                                "lastName": "Doe",
                                "phoneNumber": "+256789123456",
                                "updatedAt": "2024-07-10 00:00:00"
                            },
                            "createdAt": "2024-07-10 00:00:00",
                            "destinationAmount": "1500",
                            "destinationCountry": "UG",
                            "destinationCurrency": "UGX",
                            "fees": 0,
                            "reason": "",
                            "sourceCurrency": "UGX",
                            "status": "",
                            "transactionRef": "",
                            "updatedAt": "2024-07-10 00:00:00",
                        },
                    }
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

        assert_eq!(transaction.amount, 1500);

        mock.assert();

    }
}
