use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{payouts::{Payouts, Quotation}, ApiResponseBody};

#[derive(Serialize)]
pub struct CreateQuotationBodyParams {
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
    /// use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let countries = eversend
    ///         .payouts()
    ///         .create_momo_and_bank_payout_quotation(
    ///             &CreateQuotationBodyParams {
    ///                 amount: 1000,
    ///                 amount_type: String::from("SOURCE"),
    ///                 destination_country: String::from("UG"),
    ///                 destination_currency: String::from("UGX"),
    ///                 source_wallet: String::from("UGX"),
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
        params: &CreateQuotationBodyParams
    ) -> Result<CreateQuotationResponse, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> CreateMomoAndBankPayoutQuotation for Payouts<'a> {
    async fn create_momo_and_bank_payout_quotation(
        &self,
        params: &CreateQuotationBodyParams
    ) -> Result<CreateQuotationResponse, Box<dyn std::error::Error>> {
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
    use crate::{core::ClientId, eversend::Eversend, ApiToken};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_create_quotation_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
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
                        "token": "some-test-quotation-token",
                        "quotation": {
                            "amount": 1000,
                            "amountType": "SOURCE",
                            "destinationAmount": "1500",
                            "destinationCountry": "UG",
                            "destinationCurrency": "UGX",
                            "exchangeRate": "1",
                            "sourceAmount": "2000",
                            "sourceCountry": "UG",
                            "sourceCurrency": "UGX",
                            "totalAmount": "1050",
                            "totalFees": "50",
                            "type": "momo",
                        },
                    }
                }).to_string(),
            )
            .create();

        let response = eversend
            .payouts()
            .create_momo_and_bank_payout_quotation(
                &CreateQuotationBodyParams {
                    amount: 1000,
                    amount_type: String::from("SOURCE"),
                    destination_country: String::from("UG"),
                    destination_currency: String::from("UGX"),
                    source_wallet: String::from("UGX"),
                    transaction_type: String::from("momo"),
                }
            )
            .await
            .unwrap();

        assert_eq!(response.token, "some-test-quotation-token");
        assert_eq!(response.quotation.total_amount, "1050");

        mock.assert();

    }
}
