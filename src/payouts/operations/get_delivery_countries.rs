use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

use crate::{payouts::{Country, Payouts}, ApiResponseBody, EversendError, EversendResult};

/// An error returned from [`GetDeliveryCountries`].
#[derive(Debug, Error)]
pub enum GetDeliveryCountriesError {}

impl From<GetDeliveryCountriesError> for EversendError<GetDeliveryCountriesError> {
    fn from(err: GetDeliveryCountriesError) -> Self {
        Self::Operation(err)
    }
}

#[derive(Deserialize)]
struct DeliveryCountriesApiResponse {
    countries: Vec<Country>
}

/// [Eversend Docs: Get Delivery Countries](https://eversend.readme.io/reference/get-delivery-countries)
#[async_trait]
pub trait GetDeliveryCountries {
    /// Get [`Country`]s.
    ///
    /// [Eversend Docs: Get Delivery Countries](https://eversend.readme.io/reference/get-delivery-countries)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,ClientSecret,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetDeliveryCountriesError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &ClientSecret::from("sk_example_123456780")
    ///     );
    ///
    ///     let countries = eversend
    ///         .payouts()
    ///         .get_delivery_countries()
    ///         .await?;
    ///
    ///     Ok(())
    ///
    /// # }
    /// ```
    async fn get_delivery_countries(
        &self
    ) -> EversendResult<Vec<Country>, GetDeliveryCountriesError>;
}

#[async_trait]
impl<'a> GetDeliveryCountries for Payouts<'a> {
    async fn get_delivery_countries(
        &self
    ) -> EversendResult<Vec<Country>, GetDeliveryCountriesError> {
        let url = format!("{}/payouts/countries", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .get(url)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<DeliveryCountriesApiResponse>>()
            .await?;

        Ok(result.data.countries)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientId, eversend::Eversend, payouts::CountryPaymentType, ApiToken, ClientSecret};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_get_delivery_countries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &ClientSecret::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("GET", "/payouts/countries")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "countries": [
                            {
                                "country": "UG",
                                "id": "999",
                                "name": "Uganda",
                                "paymentTypes": [
                                  "eversend",
                                  "momo",
                                  "bank"
                                ],
                                "phonePrefix": "+256"
                            },
                            {
                                "country": "KE",
                                "id": "998",
                                "name": "Kenya",
                                "paymentTypes": [
                                  "eversend",
                                  "momo",
                                  "bank"
                                ],
                                "phonePrefix": "+254"
                            },
                            {
                                "country": "NG",
                                "id": "990",
                                "name": "Nigeria",
                                "paymentTypes": [
                                    "eversend",
                                    "bank"
                                ],
                                "phonePrefix": "+234"
                            }
                        ]
                    },
                    "success": true
                }).to_string(),
            )
            .create();

        let response = eversend
            .payouts()
            .get_delivery_countries()
            .await
            .unwrap();

        assert_eq!(response[0].country, "UG");
        assert_eq!(
            response[0].payment_types,
            vec![
                CountryPaymentType::EVERSEND,
                CountryPaymentType::MOMO,
                CountryPaymentType::BANK
            ]
        );

        assert_eq!(response[1].country, "KE");
        assert_eq!(response[1].phone_prefix, "+254");

        assert_eq!(response[2].country, "NG");
        assert_eq!(
            response[2].payment_types,
            vec![
                CountryPaymentType::EVERSEND,
                CountryPaymentType::BANK
            ]
        );
        mock.assert();

    }
}
