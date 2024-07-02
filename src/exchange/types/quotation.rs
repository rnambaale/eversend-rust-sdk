use serde::{Deserialize, Serialize};

use crate::wallets::WalletId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    /// Amount of source currency
    pub amount: u32,

    /// Source currency from Get Wallets
    pub from: WalletId,

    /// Destination currency from Get Wallets
    pub to: WalletId,

    // TODO: Get actual quotation response attributes
}
