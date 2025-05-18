use async_recursion::async_recursion;
use serde_json::Value;

use crate::{application::{chain::{chain::send_prompt_to_chain_client, chain_client::chain_existing_client}, node::{node::send_prompt_to_node_client, node_client::node_existing_client}}, domain::{chat::chat::ChatChannel, data::data_utils::merge_values, error::Error, node::node_functions::trace_node_prompt, repeat::{repeat_client::{RepeatClient, RepeatClientProvider}, repeat_functions::input_for_repeat}}};

use super::repeat_client::repeat_existing_client;

#[async_recursion]
pub async fn send_prompt_to_repeat_client<T>( 
    client : RepeatClient, 
    prompt : &str,
    context : Value,
    chat_channel : T
) -> Result<Vec<( String, Value )>, Error>
where T: ChatChannel + Send + Sync
{
    let inputs = input_for_repeat( &client, prompt, &context )?;

    let mut output = vec![];

    for input in inputs.into_iter()
    {
        output.push( send_prompt_to_repeat( &client, input, &context, chat_channel.clone() ).await? );
    }

    Ok( output )
}

async fn send_prompt_to_repeat<T>(
    client : &RepeatClient,
    prompt : String,
    context : &Value,
    chat_channel : T
) -> Result<(String, Value), Error>
where T: ChatChannel + Send + Sync
{
    match &*client.provider
    {
        RepeatClientProvider::Node( n ) => send_prompt_to_repeat_node( n, &prompt, chat_channel ).await,
        RepeatClientProvider::Chain( c ) => send_prompt_to_repeat_chain( c, &prompt, context, chat_channel ).await,
        RepeatClientProvider::Repeat( r ) => send_prompt_to_repeat_repeat( r, &prompt, context, chat_channel ).await,
    }
}

async fn send_prompt_to_repeat_node<T>( 
    client_id : &str, 
    prompt : &str,
    chat_channel : T
) -> Result<(String, Value), Error>
where T: ChatChannel + Send + Sync
{
    let client = node_existing_client( client_id )?;

    trace_node_prompt( &client.output, format!( "\n\nGenerated prompt:\n{}", &prompt ).as_str(), &chat_channel );

    let output_str = send_prompt_to_node_client( client, prompt, chat_channel ).await?;

    Ok( ( output_str, Value::Null ) )
}

async fn send_prompt_to_repeat_chain<T>( 
    client_id : &str, 
    prompt : &str,
    context : &Value,
    chat_channel : T
) -> Result<(String, Value), Error>
where T: ChatChannel + Send + Sync
{
    let client = chain_existing_client( client_id )?;

    send_prompt_to_chain_client( client, &prompt, context.clone(), chat_channel ).await
}

async fn send_prompt_to_repeat_repeat<T>( 
    client_id : &str, 
    prompt : &str,
    context : &Value,
    chat_channel : T
) -> Result<(String, Value), Error>
where T: ChatChannel + Send + Sync
{
    let client = repeat_existing_client( client_id )?;

    let arr = send_prompt_to_repeat_client( client, prompt, context.clone(), chat_channel ).await?;

    Ok(
        arr.into_iter().fold(
            ( "".to_string(), Value::Null ), 
            | mut a, ( s, v ) |
            {
                a.0.push_str( s.as_str() );

                a.1 = merge_values( Some( v ), a.1 );

                a
            }
        )
    )
}