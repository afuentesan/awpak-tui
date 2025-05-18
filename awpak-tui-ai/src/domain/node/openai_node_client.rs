use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenAIConfig
{
    pub api_key : String,
    pub model : String
}

