use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Country {

    pub country: String,

    pub id: String,

    pub name: String,

    #[serde(rename = "paymentTypes")]
    pub payment_types: Vec<CountryPaymentType>,

    #[serde(rename = "phonePrefix")]
    pub phone_prefix: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum CountryPaymentType {
    #[serde(rename = "momo")]
    MOMO,

    #[serde(rename = "eversend")]
    EVERSEND,

    #[serde(rename = "bank")]
    BANK
}
