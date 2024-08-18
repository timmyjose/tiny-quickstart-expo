use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ItemPublicTokenExchangeRequest {
    pub client_id: String,
    pub secret: String,
    pub public_token: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ItemPublicTokenExchangeResponse {
    pub access_token: String,
    pub item_id: String,
    pub request_id: String,
}
