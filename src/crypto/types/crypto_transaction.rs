use serde::Deserialize;

use super::CryptoAddress;

#[derive(Deserialize)]
pub struct CryptoTransaction {
    #[serde(rename = "accountId")]
    pub account_id: u32,

    pub address: CryptoAddress,

    #[serde(rename = "addressId")]
    pub address_id: u32,

    pub amount: String,

    pub id: u32,

    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    pub meta: TransactionMetaData,

    pub status: String,

    #[serde(rename = "subStatus")]
    pub sub_status: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct TransactionMetaData {
    #[serde(rename = "actualCoin")]
    pub actual_coin: String,

    pub amount: u32,

    #[serde(rename = "blockchainHash")]
    pub blockchain_hash: String,

    #[serde(rename = "blockchainStatus")]
    pub blockchain_status: String,

    #[serde(rename = "blockchainSubStatus")]
    pub blockchain_sub_status: String,

    pub charges: u32,

    pub country: String,

    #[serde(rename = "creationDate")]
    pub creation_date: String,

    pub currency: String,

    pub date: String,

    #[serde(rename = "eversendRef")]
    pub eversend_ref: String,

    pub fees: String,

    #[serde(rename = "fireblocksId")]
    pub fireblocks_id: String,

    pub processor: String,

    pub source: String,

    #[serde(rename = "toppedUp")]
    pub topped_up: bool,

    #[serde(rename = "toppedUpDate")]
    pub topped_up_date: String,

    #[serde(rename = "totalToPay")]
    pub total_to_pay: u32,

    #[serde(rename = "type")]
    pub transaction_type: String,

    pub username: String,
}
