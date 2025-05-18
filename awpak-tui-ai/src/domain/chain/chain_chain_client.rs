use serde_json::Value;

use crate::domain::error::Error;

use super::{chain_client::ChainClientItem, node_chain_client::input_item_node_chain_client};

// Input ChainChainClient

pub fn input_item_chain_chain_client( item : &ChainClientItem, prompt : &str, context : &Value ) -> Result<(String, Value), Error>
{
    let prompt = input_item_node_chain_client( item, prompt, context )?.0;

    Ok( ( prompt, context.clone() ) )
}

// END Input ChainChainClient