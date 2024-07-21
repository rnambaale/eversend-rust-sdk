use serde::Deserialize;

#[derive(Deserialize)]
pub struct MobileMoneyCollection {
    pub amount: String,

    #[serde(rename = "balanceAfter")]
    pub balance_after: Option<String>,

    #[serde(rename = "balanceBefore")]
    pub balance_before: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    pub currency: String,

    pub customer: Option<serde_json::Value>,

    pub status: String,

    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "transactionRef")]
    pub transaction_ref: String,

    #[serde(rename = "type")]
    pub transaction_type: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
