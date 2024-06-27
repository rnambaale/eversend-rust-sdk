use async_trait::async_trait;
use serde::Deserialize;

use crate::{auth::Auth, core::ApiToken};

/// [Eversend Docs: Generate Token](https://eversend.readme.io/reference/get-token)
#[async_trait]
pub trait GenerateApiToken {
    /// Retrieves an [`ApiToken`].
    ///
    /// [Eversend Docs: Get an API token](https://eversend.readme.io/reference/get-token)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::auth::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let api_token = eversend
    ///         .auth()
    ///         .generate_api_token()
    ///         .await?;
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn generate_api_token(
        &self,
    ) -> Result<ApiToken, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> GenerateApiToken for Auth<'a> {
    async fn generate_api_token(
        &self
    ) -> Result<ApiToken, Box<dyn std::error::Error>> {
        let url = format!("{}/auth/token", self.eversend.base_url());

        let response = self
            .eversend
            .client()
            .get(url)
            .header("clientId", self.eversend.client_id().to_string())
            .header("clientSecret", self.eversend.client_secret())
            .send()
            .await?
            .json::<ApiTokenResponse>()
            .await?;

        // if response.status().is_success() {
        //     let body = response.json::<ApiToken>()?;
        //     Ok(body)
        // } else {
        //     Err(format!("Request failed with status: {}", response.status()).into())
        // }

        Ok(response.token)
    }
}

#[derive(Deserialize, Debug)]
struct ApiTokenResponse {
    // status: u32,
    token: ApiToken
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{core::ClientId, eversend::Eversend};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_token_generation_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .build();

        let _mock = mock("GET", "/auth/token")
            .with_status(200)
            .with_body(
                json!({
                    "status": 200,
                    "token": "some_test_token"
                }).to_string(),
            )
            .create();

        let api_token = eversend
            .auth()
            .generate_api_token()
            .await
            .unwrap();

        assert_eq!(
            api_token,
            ApiToken::from("some_test_token")
        )
    }
}
