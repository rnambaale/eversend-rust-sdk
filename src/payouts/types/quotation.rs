use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Quotation {

    pub amount: String,

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

    pub merchant: Option<Merchant>
}

#[derive(Deserialize, Debug)]
pub struct Merchant {
    pub result: String,

    #[serde(rename = "merchantExists")]
    pub merchant_exists: bool,

    pub country: String,

    #[serde(rename = "defaultWallet")]
    pub default_wallet: String,

    #[serde(rename = "isMerchant")]
    pub is_merchant: bool,

    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,

    pub email: String,

    #[serde(rename = "phoneNumber")]
    pub phone_number: PhoneNumber,

    pub tag: String,
}

#[derive(Deserialize, Debug)]
pub struct PhoneNumber {
    pub prefix: String,
    pub number: String,
}
