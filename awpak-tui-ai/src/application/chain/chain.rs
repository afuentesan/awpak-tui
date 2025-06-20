use async_recursion::async_recursion;
use serde_json::Value;

use crate::{application::{node::{node::send_prompt_to_node_client, node_client::node_existing_client}, repeat::{repeat::send_prompt_to_repeat_client, repeat_client::repeat_existing_client}}, domain::{chain::{chain_client::{ChainClient, ChainClientItem, ChainClientProvider}, chain_functions::{input_item_chain_client, merge_output_item_chain_client, merge_output_item_chain_repeat_client}}, chat::chat::ChatChannel, data::data_utils::merge_values, error::Error, node::{node_client::NodeClient, node_functions::trace_node_prompt}, repeat::repeat_client::RepeatClient}};

use super::chain_client::chain_existing_client;


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
            item, prompt, context, chat_channel.clone()
        ).await?;
    }

    Ok( ( output, context ) )
}

async fn send_prompt_to_chain_client_item<T>(
    item : ChainClientItem,
    prompt : &str,
    context : Value,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    let ( input_prompt, input_context ) = input_item_chain_client( &item, prompt, &context )?;

    let ( output_str, context ) : ( String, Value ) = match &item.provider
    {
        ChainClientProvider::Node( n ) =>
        {
            let n = node_existing_client( n )?;

            let ( output_str, output_context ) = send_prompt_to_node_chain_client( 
                n, input_prompt, chat_channel 
            ).await?;

            let context = merge_output_item_chain_client( &item, context, &output_str, output_context )?;

            ( output_str, context )
        }
        ChainClientProvider::Chain( c ) => 
        {
            let c = chain_existing_client( c )?;

            let ( output_str, output_context ) = send_prompt_to_chain_chain_client( 
                c, input_prompt, input_context, chat_channel 
            ).await?;

            let context = merge_output_item_chain_client( &item, context, &output_str, output_context )?;

            ( output_str, context )
        },
        ChainClientProvider::Repeat( r ) =>
        {
            let r = repeat_existing_client( r )?;

            let arr_results = send_prompt_to_repeat_chain_client(r, input_prompt, input_context, chat_channel.clone() ).await?;

            merge_output_item_chain_repeat_client( &item, context, arr_results )?
        }
    };

    Ok( ( output_str, context ) )
}

async fn send_prompt_to_node_chain_client<T>(
    node : NodeClient,
    prompt : String,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    trace_node_prompt( &node.output, format!( "\n\nGenerated prompt:\n{}", &prompt ).as_str(), &chat_channel );

    let output_str = send_prompt_to_node_client( node, prompt.as_str(), chat_channel ).await?;

    Ok( ( output_str, Value::Null ) )
}

async fn send_prompt_to_chain_chain_client<T>(
    chain : ChainClient,
    prompt : String,
    context : Value,
    chat_channel : T
) -> Result<( String, Value ), Error>
where T: ChatChannel + Send + Sync
{
    send_prompt_to_chain_client( chain, &prompt, context, chat_channel ).await
}

async fn send_prompt_to_repeat_chain_client<T>(
    repeat : RepeatClient,
    prompt : String,
    context : Value,
    chat_channel : T
) -> Result<Vec<( String, Value )>, Error>
where T: ChatChannel + Send + Sync
{
    send_prompt_to_repeat_client( repeat, prompt.as_str(), context, chat_channel ).await
}
