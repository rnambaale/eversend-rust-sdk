use serde::{Deserialize, Serialize};

use crate::wallets::WalletId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    #[serde(rename = "baseAmount")]
    pub base_amount: u32,

    #[serde(rename = "baseCurrency")]
    pub base_currency: WalletId,

    #[serde(rename = "baseWalletAfter")]
    pub base_wallet_after: f64,

    #[serde(rename = "baseWalletBefore")]
    pub base_wallet_before: f64,

    #[serde(rename = "destAmount")]
    pub dest_amount: f64,

    #[serde(rename = "destCurrency")]
    pub dest_currency: WalletId,

    #[serde(rename = "destWalletAfter")]
    pub dest_wallet_after: Option<f64>,

    #[serde(rename = "destWalletBefore")]
    pub dest_wallet_before: f64,

    pub rate: f64,
}
