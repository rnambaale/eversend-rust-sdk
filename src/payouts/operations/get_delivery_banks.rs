use async_trait::async_trait;
use thiserror::Error;

use crate::{payouts::{Bank, Payouts}, ApiResponseBody, EversendError, EversendResult};

/// An error returned from [`GetDeliveryBanks`].
#[derive(Debug, Error)]
pub enum GetDeliveryBanksError {}

impl From<GetDeliveryBanksError> for EversendError<GetDeliveryBanksError> {
    fn from(err: GetDeliveryBanksError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: Get Delivery Banks](https://eversend.readme.io/reference/get-delivery-banks)
#[async_trait]
pub trait GetDeliveryBanks {
    /// Get [`Bank`]s.
    ///
    /// [Eversend Docs: Get Delivery Banks](https://eversend.readme.io/reference/get-delivery-banks)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetDeliveryBanksError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let banks = eversend
    ///         .payouts()
    ///         .get_delivery_banks(String::from("UG"))
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn get_delivery_banks(
        &self,
        country: String
    ) -> EversendResult<Vec<Bank>, GetDeliveryBanksError>;
}

#[async_trait]
impl<'a> GetDeliveryBanks for Payouts<'a> {
    async fn get_delivery_banks(
        &self,
        country: String
    ) -> EversendResult<Vec<Bank>, GetDeliveryBanksError> {
        let url = format!("{}/payouts/banks/{}", self.eversend.base_url(), country);

        let result = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<Vec<Bank>>>()
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
    async fn it_calls_the_get_delivery_banks_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("GET", "/payouts/banks/UG")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": [
                        {
                            "active": true,
                            "branch": {
                                "city": "Kampala",
                                "code": "CODE11",
                                "id": "11",
                                "name": "Branch One",
                                "state": "The State",
                            },
                            "id": "1",
                            "name": "Bank One",
                        },
                        {
                            "active": true,
                            "branch": {
                                "city": "Kampala",
                                "code": "CODE21",
                                "id": "21",
                                "name": "Branch Two",
                                "state": "The State",
                            },
                            "id": "2",
                            "name": "Bank Two",
                        },
                    ]
                }).to_string(),
            )
            .create();

        let response = eversend
            .payouts()
            .get_delivery_banks(String::from("UG"))
            .await
            .unwrap();

        assert_eq!(response[0].name, "Bank One");
        assert_eq!(response[0].branch.code, "CODE11");

        assert_eq!(response[1].name, "Bank Two");
        assert_eq!(response[1].branch.code, "CODE21");
        mock.assert();

    }
}
