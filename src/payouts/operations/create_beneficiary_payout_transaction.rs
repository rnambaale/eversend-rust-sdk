use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{payouts::{Payouts, Transaction}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateBeneficiaryPayoutTransactionParams {
    /// Id of beneficiary you are sending to, you can get the beneficiary Id from the get beneficiary endpoint
    #[serde(rename = "beneficiaryId")]
    pub beneficiary_id: String,

    /// JWT token from quotation
    pub token: String,
}

/// An error returned from [`CreateBeneficiaryPayoutTransaction`].
#[derive(Debug, Error)]
pub enum CreateBeneficiaryPayoutTransactionError {}

impl From<CreateBeneficiaryPayoutTransactionError> for EversendError<CreateBeneficiaryPayoutTransactionError> {
    fn from(err: CreateBeneficiaryPayoutTransactionError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateBeneficiaryPayoutResponse {
    transaction: Transaction
}

// [Eversend Docs: Create Payout Transaction Beneficiary](https://eversend.readme.io/reference/create-payout-transaction-beneficiary)
#[async_trait]
pub trait CreateBeneficiaryPayoutTransaction {
    /// Create a [`Transaction`].
    ///
    /// [Eversend Docs: Create Payout Transaction Beneficiary](https://eversend.readme.io/reference/create-payout-transaction-beneficiary)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateBeneficiaryPayoutTransactionError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let transaction = eversend
    ///         .payouts()
    ///         .create_beneficiary_payout_transaction(
    ///             &CreateBeneficiaryPayoutTransactionParams {
    ///                 token: String::from("some-token"),
    ///                 beneficiary_id: String::from("123"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    async fn create_beneficiary_payout_transaction(
        &self,
        params: &CreateBeneficiaryPayoutTransactionParams
    ) -> EversendResult<Transaction, CreateBeneficiaryPayoutTransactionError>;
}

#[async_trait]
impl<'a> CreateBeneficiaryPayoutTransaction for Payouts<'a> {
    async fn create_beneficiary_payout_transaction(
        &self,
        params: &CreateBeneficiaryPayoutTransactionParams
    ) -> EversendResult<Transaction, CreateBeneficiaryPayoutTransactionError> {
        let url = format!("{}/payouts", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CreateBeneficiaryPayoutResponse>>()
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
                                "country": "NG",
                                "bankCode": null,
                                "bankName": null,
                                "bankAccountName": null,
                                "bankAccountNumber": null,
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
            .create_beneficiary_payout_transaction(
                &CreateBeneficiaryPayoutTransactionParams {
                    token: String::from("some-token"),
                    beneficiary_id: String::from("123"),
                }
            )
            .await
            .unwrap();

        assert_eq!(transaction.amount, 1000);

        mock.assert();

    }
}
