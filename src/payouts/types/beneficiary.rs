use serde::Deserialize;

#[derive(Deserialize)]
pub struct Beneficiary {
    /// The country.
    pub country: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,

    /// Defaults to 0
    pub id: Option<u32>,

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
    pub updated_at: Option<String>,

    /// Account holder name with bank.
    #[serde(rename = "bankAccountName")]
    pub bank_account_name: Option<String>,

    /// Account number from bank.
    #[serde(rename = "bankAccountNumber")]
    pub bank_account_number: Option<String>,

    #[serde(rename = "bankCode")]
    pub bank_code: Option<String>,

    /// The bank Name.
    #[serde(rename = "bankName")]
    pub bank_name: Option<String>,
}
