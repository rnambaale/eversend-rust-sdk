use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Country {

    pub country: String,

    pub id: String,

    pub name: String,

    #[serde(rename = "paymentTypes")]
    pub payment_types: Vec<String>,

    #[serde(rename = "phonePrefix")]
    pub phone_prefix: String,
}

