use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use async_recursion::async_recursion;

use crate::{application::{node::node_client::node_client, repeat::repeat_client::repeat_client}, domain::{agent::agent::AIAgent, chain::{chain::{Chain, ChainItem}, chain_client::{ChainClient, ChainClientItem, ChainClientProvider}}, error::Error}};

// MANAGE CHAIN CLIENTS

pub async fn chain_client( id : &str, chain : &Chain ) -> Result<ChainClient, Error>
{
    match chain_existing_client( id )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_chain_client( id, chain ).await?;

            chain_clients().lock().unwrap().insert( id.to_string(), client.clone() );

            Ok( client )
        }
    }
}

fn chain_clients() -> &'static Arc<Mutex<HashMap<String, ChainClient>>>
{
    static CHAIN_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, ChainClient>>>> = OnceLock::new();
    CHAIN_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn chain_existing_client( id : &str ) -> Result<ChainClient, Error>
{
    match chain_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

// END MANAGE CHAIN CLIENTS

// CREATE CHAIN

#[async_recursion]
async fn create_chain_client( 
    id : &str, 
    chain : &Chain
) -> Result<ChainClient, Error>
{
    Ok(
        ChainClient 
        { 
            id : id.to_string(), 
            items : create_chain_client_items( &chain.items ).await?,
            initial_context : chain.initial_context.clone()
        }
    )
}

async fn create_chain_client_items(
    items : &Vec<ChainItem>
) -> Result<Vec<ChainClientItem>, Error>
{
    let mut ret = vec![];

    for item in items
    {
        ret.push( create_chain_client_item( item ).await? )
    }

    Ok( ret )
}

async fn create_chain_client_item(
    item : &ChainItem
) -> Result<ChainClientItem, Error>
{
    Ok(
        ChainClientItem
        {
            id : uuid::Uuid::new_v4().to_string(),
            
            input : item.input.clone(),

            input_separator : item.input_separator.clone(),
            
            output : item.output.clone(),

            provider : create_chain_client_provider( &item.agent ).await?
        }
    )
}

async fn create_chain_client_provider(
    agent : &AIAgent
) -> Result<ChainClientProvider, Error>
{
    match agent
    {
        AIAgent::Node( n ) =>
        {
            let id = uuid::Uuid::new_v4().to_string();

            let client = node_client( id.as_str(), n ).await?;

            Ok( ChainClientProvider::Node( client.id ) )
        },
        AIAgent::Chain( c ) => 
        {
            let id = uuid::Uuid::new_v4().to_string();

            let client = chain_client( id.as_str(), c ).await?;

            Ok( ChainClientProvider::Chain( client.id ) )
        },
        AIAgent::Repeat( r ) => 
        {
            let id = uuid::Uuid::new_v4().to_string();

            let client = repeat_client( id.as_str(), r ).await?;

            Ok( ChainClientProvider::Repeat( client.id ) )
        },
        AIAgent::Command( _ ) => todo!()
    }
}

// END CREATE CHAIN