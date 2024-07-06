use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseBody<T> {
    pub code: u16,
    pub data: Option<T>,
    pub success: bool,
}
