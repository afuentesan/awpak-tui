use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use async_recursion::async_recursion;

use crate::{application::{chain::chain_client::chain_client, node::node_client::node_client}, domain::{agent::agent::AIAgent, error::Error, repeat::{repeat::Repeat, repeat_client::{RepeatClient, RepeatClientProvider}}}};

// MANAGE REPEAT CLIENTS

pub async fn repeat_client( id : &str, repeat : &Repeat ) -> Result<RepeatClient, Error>
{
    match repeat_existing_client( id )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_repeat_client( id, repeat ).await?;

            repeat_clients().lock().unwrap().insert( id.to_string(), client.clone() );

            Ok( client )
        }
    }
}

fn repeat_clients() -> &'static Arc<Mutex<HashMap<String, RepeatClient>>>
{
    static REPEAT_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, RepeatClient>>>> = OnceLock::new();
    REPEAT_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn repeat_existing_client( id : &str ) -> Result<RepeatClient, Error>
{
    match repeat_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

// END MANAGE REPEAT CLIENTS

// CREATE REPEAT CLIENT

#[async_recursion]
async fn create_repeat_client( 
    id : &str, 
    repeat : &Repeat
) -> Result<RepeatClient, Error>
{
    Ok(
        RepeatClient 
        { 
            id : id.to_string(), 
            provider : Box::new( create_repeat_provider( &repeat.agent ).await? ),
            input : repeat.input.clone()
        }
    )
}

async fn create_repeat_provider(
    agent : &AIAgent
) -> Result<RepeatClientProvider, Error>
{
    match agent
    {
        AIAgent::Node( n ) =>
        {
            let id = uuid::Uuid::new_v4().to_string();

            let node_client = node_client( id.as_str(), n ).await?;

            Ok( RepeatClientProvider::Node( node_client.id ) )
        },
        AIAgent::Chain( c ) =>
        {
            let id = uuid::Uuid::new_v4().to_string();

            let chain_client = chain_client( id.as_str(), c ).await?;

            Ok( RepeatClientProvider::Chain( chain_client.id ) )
        },
        AIAgent::Repeat( r ) =>
        {
            let id = uuid::Uuid::new_v4().to_string();

            let repeat_client = repeat_client( id.as_str(), r ).await?;

            Ok( RepeatClientProvider::Repeat( repeat_client.id ) )
        }
    }
}

// END CREATE REPEAT CLIENT