use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

use crate::{beneficiaries::Beneficiaries, EversendError, EversendResult, ResponseExtension};

/// An error returned from [`DeleteBeneficiary`].
#[derive(Debug, Error)]
pub enum DeleteBeneficiaryError {}

impl From<DeleteBeneficiaryError> for EversendError<DeleteBeneficiaryError> {
    fn from(err: DeleteBeneficiaryError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
pub struct DeleteBeneficiaryApiResponse {
    pub code: u16,
    pub success: bool
}

/// [Eversend Docs: Delete A Beneficiary](https://eversend.readme.io/reference/delete-a-beneficiary)
#[async_trait]
pub trait DeleteBeneficiary {
    /// Delete a [`Beneficiary`].
    ///
    /// [Eversend Docs: Delete A Beneficiary](https://eversend.readme.io/reference/delete-a-beneficiary)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::beneficiaries::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), DeleteBeneficiaryError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let _response = eversend
    ///         .beneficiaries()
    ///         .delete_beneficiary(
    ///             206
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn delete_beneficiary(
        &self,
        beneficiary_id: u32
    ) -> EversendResult<(), DeleteBeneficiaryError>;
}

#[async_trait]
impl<'a> DeleteBeneficiary for Beneficiaries<'a> {
    async fn delete_beneficiary(
        &self,
        beneficiary_id: u32
    ) -> EversendResult<(), DeleteBeneficiaryError> {
        let url = format!("{}/beneficiaries/{}", self.eversend.base_url(), beneficiary_id);

        let _response = self
            .eversend
            .client()
            .delete(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .handle_unauthorized_or_generic_error()?
            .json::<DeleteBeneficiaryApiResponse>()
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
    async fn it_calls_the_delete_beneficiary_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("DELETE", "/beneficiaries/16")
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
            .delete_beneficiary(16)
            .await
            .unwrap();

        mock.assert();
    }
}
