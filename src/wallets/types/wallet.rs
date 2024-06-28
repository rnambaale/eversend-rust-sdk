use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wallet {
    /// The currency of the wallet.
    pub currency: String,

    #[serde(rename = "currencyType")]
    pub currency_type: String,
    pub amount: u32,
    pub enabled: bool,

    /// The name of the wallet.
    pub name: String,
    pub icon: String,

    #[serde(rename = "amountInBaseCurrency")]
    pub amount_in_base_currency: u32,

    #[serde(rename = "isMain")]
    pub is_main: bool,
}
