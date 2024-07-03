use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::beneficiaries::Beneficiaries;

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

    /// The phone Number.
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    /// Is Bank? Deafults to true.
    #[serde(rename = "isBank")]
    pub is_bank: bool,

    /// Is Momo? Deafults to true.
    #[serde(rename = "isMomo")]
    pub is_momo: bool,

    /// The bank Account Name.
    #[serde(rename = "bankAccountName")]
    pub bank_account_name: Option<String>,

    /// The bank Account Number.
    #[serde(rename = "bankAccountNumber")]
    pub bank_account_number: Option<String>,

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
    /// use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    /// use eversend_rust_sdk::wallets::WalletId;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let _response = eversend
    ///         .beneficiaries()
    ///         .create_beneficiaries(vec![
    ///             &CreateBeneficaryParamItem {
    ///                 first_name: String::from("Frank"),
    ///                 last_name: String::from("Odongkara"),
    ///                 country: String::from("UG"),
    ///                 phone_number: String::from("+256781650001"),
    ///                 bank_account_name: None,
    ///                 bank_account_number: None,
    ///                 is_bank: false,
    ///                 is_momo: true,
    ///             },
    ///             &CreateBeneficaryParamItem {
    ///                 first_name: String::from("Jane"),
    ///                 last_name: String::from("Doe"),
    ///                 country: String::from("KE"),
    ///                 phone_number: String::from("+254781650002"),
    ///                 bank_account_name: Some(String::from("Stanbic Bank")),
    ///                 bank_account_number: Some(String::from("28776353527287")),
    ///                 is_bank: true,
    ///                 is_momo: true,
    ///             }
    ///         ])
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    ///
    async fn create_beneficiaries(
        &self,
        params: Vec<&CreateBeneficaryParamItem>
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> CreateBeneficiaries for Beneficiaries<'a> {
    async fn create_beneficiaries(
        &self,
        params: Vec<&CreateBeneficaryParamItem>
    ) -> Result<(), Box<dyn std::error::Error>> {
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
    use crate::{core::ClientId, eversend::Eversend, ApiToken};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_create_beneficiaries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
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

        let beneficiary_one = CreateBeneficaryParamItem {
            first_name: String::from("Frank"),
            last_name: String::from("Odongkara"),
            country: String::from("UG"),
            phone_number: String::from("+256781650001"),
            bank_account_name: None,
            bank_account_number: None,
            is_bank: false,
            is_momo: true,
        };

        let beneficiary_two = CreateBeneficaryParamItem {
            first_name: String::from("Jane"),
            last_name: String::from("Doe"),
            country: String::from("KE"),
            phone_number: String::from("+254781650002"),
            bank_account_name: Some(String::from("Stanbic Bank")),
            bank_account_number: Some(String::from("28776353527287")),
            is_bank: true,
            is_momo: true,
        };

        let params = vec![
            &beneficiary_one,
            &beneficiary_two,
        ];

        eversend
            .beneficiaries()
            .create_beneficiaries(params)
            .await
            .unwrap();

        mock.assert();
    }
}
