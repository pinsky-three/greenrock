use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub api_key: String,
    pub api_secret: String,
    pub api_key_id: String,
    pub api_secret_key: String,
}
