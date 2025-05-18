use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaConfig
{
    pub model : String
}