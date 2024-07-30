use serde::Deserialize;

#[derive(Deserialize)]
pub struct CryptoAddress {
    pub address: String,

    pub coin: String,

    #[serde(rename = "destinationAddressDescription")]
    pub destination_address_description: String,

    pub purpose: String,

    #[serde(rename = "ownerName")]
    pub owner_name: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
