use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::{beneficiaries::{BankDetails, Beneficiaries}, ApiResponseBody, EversendError, EversendResult, ResponseExtension};

#[derive(Serialize)]
pub struct GetBankDetailsParams {
    /// Bank account number.
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    /// Bank code from Get Delivery Banks.
    #[serde(rename = "bankCode")]
    pub bank_code: String,

    /// Alpha-2 country code from Get Delivery Countries.
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

/// An error returned from [`GetBankDetails`].
#[derive(Debug, Error)]
pub enum GetBankDetailsError {}

impl From<GetBankDetailsError> for EversendError<GetBankDetailsError> {
    fn from(err: GetBankDetailsError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: Get Bank Details](https://eversend.readme.io/reference/get-bank-details)
#[async_trait]
pub trait GetBankDetails {
    /// Get [`BankDetails`].
    ///
    /// [Eversend Docs: Get Bank Details](https://eversend.readme.io/reference/get-bank-details)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetBankDetailsError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let _response = eversend
    ///         .beneficiaries()
    ///         .get_bank_details(
    ///             &GetBankDetailsParams {
    ///                 account_number: String::from("0012033048"),
    ///                 bank_code: String::from("14"),
    ///                 country_code: String::from("UG"),
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn get_bank_details(
        &self,
        params: &GetBankDetailsParams
    ) -> EversendResult<BankDetails, GetBankDetailsError>;
}

#[async_trait]
impl<'a> GetBankDetails for Beneficiaries<'a> {
    async fn get_bank_details(
        &self,
        params: &GetBankDetailsParams
    ) -> EversendResult<BankDetails, GetBankDetailsError> {
        let url = format!("{}/beneficiaries/accounts/banks", self.eversend.base_url());

        let response = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<ApiResponseBody<BankDetails>>()
            .await?;

        Ok(response.data)
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
    async fn it_calls_the_get_bank_details_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/beneficiaries/accounts/banks")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "bank_code": "14",
                        "account_number": "0012033048",
                        "account_name": "JOHN DOE"
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let bank_details = eversend
            .beneficiaries()
            .get_bank_details(
                &GetBankDetailsParams {
                    account_number: String::from("0012033048"),
                    bank_code: String::from("14"),
                    country_code: String::from("UG"),
                }
            )
            .await
            .unwrap();

        mock.assert();
        assert_eq!(bank_details.account_name, String::from("JOHN DOE"));
    }
}

