use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AnthropicConfig
{
    pub api_key : String,
    pub model : String,
    pub max_tokens : u64
}