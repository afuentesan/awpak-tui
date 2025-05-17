use async_recursion::async_recursion;
use serde_json::Value;

use crate::domain::{agent::agent::AIAgent, data::{data::OutputData, data_functions::merge_data}, error::Error};

use super::{chain::{Chain, ChainItem}, chain_chain_client::{create_chain_chain_client, input_item_chain_chain_client}, chain_client::{ChainClient, ChainClientItem, ChainClientProvider}, node_chain_client::{create_node_chain_client, input_item_node_chain_client}, repeat_chain_client::{create_repeat_chain_client, input_item_repeat_chain_client}};

// CREATE CHAIN

#[async_recursion]
pub async fn create_chain_client( 
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
        AIAgent::Node( n ) => Ok( ChainClientProvider::Node( create_node_chain_client( n ).await? ) ),
        AIAgent::Chain( c ) => Ok( ChainClientProvider::Chain( create_chain_chain_client( c ).await? ) ),
        AIAgent::Repeat( r ) => Ok( ChainClientProvider::Repeat( create_repeat_chain_client( r ).await? ) )
    }
}

// END CREATE CHAIN

// MANAGE INPUT

pub fn input_item_chain_client( item : &ChainClientItem, prompt : &str, context : &Value ) -> Result<(String, Value), Error>
{
    match &item.provider
    {
        ChainClientProvider::Node( _ ) => input_item_node_chain_client( item, prompt, context ),
        ChainClientProvider::Chain( _ ) => input_item_chain_chain_client( item, prompt, context ),
        ChainClientProvider::Repeat( _ ) => input_item_repeat_chain_client( item, prompt, context )
    }
}

// END MANAGE INPUT

// MANAGE OUTPUT

pub fn merge_output_item_chain_client( 
    item : &ChainClientItem,
    context : Value,
    output_str : &str,
    output_context : Value
) -> Result<Value, Error>
{
    match &item.output 
    {
        OutputData::Output( d ) => merge_data( d, Value::String( output_str.to_string() ), context ),
        OutputData::Context( d ) => merge_data( d, output_context, context ),
        OutputData::None => Ok( context )
    }
}

// END MANAGE OUTPUT