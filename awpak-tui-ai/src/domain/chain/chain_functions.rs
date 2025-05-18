use serde_json::Value;

use crate::domain::{data::{data::OutputData, data_functions::merge_data}, error::Error};

use super::{chain_chain_client::input_item_chain_chain_client, chain_client::{ChainClientItem, ChainClientProvider}, node_chain_client::input_item_node_chain_client, repeat_chain_client::input_item_repeat_chain_client};

// MANAGE INPUT

pub fn input_item_chain_client( item : &ChainClientItem, prompt : &str, context : &Value ) -> Result<(String, Value), Error>
{
    match &item.provider
    {
        ChainClientProvider::Node( _ ) => input_item_node_chain_client( item, prompt, context ),
        ChainClientProvider::Chain( _ ) => input_item_chain_chain_client( item, prompt, context ),
        ChainClientProvider::Repeat { .. } => input_item_repeat_chain_client( item, prompt, context )
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

pub fn merge_output_item_chain_repeat_client( 
    item : &ChainClientItem,
    context : Value,
    output : Vec<(String, Value)>
) -> Result<( String, Value), Error>
{
    let output_str = output.iter()
    .fold(
        "".to_string(), 
        | mut a, ( s, _ ) |
        {
            a.push_str( s );

            a
        }
    );

    match &item.output
    {
        OutputData::Output( d ) =>
        {
            let output_context = Value::Array( output.into_iter().map( | a | Value::String( a.0 ) ).collect::<Vec<_>>() );

            Ok( ( output_str, merge_data( d, output_context, context )? ) )
        },
        OutputData::Context( d ) =>
        {
            let output_context = Value::Array( output.into_iter().map( | a | a.1 ).collect::<Vec<_>>() );

            Ok( ( output_str, merge_data( d, output_context, context )? ) )
        },
        OutputData::None => Ok( ( output_str, context ) )
    }
}

// END MANAGE OUTPUT