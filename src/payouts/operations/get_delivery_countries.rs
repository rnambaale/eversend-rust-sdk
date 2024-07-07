use async_trait::async_trait;
use serde::Deserialize;

use crate::{payouts::{Country, Payouts}, ApiResponseBody};

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
    /// use eversend_rust_sdk::payouts::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
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
    ) -> Result<Vec<Country>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl<'a> GetDeliveryCountries for Payouts<'a> {
    async fn get_delivery_countries(
        &self
    ) -> Result<Vec<Country>, Box<dyn std::error::Error>> {
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
    use crate::{core::ClientId, eversend::Eversend, ApiToken};

    use super::*;
    use mockito::{self, mock};
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn it_calls_the_get_delivery_countries_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
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
                                "id": "1",
                                "name": "Uganda",
                                "paymentTypes": ["mobile money", "bank"],
                                "phonePrefix": "+256",
                            },
                            {
                                "country": "KE",
                                "id": "2",
                                "name": "Kenya",
                                "paymentTypes": ["mobile money", "cash"],
                                "phonePrefix": "+254",
                            },
                        ]
                    }
                }).to_string(),
            )
            .create();

        let response = eversend
            .payouts()
            .get_delivery_countries()
            .await
            .unwrap();

        assert_eq!(response[0].country, "UG");
        assert_eq!(response[0].payment_types, vec!["mobile money", "bank"]);

        assert_eq!(response[1].country, "KE");
        assert_eq!(response[1].payment_types, vec!["mobile money", "cash"]);
        mock.assert();

    }
}
