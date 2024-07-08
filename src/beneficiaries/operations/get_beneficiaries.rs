use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{beneficiaries::{Beneficiaries, Beneficiary}, ApiResponseBody};

#[derive(Serialize)]
pub struct GetBeneficiariesParams {
    /// Defaults to momo
    #[serde(rename = "type")]
    pub beneficary_type: String,

    /// Defaults to frank
    pub search: String,

    /// Defaults to 100
    pub limit: u32,

    /// Defaults to 1
    pub page: u32,
}

impl Default for GetBeneficiariesParams {
    fn default() -> Self {
        Self {
            beneficary_type: String::from("momo"),
            search: String::from("frank"),
            limit: 100,
            page: 1
        }
    }
}

#[derive(Deserialize)]
struct BeneficiariesApiResponse {
    beneficiaries: Vec<Beneficiary>
}

/// [Eversend Docs: Get Beneficiaries](https://eversend.readme.io/reference/get-beneficiaries)
#[async_trait]
pub trait GetBeneficiaries {
    /// Get [`Beneficiary`]s.
    ///
    /// [Eversend Docs: Get Beneficiaries](https://eversend.readme.io/reference/get-beneficiaries)
    ///
    /// # Examples
    /// ```
    /// use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let beneficiaries = eversend
    ///         .beneficiaries()
    ///         .get_beneficiaries(&GetBeneficiariesParams::default())
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn get_beneficiaries(
        &self,
        params: &GetBeneficiariesParams
    ) -> Result<Vec<Beneficiary>,Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> GetBeneficiaries for Beneficiaries<'a> {
    async fn get_beneficiaries(
        &self,
        params: &GetBeneficiariesParams
    ) -> Result<Vec<Beneficiary>,Box<dyn std::error::Error>> {
        let url = format!("{}/beneficiaries", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .get(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<BeneficiariesApiResponse>>()
            .await?;

        Ok(result.data.unwrap().beneficiaries)
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
    async fn it_calls_the_get_benefieciaries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("GET", "/beneficiaries")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "beneficiaries": [
                            {
                                "id": 1,
                                "firstName": "Frank",
                                "lastName": "Odongkara",
                                "email": "frank@email.com",
                                "phoneNumber": "+256781650001",
                                "bankName": "",
                                "bankCode": "",
                                "bankAccountName": "",
                                "bankAccountNumber": "",
                                "country": "UG",
                                "isEversend": true,
                                "avatar": "",
                                "isBank": false,
                                "isMomo": true,
                            },
                            {
                                "id": 2,
                                "firstName": "Jane",
                                "lastName": "Doe",
                                "email": "frank@email.com",
                                "phoneNumber": "+254781650002",
                                "bankName": "Stanbic Bank",
                                "bankCode": "The Bank Code",
                                "bankAccountName": "Jane",
                                "bankAccountNumber": "28776353527287",
                                "country": "KE",
                                "isEversend": true,
                                "avatar": "",
                                "isBank": true,
                                "isMomo": true,
                            },
                        ]
                    },
                    "success": true,
                }).to_string(),
            )
            .create();

        let response = eversend
            .beneficiaries()
            .get_beneficiaries(
                &GetBeneficiariesParams::default()
            )
            .await
            .unwrap();

        assert_eq!(response[0].phone_number, "+256781650001");

        assert_eq!(response[1].phone_number, "+254781650002");
        mock.assert();

    }
}
