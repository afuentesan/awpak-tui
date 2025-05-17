use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::{error::Error, mcp::mcp_functions::add_clients_from_servers};

use super::{node::Node, node_client::NodeClientProvider};

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenAIConfig
{
    pub api_key : String,
    pub model : String
}

pub async fn openai_node_client( 
    node : &Node,
    config : &OpenAIConfig
) -> Result<NodeClientProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::AgentErr( e.to_string() ) )?;

    let client = rig::providers::openai::Client::new( &api_key );

    let agent = client.agent( &config.model );

    let mut agent = add_clients_from_servers( agent, &node.servers ).await?;

    if let Some( p ) = node.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }
    
    let agent = agent.build();

    Ok( NodeClientProvider::OpenAI( Arc::new( agent ) ) )
}