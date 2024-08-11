use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{beneficiaries::{Beneficiaries, Beneficiary}, ApiResponseBody, EversendError, EversendResult, ResponseExtension};

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

/// An error returned from [`GetBeneficiaries`].
#[derive(Debug, Error)]
pub enum GetBeneficiariesError {}

impl From<GetBeneficiariesError> for EversendError<GetBeneficiariesError> {
    fn from(err: GetBeneficiariesError) -> Self {
        Self::Operation(err)
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
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetBeneficiariesError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
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
    ) -> EversendResult<Vec<Beneficiary>, GetBeneficiariesError>;
}

#[async_trait]
impl<'a> GetBeneficiaries for Beneficiaries<'a> {
    async fn get_beneficiaries(
        &self,
        params: &GetBeneficiariesParams
    ) -> EversendResult<Vec<Beneficiary>, GetBeneficiariesError> {
        let url = format!("{}/beneficiaries", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .get(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<ApiResponseBody<BeneficiariesApiResponse>>()
            .await?;

        Ok(result.data.beneficiaries)
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
    async fn it_calls_the_get_benefieciaries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
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
                                "id": 196,
                                "firstName": "John",
                                "lastName": "Doe",
                                "email": null,
                                "phoneNumber": null,
                                "bankName": null,
                                "bankCode": null,
                                "bankAccountName": null,
                                "bankAccountNumber": null,
                                "country": "KE",
                                "isEversend": true,
                                "avatar": null,
                                "isBank": true,
                                "isMomo": true
                            },
                            {
                                "id": 189,
                                "firstName": "Lucas",
                                "lastName": "Graham",
                                "email": null,
                                "phoneNumber": null,
                                "bankName": null,
                                "bankCode": null,
                                "bankAccountName": null,
                                "bankAccountNumber": null,
                                "country": "RW",
                                "isEversend": false,
                                "avatar": null,
                                "isBank": false,
                                "isMomo": true
                            }
                        ],
                        "total": 2,
                        "limit": 10,
                        "page": 1,
                        "totalBeneficiaries": 2
                    },
                    "success": true
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

        assert_eq!(response[0].first_name, "John");
        assert_eq!(response[0].last_name, "Doe");

        assert_eq!(response[1].first_name, "Lucas");
        assert_eq!(response[1].last_name, "Graham");
        mock.assert();

    }
}
