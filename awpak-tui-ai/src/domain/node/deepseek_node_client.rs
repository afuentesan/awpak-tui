use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeepSeekConfig
{
    pub api_key : String,
    pub model : String,
    #[serde(default)]
    pub max_tokens : Option<u64>
}