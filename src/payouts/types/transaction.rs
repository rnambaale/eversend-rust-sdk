use serde::Deserialize;

use super::Beneficiary;

#[derive(Deserialize)]
pub struct Transaction {
    /// Defaults to 0
    pub amount: u32,

    /// Defaults to 0
    #[serde(rename = "balanceAfter")]
    pub balance_after: u32,

    /// Defaults to 0
    #[serde(rename = "balanceBefore")]
    pub balance_before: u32,

    pub beneficiary: Beneficiary,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    pub currency: String,

    #[serde(rename = "destinationAmount")]
    pub destination_amount: String,

    #[serde(rename = "destinationCountry")]
    pub destination_country: String,

    #[serde(rename = "destinationCurrency")]
    pub destination_currency: String,

    /// Defaults to 0
    pub fees: u32,

    pub reason: String,

    #[serde(rename = "sourceCurrency")]
    pub source_currency: String,

    pub status: String,

    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,

    #[serde(rename = "type")]
    pub transaction_type: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    /// Defaults to 0
    #[serde(rename = "userId")]
    pub user_id: u32,
}
