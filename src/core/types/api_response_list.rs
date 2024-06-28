use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseList<T> {
    pub data: Vec<T>,
    pub code: u32,
}
