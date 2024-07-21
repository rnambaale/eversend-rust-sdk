use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

use crate::{collections::{Collections, MobileMoneyCollection}, ApiResponseBody, EversendError, EversendResult};

#[derive(Serialize)]
pub struct Otp {
    pub pin: String,

    #[serde(rename = "pinId")]
    pub pin_id: String,
}

#[derive(Serialize)]
pub struct GetMobileMoneyCollectionParams {
    /// Amount as a number
    pub amount: u32,

    /// The country you are collecting from. Options are UG, KE, GH, RW, CM, CI, TZ
    pub country: String,

    /// Currency you're collecting. Options are UGX, KES, GHS, RWF
    pub currency: String,

    /// An optional JSON object with customer information e.g. '{"email":"john@example.com"}'
    pub customer: Option<serde_json::Value>,

    /// A JSON object with pinId from Get Collection OTP and pin from customer e.g {"pinId":"132466gdfsfsrey1535", "pin":"123456"}. NB: This is an optional field
    pub otp: Option<Otp>,

    /// Phone number in international format
    #[serde(rename = "phone")]
    pub phone_number: String,

    /// This is required when your collection country is GH and currency is GHS. This is necessary so after phone number verification, it redirects the user back to your system after successful collection
    #[serde(rename = "redirectUrl")]
    pub redirect_url: Option<String>,

    /// Optional unique alphanumeric string set by the client
    #[serde(rename = "transactionRef")]
    pub transaction_ref: Option<String>,
}

/// An error returned from [`GetMobileMoneyCollection`].
#[derive(Debug, Error)]
pub enum GetMobileMoneyCollectionError {}

impl From<GetMobileMoneyCollectionError> for EversendError<GetMobileMoneyCollectionError> {
    fn from(err: GetMobileMoneyCollectionError) -> Self {
        Self::Operation(err)
    }
}

/// [Eversend Docs: Mobile Money Collection](https://eversend.readme.io/reference/mobile-money-collection)
#[async_trait]
pub trait GetMobileMoneyCollection {
    /// Mobile Money Collection.
    ///
    /// [Eversend Docs: Mobile Money Collection](https://eversend.readme.io/reference/mobile-money-collection)
    ///
    /// This is for the momo collection for Kenya, Rwanda, Uganda and Ghana. The OTP object in the payload is an
    /// optional field, it's only needed when your business account isn't whitelisted for verifying phone numbers before
    /// collections, using the collection OTP endpoint. If you have a system in place to verify phone numbers in your
    /// app then reach out to us to whitelist your account so you can proceed to integrate this collection API without
    /// the need for a phone number verification endpoint(OTP collection)
    ///
    /// # Examples
    /// ```
    /// # use eversend_rust_sdk::EversendResult;
    /// # use eversend_rust_sdk::collections::*;
    /// use eversend_rust_sdk::{ClientId,Eversend};
    ///
    /// # async fn run() -> EversendResult<(), GetMobileMoneyCollectionError> {
    ///     let eversend = Eversend::new(
    ///         &ClientId::from("sk_example_123456789"),
    ///         &String::from("sk_example_123456780")
    ///     );
    ///
    ///     let collection = eversend
    ///         .collections()
    ///         .get_mobile_money_collection(
    ///             &GetMobileMoneyCollectionParams {
    ///                 amount: 1000,
    ///                 country: String::from("UG"),
    ///                 currency: String::from("UGX"),
    ///                 phone_number: String::from("+256712345678"),
    ///                 transaction_ref: Some(String::from("ADR234526534")),
    ///                 redirect_url: Some(String::from("https://eversend.co")),
    ///                 customer: None,
    ///                 otp: None,
    ///             }
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    ///
    async fn get_mobile_money_collection(
        &self,
        params: &GetMobileMoneyCollectionParams
    ) -> EversendResult<MobileMoneyCollection, GetMobileMoneyCollectionError>;
}

#[async_trait]
impl<'a> GetMobileMoneyCollection for Collections<'a> {
    async fn get_mobile_money_collection(
        &self,
        params: &GetMobileMoneyCollectionParams
    ) -> EversendResult<MobileMoneyCollection, GetMobileMoneyCollectionError> {
        let url = format!("{}/collections/momo", self.eversend.base_url());

        let result = self
            .eversend
            .client()
            .post(url)
            .json(&params)
            .bearer_auth(self.eversend.api_token().unwrap())
            .send()
            .await?
            .json::<ApiResponseBody<MobileMoneyCollection>>()
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
    async fn it_calls_the_mobile_money_collection_endpoint() {
        let eversend = Eversend::builder(
            &ClientId::from("sk_example_123456789"),
            &String::from("sk_example_123456780")
        )
            .set_base_url(&mockito::server_url())
            .set_api_token(&ApiToken::from("some_test_token"))
            .build();

        let mock = mock("POST", "/collections/momo")
            .with_status(200)
            .with_body(
                json!({
                    "code": 200,
                    "data": {
                        "transactionId": "BE31650891443685",
                        "transactionRef": "ADR234526534",
                        "type": "collection",
                        "currency": "UGX",
                        "amount": "2000",
                        "customer": {
                            "email": "john@example.com"
                        },
                        "balanceBefore": null,
                        "balanceAfter": null,
                        "status": "pending",
                        "createdAt": "2022-04-25T12:57:41.444Z",
                        "updatedAt": "2022-04-25T12:57:41.445Z"
                    },
                    "success": true
                  }).to_string(),
            )
            .create();

        let collection = eversend
            .collections()
            .get_mobile_money_collection(
                &GetMobileMoneyCollectionParams {
                    amount: 1000,
                    country: String::from("UG"),
                    currency: String::from("UGX"),
                    phone_number: String::from("+256712345678"),
                    transaction_ref: Some(String::from("ADR234526534")),
                    redirect_url: Some(String::from("https://eversend.co")),
                    customer: None,
                    otp: None,
                }
            )
            .await
            .unwrap();

        assert_eq!(collection.transaction_id, "BE31650891443685");

        mock.assert();

    }
}
