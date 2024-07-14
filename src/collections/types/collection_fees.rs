use serde::Deserialize;

#[derive(Deserialize)]
pub struct CollectionFees {
    pub amount: String,
    pub amount_available_to_load: String,
    pub charges: String,
    pub currency: String,
    pub max_load_amount: String,
    pub max_limit: String,
    pub min_load_amount: String,
    pub new_balance: String,
    pub payment_method: String,
    pub total_to_pay: String,
}
