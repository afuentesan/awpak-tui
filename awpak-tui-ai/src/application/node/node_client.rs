use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use rig::message::Message;

use crate::domain::{error::Error, mcp::mcp_functions::add_clients_from_servers, node::{node::{Node, NodeProvider}, node_client::{NodeClient, NodeClientProvider}, ollama_node_client::OllamaConfig, openai_node_client::OpenAIConfig}};

// MANAGE NODE CLIENTS

pub async fn node_client( id : &str, node : &Node ) -> Result<NodeClient, Error>
{
    match node_existing_client( id )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_node_client( id, node ).await?;

            node_clients().lock().unwrap().insert( id.to_string(), client.clone() );

            Ok( client )
        }
    }
}

fn node_clients() -> &'static Arc<Mutex<HashMap<String, NodeClient>>>
{
    static NODE_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, NodeClient>>>> = OnceLock::new();
    NODE_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn node_existing_client( id : &str ) -> Result<NodeClient, Error>
{
    match node_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

// END MANAGE NODE CLIENTS

// CREATE NODE CLIENT

async fn create_node_client( 
    id : &str, 
    node : &Node
) -> Result<NodeClient, Error>
{
    Ok(
        NodeClient 
        { 
            id : id.to_string(), 
            history : vec![], 
            save_history: node.save_history, 
            output : node.output.clone(),
            provider : create_node_client_provider( node ).await?
        }
    )
}

async fn create_node_client_provider( 
    node : &Node
) -> Result<NodeClientProvider, Error>
{
    match &node.provider
    {
        NodeProvider::Ollama( c ) => ollama_node_client( node, c ).await,
        NodeProvider::OpenAI( c ) => openai_node_client( node,c ).await,
        NodeProvider::Empty => return Err( Error::AgentErr( "AgentErr: Empty NodeProvider".into() ) )    
    }
}

async fn ollama_node_client( 
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

async fn openai_node_client( 
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

// END CREATE NODE CLIENT

// NODE HISTORY

pub fn save_node_history( id : &str ) -> Box<dyn FnOnce( Vec<Message> )>
{
    let id = id.to_string();

    Box::new(
        move | h |
        {
            fn_save_node_history( id, h )
        }
    )
}

fn fn_save_node_history( id : String, history : Vec<Message> )
{
    match node_existing_client( id.as_str() )
    {
        Ok( c ) => save_history( id, c, history ),
        _ => {}
    }
}

fn save_history( id : String, mut client : NodeClient, new_history : Vec<Message> )
{
    client.history = if client.save_history { new_history } else { client.history };

    node_clients().lock().unwrap().insert( id, client );
}

// END NODE HISTORY