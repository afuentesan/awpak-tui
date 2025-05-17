use serde_json::Value;

use crate::domain::{data::data_functions::string_from_input_data, error::Error, node::{node::Node, node_client::NodeClient, node_functions::create_node_client}};

use super::chain_client::ChainClientItem;

// Create NodeChainClient

pub async fn create_node_chain_client(
    node : &Node
) -> Result<NodeClient, Error>
{
    let id = uuid::Uuid::new_v4().to_string();

    Ok( create_node_client( id.as_str(), node ).await? )
}

// END Create NodeChainClient

// Input NodeChainClient

pub fn input_item_node_chain_client( item : &ChainClientItem, prompt : &str, context : &Value ) -> Result<(String, Value), Error>
{
    let prompt = item.input.iter()
    .fold( 
        "".to_string(), 
        | mut a, d |
        {
            match string_from_input_data( prompt, context, d )
            {
                Ok( s ) => 
                {
                    a.push_str( s.as_str() );

                    if let Some( sep ) = item.input_separator.as_ref()
                    {
                        a.push_str( sep );
                    }
                },
                _ => {}
            };

            a
        }
    );

    Ok( ( prompt, Value::Null ) )
}

// END Input NodeChainClient