use serde::Deserialize;

#[derive(Deserialize)]
pub struct Quotation {

    pub amount: u32,

    #[serde(rename = "amountType")]
    pub amount_type: String,

    #[serde(rename = "destinationAmount")]
    pub destination_amount: String,

    #[serde(rename = "destinationCountry")]
    pub destination_country: String,

    #[serde(rename = "destinationCurrency")]
    pub destination_currency: String,

    #[serde(rename = "exchangeRate")]
    pub exchange_rate: String,

    #[serde(rename = "sourceAmount")]
    pub source_amount: String,

    #[serde(rename = "sourceCountry")]
    pub source_country: String,

    #[serde(rename = "sourceCurrency")]
    pub source_currency: String,

    #[serde(rename = "totalAmount")]
    pub total_amount: String,

    #[serde(rename = "totalFees")]
    pub total_fees: String,

    #[serde(rename = "type")]
    pub transaction_type: String,
}
