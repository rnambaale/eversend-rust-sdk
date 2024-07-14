use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::{collections::{CollectionFees, Collections}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub enum CollectionMethod {
    #[serde(rename = "momo")]
    MOMO,

    #[serde(rename = "bank")]
    BANK
}

#[derive(Serialize)]
pub struct GetCollectionFeesParams {
    pub amount: u32,

    pub currency: String,

    /// Options are: momo, bank
    pub method: CollectionMethod,
}

/// An error returned from [`GetCollectionFees`].
#[derive(Debug, Error)]
pub enum GetCollectionFeesError {}

impl From<GetCollectionFeesError> for EversendError<GetCollectionFeesError> {
    fn from(err: GetCollectionFeesError) -> Self {
        Self::Operation(err)
    }
}

// [Eversend Docs: Get Collection Fees](https://eversend.readme.io/reference/get-collection-fees)
#[async_trait]
pub trait GetCollectionFees {
    /// Get [`CollectionFees`].
    ///
    /// [Eversend Docs: Get Collection Fees](https://eversend.readme.io/reference/get-collection-fees)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::collections::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetCollectionFeesError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let fees = eversend
    ///         .collections()
    ///         .get_collection_fees(
    ///             &GetCollectionFeesParams {
    ///                 method: CollectionMethod::MOMO,
    ///                 currency: String::from("UGX"),
    ///                 amount: 1000
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn get_collection_fees(
        &self,
        params: &GetCollectionFeesParams
    ) -> EversendResult<CollectionFees, GetCollectionFeesError>;
}

#[async_trait]
impl<'a> GetCollectionFees for Collections<'a> {
    async fn get_collection_fees(
        &self,
        params: &GetCollectionFeesParams
    ) -> EversendResult<CollectionFees, GetCollectionFeesError> {
        let url = format!("{}/collections/fees", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<CollectionFees>>()
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
    async fn it_calls_the_collection_fees_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/collections/fees")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "currency": "UGX",
                        "min_load_amount": "500.00",
                        "max_load_amount": "4000000.00",
                        "max_limit": "40000000.00",
                        "amount_available_to_load": "39977650.34",
                        "charges": "300",
                        "total_to_pay": "20300",
                        "amount": "20000",
                        "payment_method": "16",
                        "new_balance": "42349.66"
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let fees = eversend
            .collections()
            .get_collection_fees(
                &GetCollectionFeesParams {
                    method: CollectionMethod::MOMO,
                    currency: String::from("UGX"),
                    amount: 1000
                }
            )
            .await
            .unwrap();

        assert_eq!(fees.total_to_pay, "20300");

        mock.assert();

    }
}
