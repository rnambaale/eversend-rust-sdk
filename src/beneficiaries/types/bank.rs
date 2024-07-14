use serde::Deserialize;

#[derive(Deserialize)]
pub struct BankDetails {
    /// Bank account name.
    pub account_name: String,

    /// Bank account number.
    pub account_number: String,

    /// Bank code from Get Delivery Banks.
    pub bank_code: String,
}
