use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Transaction {

    #[serde(rename = "accountId")]
    pub account_id: u32,

    pub amount: String,

    #[serde(rename = "balanceAfter")]
    pub balance_after: String,

    #[serde(rename = "balanceBefore")]
    pub balance_before: String,

    pub beneficiary: Option<String>,

    #[serde(rename = "beneficiaryId")]
    pub beneficiary_id: Option<u32>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// Defaults to UGX
    pub currency: TransactionCurrencyOption,

    // pub customer: Option<String>,

    #[serde(rename = "destinationAmount")]
    pub destination_amount: String,

    #[serde(rename = "destinationCurrency")]
    pub destination_currency: Option<String>,

    #[serde(rename = "destinationCountry")]
    pub destination_country: Option<String>,

    pub fees: Option<String>,

    pub id: u32,

    #[serde(rename = "isRefunded")]
    pub is_refunded: bool,

    #[serde(rename = "merchantId")]
    pub merchant_id: Option<String>,

    pub meta: TransactionMetaData,

    #[serde(rename = "pesapotId")]
    pub pesapot_id: Option<String>,

    #[serde(rename = "pesapotResponse")]
    pub pesapot_response: Option<String>,

    pub reason: Option<String>,

    #[serde(rename = "remitOneId")]
    pub remit_one_id: Option<String>,

    #[serde(rename = "sourceCountry")]
    pub source_country: Option<String>,

    #[serde(rename = "sourceCurrency")]
    pub source_currency: Option<String>,

    pub status: TransactionStatusOption,

    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "type")]
    pub transaction_type: TransactionTypeOption,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub user: Option<String>,

    #[serde(rename = "userId")]
    pub user_id: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub enum TransactionCurrencyOption {
    GHS,
    KES,
    NGN,
    RWF,
    TZS,
    UGX,
    USD,
}

#[derive(Serialize, Deserialize)]
pub enum TransactionTypeOption {
    #[serde(rename = "collection")]
    COLLECTION,

    #[serde(rename = "exchange")]
    EXCHANGE,

    #[serde(rename = "payout")]
    PAYOUT,
}

#[derive(Serialize, Deserialize)]
pub enum TransactionStatusOption {
    #[serde(rename = "failed")]
    FAILED,

    #[serde(rename = "pending")]
    PENDING,

    #[serde(rename = "successful")]
    SUCCESSFUL,
}

#[derive(Serialize)]
pub enum TransactionRangeOption {
    #[serde(rename = "day")]
    DAY,

    #[serde(rename = "week")]
    WEEK,

    #[serde(rename = "month")]
    MONTH,

    #[serde(rename = "year")]
    YEAR
}

#[derive(Deserialize)]
pub struct TransactionMetaData {
    pub source: TransationAccount,
    pub destination: TransationAccount,
}

#[derive(Deserialize)]
pub struct TransationAccount {
    pub amount: f32,
    pub balance: AccountBalance,
    pub currency: TransactionCurrencyOption,
}

#[derive(Deserialize)]
pub struct AccountBalance {
    pub after: String,
    pub before: String,
}

