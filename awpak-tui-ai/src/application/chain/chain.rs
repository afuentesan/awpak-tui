use async_recursion::async_recursion;
use serde_json::Value;

use crate::{application::node::node::send_prompt_to_node_client, domain::{chain::{chain_client::{ChainClient, ChainClientItem, ChainClientProvider}, chain_functions::{input_item_chain_client, merge_output_item_chain_client}}, chat::chat::ChatChannel, data::data_utils::merge_values, error::Error, node::node_client::NodeClient}};

use super::chain_client::save_node_chain_history;

#[async_recursion]
pub async fn send_prompt_to_chain_client<T>( 
    client : ChainClient, 
    prompt : &str,
    mut context : Value,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    context = merge_values( client.initial_context.clone(), context );

    let mut output = String::new();

    for item in client.items
    {
        ( output, context ) = send_prompt_to_chain_client_item( 
            client.id.as_str(), item, prompt, context, chat_channel.clone()
        ).await?;
    }

    Ok( ( output, context ) )
}

async fn send_prompt_to_chain_client_item<T>(
    client_id : &str,
    item : ChainClientItem,
    prompt : &str,
    context : Value,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    let ( input_prompt, input_context ) = input_item_chain_client( &item, prompt, &context )?;

    let ( output_str, output_context ) : ( String, Value ) = match &item.provider
    {
        ChainClientProvider::Node( n ) =>
        {
             send_prompt_to_node_chain_client( client_id, &item.id, n, input_prompt, chat_channel ).await?
        }
        ChainClientProvider::Chain( c ) => 
        {
            send_prompt_to_chain_chain_client( c, input_prompt, input_context, chat_channel ).await?
        },
        ChainClientProvider::Repeat( r ) =>
        {
            todo!()
        }
    };

    let context = merge_output_item_chain_client( &item, context, &output_str, output_context )?;

    Ok( ( output_str, context ) )
}

async fn send_prompt_to_node_chain_client<T>(
    client_id : &str,
    item_id : &str,
    node : &NodeClient,
    prompt : String,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    let ( output_str, history ) = send_prompt_to_node_client( node.clone(), prompt.as_str(), chat_channel ).await?;

    save_node_chain_history( client_id, item_id, &node.id, history );
    // save_node_history( &node.id )( history );

    Ok( ( output_str, Value::Null ) )
}

async fn send_prompt_to_chain_chain_client<T>(
    node : &ChainClient,
    prompt : String,
    context : Value,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    send_prompt_to_chain_client( node.clone(), &prompt, context, chat_channel ).await
}
