use serde::Deserialize;

#[derive(Deserialize)]
pub struct BankDetails {
    /// Bank account name.
    #[serde(rename = "accountName")]
    pub account_name: String,

    /// Bank account number.
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    /// Bank code from Get Delivery Banks.
    #[serde(rename = "bankCode")]
    pub bank_code: String,
}
