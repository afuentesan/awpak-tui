use serde_json::Value;

use crate::domain::error::Error;

use super::{chain::Chain, chain_client::{ChainClient, ChainClientItem}, chain_functions::create_chain_client};

// Create ChainChainClient

pub async fn create_chain_chain_client(
    chain : &Chain
) -> Result<ChainClient, Error>
{
    let id = uuid::Uuid::new_v4().to_string();

    Ok( create_chain_client(  id.as_str(), chain ).await? )
}

// END Create ChainChainClient

// Input ChainChainClient

pub fn input_item_chain_chain_client( item : &ChainClientItem, prompt : &str, context : &Value ) -> Result<(String, Value), Error>
{
    todo!()

    // Ok( ( "".to_string(), Value::Null ) )
}

// END Input ChainChainClient