use serde::{Deserialize, Serialize};

use crate::domain::mcp::mcp::NodeMCPServer;

use super::{ollama_node_client::OllamaConfig, openai_node_client::OpenAIConfig};


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
    pub output : NodeOutputDestination
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NodeProvider
{
    Ollama( OllamaConfig ),
    OpenAI( OpenAIConfig ),
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
            output : NodeOutputDestination::default() 
        }
    }
}