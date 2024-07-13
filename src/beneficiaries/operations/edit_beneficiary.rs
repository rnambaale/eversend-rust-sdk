use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{beneficiaries::Beneficiaries, EversendError, EversendResult};

#[derive(Deserialize, Serialize)]
pub struct EditBeneficiaryParams {
    /// The first name.
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// The last name.
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// Phone number in international format.
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    /// The bank Name.
    #[serde(rename = "bankName")]
    pub bank_name: Option<String>,

    /// Bank code from `Get Delivery Banks`.
    #[serde(rename = "bankCode")]
    pub bank_code: Option<String>,

    /// Account holder name with bank.
    #[serde(rename = "bankAccountName")]
    pub bank_account_name: Option<String>,

    /// Account number from bank.
    #[serde(rename = "bankAccountNumber")]
    pub bank_account_number: Option<String>,
}

/// An error returned from [`EditBeneficiary`].
#[derive(Debug, Error)]
pub enum EditBeneficiaryError {}

impl From<EditBeneficiaryError> for EversendError<EditBeneficiaryError> {
    fn from(err: EditBeneficiaryError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: Edit A Beneficiary](https://eversend.readme.io/reference/edit-a-beneficiary)
#[async_trait]
pub trait EditBeneficiary {
    /// Edit a [`Beneficiary`].
    ///
    /// [Eversend Docs: Edit A Beneficiary](https://eversend.readme.io/reference/edit-a-beneficiary)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), EditBeneficiaryError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let _response = eversend
    ///         .beneficiaries()
    ///         .edit_beneficiary(
    ///             206,
    ///             &EditBeneficiaryParams {
    ///                 first_name: String::from("Frank"),
    ///                 last_name: String::from("Odongkara"),
    ///                 phone_number: String::from("+256781650001"),
    ///                 bank_account_name: None,
    ///                 bank_account_number: None,
    ///                 bank_name: None,
    ///                 bank_code: None,
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn edit_beneficiary(
        &self,
        beneficiary_id: u32,
        params: &EditBeneficiaryParams
    ) -> EversendResult<(), EditBeneficiaryError>;
}

#[derive(Deserialize)]
pub struct EditBeneficiaryResponse {
    pub code: u16,
    pub success: bool,
}

#[async_trait]
impl<'a> EditBeneficiary for Beneficiaries<'a> {
    async fn edit_beneficiary(
        &self,
        beneficiary_id: u32,
        params: &EditBeneficiaryParams
    ) -> EversendResult<(), EditBeneficiaryError> {
        let url = format!("{}/beneficiaries/{}", self.eversend.base_url(), beneficiary_id);

        let _response = self
            .eversend
            .client()
            .put(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<EditBeneficiaryResponse>()
            .await?;
        Ok(())
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
    async fn it_calls_the_edit_beneficiaries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("PUT", "/beneficiaries/206")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "success": true
                }).to_string(),
            )
            .create();

        eversend
            .beneficiaries()
            .edit_beneficiary(
                206,
                &EditBeneficiaryParams {
                    first_name: String::from("Frank"),
                    last_name: String::from("Odongkara"),
                    phone_number: String::from("+256781650001"),
                    bank_name: None,
                    bank_code: None,
                    bank_account_name: None,
                    bank_account_number: None,
                }
            )
            .await
            .unwrap();

        mock.assert();
    }
}
