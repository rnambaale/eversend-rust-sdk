use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Beneficiary {
    /// Beneficiary's ID.
    pub id: u32,

    /// The first name.
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// The last name.
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// The email.
    pub email: String,

    /// The phone Number.
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,

    /// The bank Name.
    #[serde(rename = "bankName")]
    pub bank_name: Option<String>,

    /// The bank Code.
    #[serde(rename = "bankCode")]
    pub bank_code: Option<String>,

    /// The bank Account Name.
    #[serde(rename = "bankAccountName")]
    pub bank_account_name: Option<String>,

    /// The bank Account Number.
    #[serde(rename = "bankAccountNumber")]
    pub bank_account_number: Option<String>,

    /// The country.
    pub country: String,

    /// Is Eversend? Deafults to true.
    #[serde(rename = "isEversend")]
    pub is_eversend: bool,

    /// The avatar.
    pub avatar: String,

    /// Is Bank? Deafults to true.
    #[serde(rename = "isBank")]
    pub is_bank: bool,

    /// Is Momo? Deafults to true.
    #[serde(rename = "isMomo")]
    pub is_momo: bool,
}
