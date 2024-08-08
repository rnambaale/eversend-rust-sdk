use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{payouts::{Payouts, Quotation}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateMomoAndBankPayoutQuotationParams {
    /// Source amount to pay
    pub amount: u32,

    /// DESTINATION or SOURCE - DESTINATION means if we should calculate using destination currency, SOURCE means if we should calculate using source currency. Defaults to SOURCE
    #[serde(rename = "amountType")]
    pub amount_type: String,

    /// Destination country ALPHA-2 code e.g NG for Nigeria
    #[serde(rename = "destinationCountry")]
    pub destination_country: String,

    /// Destination currency
    #[serde(rename = "destinationCurrency")]
    pub destination_currency: String,

    /// Source wallet currency from Get Wallets
    #[serde(rename = "sourceWallet")]
    pub source_wallet: String,

    /// Options are momo and bank. Call `Get Delivery Countries` to get a list of payment types per country
    #[serde(rename = "type")]
    pub transaction_type: String,
}

/// An error returned from [`CreateMomoAndBankPayoutQuotation`].
#[derive(Debug, Error)]
pub enum CreateMomoAndBankPayoutQuotationError {}

impl From<CreateMomoAndBankPayoutQuotationError> for EversendError<CreateMomoAndBankPayoutQuotationError> {
    fn from(err: CreateMomoAndBankPayoutQuotationError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateQuotationResponse {
    pub quotation: Quotation,
    pub token: String,
}

/// [Eversend Docs: Create Payout Quotation - Momo & Bank](https://eversend.readme.io/reference/create-payout-quotation)
#[async_trait]
pub trait CreateMomoAndBankPayoutQuotation {
    /// Create a [`Quotation`].
    ///
    /// [Eversend Docs: Create Payout Quotation - Momo & Bank](https://eversend.readme.io/reference/create-payout-quotation)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateMomoAndBankPayoutQuotationError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let quotation = eversend
    ///         .payouts()
    ///         .create_momo_and_bank_payout_quotation(
    ///             &CreateMomoAndBankPayoutQuotationParams {
    ///                 amount: 20,
    ///                 amount_type: String::from("SOURCE"),
    ///                 destination_country: String::from("KE"),
    ///                 destination_currency: String::from("KES"),
    ///                 source_wallet: String::from("KES"),
    ///                 transaction_type: String::from("momo"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn create_momo_and_bank_payout_quotation(
        &self,
        params: &CreateMomoAndBankPayoutQuotationParams
    ) -> EversendResult<CreateQuotationResponse, CreateMomoAndBankPayoutQuotationError>;
}

#[async_trait]
impl<'a> CreateMomoAndBankPayoutQuotation for Payouts<'a> {
    async fn create_momo_and_bank_payout_quotation(
        &self,
        params: &CreateMomoAndBankPayoutQuotationParams
    ) -> EversendResult<CreateQuotationResponse, CreateMomoAndBankPayoutQuotationError> {
        let url = format!("{}/payouts/quotation", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CreateQuotationResponse>>()
            .await?;

        Ok(result.data)
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
    async fn it_calls_the_create_quotation_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/payouts/quotation")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJxdW90YXRpb24iOnsic291cmNlQ291bnRyeSI6IlVHIiwic291cmNlQ3VycmVuY3kiOiJVR1giLCJzb3VyY2VBbW91bnQiOiI3MDAiLCJkZXN0aW5hdGlvbkNvdW50cnkiOiJLRSIsImRlc3RpbmF0aW9uQ3VycmVuY3kiOiJLRVMiLCJkZXN0aW5hdGlvbkFtb3VudCI6IjIxLjcxIiwiZXhjaGFuZ2VSYXRlIjoiMC4wMzEwMDcyMDE5ODEzNjciLCJ0b3RhbEZlZXMiOiIyNTAwIiwidG90YWxBbW91bnQiOiIzMjAwLjAwIiwidHlwZSI6Im1vbW8iLCJhbW91bnRUeXBlIjoiU09VUkNFIiwiYW1vdW50IjoiNzAwIn0sImlhdCI6MTY2MTg4Mzc1NywiZXhwIjoxNjYxODg1NTU3fQ.7Q4RweZ2Osf9YwlXfqvv_FzKM9ob-AjlCtINj17cPEI",
                        "quotation": {
                            "sourceCountry": "UG",
                            "sourceCurrency": "UGX",
                            "sourceAmount": "1000",
                            "destinationCountry": "NG",
                            "destinationCurrency": "NGN",
                            "destinationAmount": "21.71",
                            "exchangeRate": "0.031007201981367",
                            "totalFees": "2500",
                            "totalAmount": "3200.00",
                            "type": "momo",
                            "amountType": "SOURCE",
                            "amount": "1000"
                        }
                    },
                    "success": true
                  }).to_string(),
            )
            .create();

        let response = eversend
            .payouts()
            .create_momo_and_bank_payout_quotation(
                &CreateMomoAndBankPayoutQuotationParams {
                    amount: 20,
                    amount_type: String::from("SOURCE"),
                    destination_country: String::from("KE"),
                    destination_currency: String::from("KES"),
                    source_wallet: String::from("KES"),
                    transaction_type: String::from("momo"),
                }
            )
            .await
            .unwrap();

        assert_eq!(
            response.token,
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJxdW90YXRpb24iOnsic291cmNlQ291bnRyeSI6IlVHIiwic291cmNlQ3VycmVuY3kiOiJVR1giLCJzb3VyY2VBbW91bnQiOiI3MDAiLCJkZXN0aW5hdGlvbkNvdW50cnkiOiJLRSIsImRlc3RpbmF0aW9uQ3VycmVuY3kiOiJLRVMiLCJkZXN0aW5hdGlvbkFtb3VudCI6IjIxLjcxIiwiZXhjaGFuZ2VSYXRlIjoiMC4wMzEwMDcyMDE5ODEzNjciLCJ0b3RhbEZlZXMiOiIyNTAwIiwidG90YWxBbW91bnQiOiIzMjAwLjAwIiwidHlwZSI6Im1vbW8iLCJhbW91bnRUeXBlIjoiU09VUkNFIiwiYW1vdW50IjoiNzAwIn0sImlhdCI6MTY2MTg4Mzc1NywiZXhwIjoxNjYxODg1NTU3fQ.7Q4RweZ2Osf9YwlXfqvv_FzKM9ob-AjlCtINj17cPEI"
        );
        assert_eq!(response.quotation.total_amount, "3200.00");

        mock.assert();

    }
}
