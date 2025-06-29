use rig::message::Message;
use serde::{Deserialize, Serialize};

use crate::domain::{data::data::DataToString, mcp::mcp::NodeMCPServer};


#[derive(Serialize, Deserialize, Clone)]
pub struct AIAgent
{
    pub provider : AIAgentProviderConfig,

    #[serde(default)]
    pub system_prompt : Option<String>,
    #[serde(default)]
    pub save_history : bool,

    #[serde(default)]
    pub servers : Vec<NodeMCPServer>,

    #[serde(default)]
    pub prompt : Vec<DataToString>,

    #[serde(default)]
    pub history : Vec<Message>
}

impl Default for AIAgent
{
    fn default() -> Self 
    {
        Self 
        { 
            provider : AIAgentProviderConfig::Ollama( OllamaConfig { model : "llama3.1".into() } ), 
            system_prompt : None, 
            save_history : false, 
            servers : vec![], 
            prompt : vec![], 
            history: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AIAgentProviderConfig
{
    Ollama( OllamaConfig ),
    OpenAI( OpenAIConfig ),
    Anthropic( AnthropicConfig ),
    DeepSeek( DeepSeekConfig ),
    Gemini( GeminiConfig )
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaConfig
{
    pub model : String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenAIConfig
{
    pub api_key : String,
    pub model : String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnthropicConfig
{
    pub api_key : String,
    pub model : String,
    pub max_tokens : u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeepSeekConfig
{
    pub api_key : String,
    pub model : String,
    #[serde(default)]
    pub max_tokens : Option<u64>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeminiConfig
{
    pub api_key : String,
    pub model : String
}