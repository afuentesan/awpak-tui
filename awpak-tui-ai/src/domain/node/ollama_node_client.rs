use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::{error::Error, mcp::mcp_functions::add_clients_from_servers};

use super::{node::Node, node_client::NodeClientProvider};


#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaConfig
{
    pub model : String
}

pub async fn ollama_node_client( 
    node : &Node,
    config : &OllamaConfig
) -> Result<NodeClientProvider, Error>
{
    let client = rig::providers::ollama::Client::new();

    let agent = client.agent( &config.model );

    let mut agent = add_clients_from_servers( agent, &node.servers ).await?;

    if let Some( p ) = node.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }

    Ok( NodeClientProvider::Ollama( Arc::new( agent.build() ) ) )
}