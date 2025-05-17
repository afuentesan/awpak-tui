
// Create RepeatChainClient

use serde_json::Value;

use crate::domain::{error::Error, repeat::{repeat::Repeat, repeat_client::RepeatClient, repeat_functions::create_repeat_client}};

use super::chain_client::ChainClientItem;

pub async fn create_repeat_chain_client(
    repeat : &Repeat
) -> Result<RepeatClient, Error>
{
    let id = uuid::Uuid::new_v4().to_string();

    Ok( create_repeat_client(  id.as_str(), repeat ).await? )
}

// END Create RepeatChainClient

// Input RepeatChainClient

pub fn input_item_repeat_chain_client( item : &ChainClientItem, prompt : &str, context : &Value ) -> Result<(String, Value), Error>
{
    todo!()

    // Ok( ( "".to_string(), Value::Null ) )
}

// END Input RepeatChainClient