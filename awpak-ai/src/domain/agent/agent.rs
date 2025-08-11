use rig::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::{data::data::{DataFrom, DataToString}, mcp::mcp::NodeMCPServer};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIAgent
{
    pub provider : AIAgentProviderConfig,

    #[serde(default)]
    pub system_prompt : Vec<DataToString>,
    #[serde(default)]
    pub save_history : bool,

    #[serde(default)]
    pub servers : Vec<NodeMCPServer>,

    #[serde(default)]
    pub prompt : Vec<DataToString>,

    #[serde(default)]
    pub history : Vec<Message>,

    #[serde(default)]
    pub is_stream : bool,

    #[serde(default)]
    pub turns : Option<usize>,

    #[serde(default = "agent_default_id")]
    pub __id : String
}

fn agent_default_id() -> String
{
    uuid::Uuid::new_v4().to_string()
}

impl Default for AIAgent
{
    fn default() -> Self 
    {
        Self 
        { 
            provider : AIAgentProviderConfig::Ollama( OllamaConfig { model : DataFrom::Static( Value::String( "llama3.1".into() ) ) } ), 
            system_prompt : vec![], 
            save_history : false, 
            servers : vec![], 
            prompt : vec![], 
            history: vec![],
            is_stream : false,
            turns : None,
            // embeddings : vec![],
            __id : "".into()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AIAgentProviderConfig
{
    Ollama( OllamaConfig ),
    OpenAI( OpenAIConfig ),
    Anthropic( AnthropicConfig ),
    DeepSeek( DeepSeekConfig ),
    Gemini( GeminiConfig )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaConfig
{
    pub model : DataFrom
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIConfig
{
    pub api_key : String,
    pub model : DataFrom
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnthropicConfig
{
    pub api_key : String,
    pub model : DataFrom,
    pub max_tokens : u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeepSeekConfig
{
    pub api_key : String,
    pub model : DataFrom,
    #[serde(default)]
    pub max_tokens : Option<u64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeminiConfig
{
    pub api_key : String,
    pub model : DataFrom
}