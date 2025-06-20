use serde::{Deserialize, Serialize};

use crate::domain::mcp::mcp::NodeMCPServer;

use super::{anthropic_node_client::AnthropicConfig, deepseek_node_client::DeepSeekConfig, gemini_node_client::GeminiConfig, ollama_node_client::OllamaConfig, openai_node_client::OpenAIConfig};


#[derive(Serialize, Deserialize, Clone)]
pub struct Node
{
    #[serde(default)]
    pub name : String,

    pub provider : NodeProvider,

    #[serde(default)]
    pub system_prompt : Option<String>,
    #[serde(default)]
    pub save_history : bool,

    #[serde(default)]
    pub servers : Vec<NodeMCPServer>,

    #[serde(default)]
    pub output : NodeOutputDestination,

    #[serde(default)]
    pub tools_output : Option<NodeOutputDestination>,

    #[serde(default)]
    pub millis_between_tool_call : Option<u64>,

    #[serde(default)]
    pub millis_between_streams : Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NodeProvider
{
    Ollama( OllamaConfig ),
    OpenAI( OpenAIConfig ),
    Anthropic( AnthropicConfig ),
    DeepSeek( DeepSeekConfig ),
    Gemini( GeminiConfig ),
    Empty
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum NodeOutputDestination
{
    #[default]
    Channel,
    Log( String ),
    Ignore
}

impl Default for Node
{
    fn default() -> Self 
    {
        Self
        { 
            name : "".into(), 
            provider : NodeProvider::Empty, 
             
            save_history : false, 
            system_prompt : None,

            servers : vec![],
            output : NodeOutputDestination::default(),

            tools_output : None,
            millis_between_tool_call : None,
            
            millis_between_streams : None
        }
    }
}