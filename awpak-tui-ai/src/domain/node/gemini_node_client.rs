use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GeminiConfig
{
    pub api_key : String,
    pub model : String
}