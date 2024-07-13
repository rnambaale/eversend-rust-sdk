use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{payouts::{Payouts, Quotation}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateEversendPayoutBodyParams {
    /// Source amount to pay
    pub amount: u32,

    /// DESTINATION or SOURCE - DESTINATION means if we should calculate using destination currency, SOURCE means if we should calculate using source currency. Defaults to SOURCE
    #[serde(rename = "amountType")]
    pub amount_type: String,

    /// optional field, Eversend customer identifier type email
    pub email: String,

    /// Identifier must be either phone, email or tag, if one of the optional fields below is entered.
    pub identifier: String,

    /// optional field, Eversend customer identifier type phone
    pub phone: String,

    /// Source wallet currency from Get Wallets
    #[serde(rename = "sourceWallet")]
    pub source_wallet: String,

    /// optional field, Eversend customer identifier type tag
    pub tag: String,
}

/// An error returned from [`CreateEversendPayoutQuotation`].
#[derive(Debug, Error)]
pub enum CreateEversendPayoutQuotationError {}

impl From<CreateEversendPayoutQuotationError> for EversendError<CreateEversendPayoutQuotationError> {
    fn from(err: CreateEversendPayoutQuotationError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateEversendPayoutResponse {
    pub quotation: Quotation,
    pub token: String,
}

/// [Eversend Docs: Create Payout Quotation - Eversend](https://eversend.readme.io/reference/create-payout-quotation-eversend)
#[async_trait]
pub trait CreateEversendPayoutQuotation {
    /// Create a [`Quotation`].
    ///
    /// [Eversend Docs: Create Payout Quotation - Eversend](https://eversend.readme.io/reference/create-payout-quotation-eversend)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateEversendPayoutQuotationError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let countries = eversend
    ///         .payouts()
    ///         .create_eversend_payout_quotation(
    ///             &CreateEversendPayoutBodyParams {
    ///                 amount: 1000,
    ///                 amount_type: String::from("SOURCE"),
    ///                 email: String::from("jdoe@example.com"),
    ///                 identifier: String::from("email"),
    ///                 phone: String::from("+256789123456"),
    ///                 source_wallet: String::from("UGX"),
    ///                 tag: String::from("the-tag"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn create_eversend_payout_quotation(
        &self,
        params: &CreateEversendPayoutBodyParams
    ) -> EversendResult<CreateEversendPayoutResponse, CreateEversendPayoutQuotationError>;
}

#[async_trait]
impl<'a> CreateEversendPayoutQuotation for Payouts<'a> {
    async fn create_eversend_payout_quotation(
        &self,
        params: &CreateEversendPayoutBodyParams
    ) -> EversendResult<CreateEversendPayoutResponse, CreateEversendPayoutQuotationError> {
        let url = format!("{}/payouts/quotation", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CreateEversendPayoutResponse>>()
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
                            "merchant": {
                                "result": "Success",
                                "merchantExists": true,
                                "country": "UG",
                                "defaultWallet": "UGX",
                                "isMerchant": true,
                                "firstName": "John",
                                "lastName": "Doe",
                                "email": "jdoe@example.com",
                                "phoneNumber": {
                                    "prefix": "+256",
                                    "number": "789123456"
                                },
                                "tag": "some-tag"
                            }
                        },
                    }
                }).to_string(),
            )
            .create();

        let response = eversend
            .payouts()
            .create_eversend_payout_quotation(
                &CreateEversendPayoutBodyParams {
                    amount: 1000,
                    amount_type: String::from("SOURCE"),
                    email: String::from("jdoe@example.com"),
                    identifier: String::from("email"),
                    phone: String::from("+256789123456"),
                    source_wallet: String::from("UGX"),
                    tag: String::from("the-tag"),
                }
            )
            .await
            .unwrap();

        assert_eq!(response.token, "some-test-quotation-token");
        assert_eq!(response.quotation.total_amount, "1050");

        let merchant = response.quotation.merchant.unwrap();
        assert_eq!(merchant.result, "Success");
        assert_eq!(merchant.merchant_exists, true);
        assert_eq!(merchant.country, "UG");
        assert_eq!(merchant.default_wallet, "UGX");
        assert_eq!(merchant.is_merchant, true);
        assert_eq!(merchant.phone_number.number, "789123456");
        assert_eq!(merchant.phone_number.prefix, "+256");

        mock.assert();
    }
}
