use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wallet {
    /// The currency of the wallet.
    pub currenncy: String,

    pub currency_type: String,
    pub amount: u32,
    pub enabled: bool,

    /// The name of the wallet.
    pub name: String,
    pub icon: String,
    pub amount_inase_currency: u32,
    pub is_main: bool,
}
