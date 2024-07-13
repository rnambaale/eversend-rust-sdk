use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

use crate::{beneficiaries::{Beneficiaries, Beneficiary}, ApiResponseBody, EversendError, EversendResult};

#[derive(Deserialize)]
struct GetBeneficaryApiResponse {
    beneficiary: Beneficiary,
}

/// An error returned from [`GetBeneficiary`].
#[derive(Debug, Error)]
pub enum GetBeneficiaryError {}

impl From<GetBeneficiaryError> for EversendError<GetBeneficiaryError> {
    fn from(err: GetBeneficiaryError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: Get A Beneficiary](https://eversend.readme.io/reference/get-a-beneficiary)
#[async_trait]
pub trait GetBeneficiary {
    /// Get A [`Beneficiary`].
    ///
    /// [Eversend Docs: Get A Beneficiary](https://eversend.readme.io/reference/get-a-beneficiary)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetBeneficiaryError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let beneficiary = eversend
    ///         .beneficiaries()
    ///         .get_beneficiary(206)
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn get_beneficiary(
        &self,
        beneficiary_id: u32
    ) -> EversendResult<Beneficiary, GetBeneficiaryError>;
}

#[async_trait]
impl<'a> GetBeneficiary for Beneficiaries<'a> {
    async fn get_beneficiary(
        &self,
        beneficiary_id: u32
    ) -> EversendResult<Beneficiary, GetBeneficiaryError> {
        let url = format!("{}/beneficiaries/{}", self.eversend.base_url(), beneficiary_id);

        let response = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<GetBeneficaryApiResponse>>()
            .await?;

        Ok(response.data.beneficiary)
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
    async fn it_calls_the_get_beneficiary_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let _mock = mock("GET", "/beneficiaries/206")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "beneficiary" : {
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
                            "transactions": [],
                            "avatar": "",
                            "isBank": false,
                            "isMomo": true,
                        }
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let beneficiary = eversend
            .beneficiaries()
            .get_beneficiary(206)
            .await
            .unwrap();

        assert_eq!(beneficiary.phone_number, String::from("+256781650001"));
    }
}
