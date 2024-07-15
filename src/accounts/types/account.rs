use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub town: String,
    pub country: String,
    pub logo: Option<String>,
    pub website: String,

    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}
