use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{beneficiaries::Beneficiaries, EversendError, EversendResult};

#[derive(Serialize)]
pub struct CreateBeneficaryParamItem {
    /// The first name.
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// The last name.
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// The country.
    pub country: String,

    /// Phone number in international format.
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    /// Is Bank? Deafults to true.
    #[serde(rename = "isBank")]
    pub is_bank: bool,

    /// Is Momo? Deafults to true.
    #[serde(rename = "isMomo")]
    pub is_momo: bool,

    /// Account holder name with bank.
    #[serde(rename = "bankAccountName")]
    pub bank_account_name: Option<String>,

    /// Account number from bank.
    #[serde(rename = "bankAccountNumber")]
    pub bank_account_number: Option<String>,
}

/// An error returned from [`CreateBeneficiaries`].
#[derive(Debug, Error)]
pub enum CreateBeneficiariesError {}

impl From<CreateBeneficiariesError> for EversendError<CreateBeneficiariesError> {
    fn from(err: CreateBeneficiariesError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct CreateBeneficiariesApiResponse {
    pub code: u16,
    pub success: bool
}

/// [Eversend Docs: Create Beneficiaries](https://eversend.readme.io/reference/create-beneficiaries)
#[async_trait]
pub trait CreateBeneficiaries {
    /// Create [`Beneficiary`]s.
    ///
    /// [Eversend Docs: Create Beneficiaries](https://eversend.readme.io/reference/create-beneficiaries)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), CreateBeneficiariesError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let _response = eversend
    ///         .beneficiaries()
    ///         .create_beneficiary(
    ///             &CreateBeneficaryParams {
    ///                 first_name: String::from("Jane"),
    ///                 last_name: String::from("Doe"),
    ///                 country: String::from("KE"),
    ///                 phone_number: String::from("+254781650002"),
    ///                 bank_account_name: Some(String::from("Stanbic Bank")),
    ///                 bank_account_number: Some(String::from("28776353527287")),
    ///                 is_bank: true,
    ///                 is_momo: true,
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn create_beneficiary(
        &self,
        params: &CreateBeneficaryParamItem
    ) -> EversendResult<(), CreateBeneficiariesError>;
}

#[async_trait]
impl<'a> CreateBeneficiaries for Beneficiaries<'a> {
    async fn create_beneficiary(
        &self,
        params: &CreateBeneficaryParamItem
    ) -> EversendResult<(), CreateBeneficiariesError> {
        let url = format!("{}/beneficiaries", self.eversend.base_url());

        let _response = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<CreateBeneficiariesApiResponse>()
            .await?;

        Ok(())
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
    async fn it_calls_the_create_beneficiaries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/beneficiaries")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "success": true
                }).to_string(),
            )
            .create();

        let beneficiary = CreateBeneficaryParamItem {
            first_name: String::from("Jane"),
            last_name: String::from("Doe"),
            country: String::from("KE"),
            phone_number: String::from("+254781650002"),
            bank_account_name: Some(String::from("Stanbic Bank")),
            bank_account_number: Some(String::from("28776353527287")),
            is_bank: true,
            is_momo: true,
        };

        eversend
            .beneficiaries()
            .create_beneficiary(&beneficiary)
            .await
            .unwrap();

        mock.assert();
    }
}
