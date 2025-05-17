
// CREATE REPEAT

use async_recursion::async_recursion;

use crate::domain::{agent::agent::AIAgent, chain::chain_functions::create_chain_client, error::Error, node::node_functions::create_node_client};

use super::{repeat::Repeat, repeat_client::{RepeatClient, RepeatClientProvider}};

#[async_recursion]
pub async fn create_repeat_client( 
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

            Ok( RepeatClientProvider::Node( create_node_client( id.as_str(), n ).await? ) )
        },
        AIAgent::Chain( c ) =>
        {
            let id = uuid::Uuid::new_v4().to_string();

            Ok( RepeatClientProvider::Chain( create_chain_client( id.as_str(), c ).await? ) )
        },
        AIAgent::Repeat( r ) =>
        {
            let id = uuid::Uuid::new_v4().to_string();

            Ok( RepeatClientProvider::Repeat( create_repeat_client( id.as_str(), r ).await? ) )
        }
    }
}

// END CREATE REPEAT