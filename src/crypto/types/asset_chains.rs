use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssetChains {
    #[serde(rename = "Binance Smart Chain (BEP20)")]
    pub binance_smart_chain: String,

    #[serde(rename = "Ethereum (ERC20)")]
    pub ethereum: String,

    #[serde(rename = "TRON (TRC20)")]
    pub tron: String,
}
