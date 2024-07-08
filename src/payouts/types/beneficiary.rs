use serde::Deserialize;

#[derive(Deserialize)]
pub struct Beneficiary {
    /// The country.
    pub country: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// Defaults to 0
    pub id: u32,

    /// The first name.
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// The last name.
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// Phone number in international format.
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
