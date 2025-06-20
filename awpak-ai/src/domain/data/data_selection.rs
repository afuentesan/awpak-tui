use std::collections::HashMap;

use awpak_utils::string_utils::str_len;
use serde_json::Value;

use crate::domain::{data::{data::{DataFrom, DataOperation, DataToString, FromContext, FromParsedInput}, data_insert::merge_values, data_operations::{add_values, substract_values}, data_utils::{value_from_map, value_is_null, value_to_string}}, error::Error, path::expand_path::expand_path};

pub fn data_to_string(
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    data : Vec<DataToString>
) -> String
{
    data.into_iter()
    .fold(
        "".to_string(), 
        | mut a, d |
        {
            a.push_str( &item_data_to_string( input, parsed_input, context, d ) );

            a
        }
    )
}

fn item_data_to_string(
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    data : DataToString
) -> String
{
    format!(
        "{}{}{}",
        data.prefix.unwrap_or( "".into() ),
        match data_selection( input, parsed_input, context, &data.from )
        {
            Ok( v ) => value_to_string( &v ),
            Err( e ) => e.to_string()
        },
        data.suffix.unwrap_or( "".into() )
    )
}

pub fn data_selection( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    from : &DataFrom
) -> Result<Value, Error>
{
    match from
    {
        DataFrom::Context( f ) => data_from_context( context, f ),
        DataFrom::ParsedInput( f ) => data_from_parsed_input( context, parsed_input, f ),
        DataFrom::Input { required } => data_from_input( input, *required ),
        DataFrom::Static( v ) => Ok( v.clone() ),
        DataFrom::Concat( f ) => concat_data_from( input, parsed_input, context, f ),
        DataFrom::Operation( o ) => operation_data_from( input, parsed_input, context, o )
    }
}

fn operation_data_from(
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    operation : &DataOperation
) -> Result<Value, Error>
{
    match operation
    {
        DataOperation::Len( f ) =>
        {
            let value = data_selection( input, parsed_input, context, f )?;

            match value
            {
                Value::Array( a ) => Ok( Value::from( a.len() ) ),
                Value::Number( n ) => Ok( Value::Number( n ) ),
                Value::Object( o ) => Ok( Value::from( o.len() ) ),
                Value::Null => Ok( Value::from( 0 ) ),
                Value::String( s ) => Ok( Value::from( str_len( s.as_str() ) ) ),
                Value::Bool( b ) => Ok( if b { Value::from( 1 as usize ) } else { Value::from( 0 as usize ) } )
            }
        },
        DataOperation::Substract { num_1, num_2 } =>
        {
            let v1 = data_selection( input, parsed_input, context, num_1 )?;
            let v2 = data_selection( input, parsed_input, context, num_2 )?;

            substract_values( v1, v2 )
        },
        DataOperation::Add { num_1, num_2 } =>
        {
            let v1 = data_selection( input, parsed_input, context, num_1 )?;
            let v2 = data_selection( input, parsed_input, context, num_2 )?;

            add_values( v1, v2 )
        }
    }
}

fn concat_data_from(
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    from : &Vec<DataFrom>
) -> Result<Value, Error>
{
    from.iter().try_fold(
        Value::Null, 
        | acc, from |
        {
            let value = data_selection( input, parsed_input, context, from )?;
            
            Ok( merge_values( acc, value ) )
        }
    )
}

fn data_from_context( context : &HashMap<String, Value>, from : &FromContext ) -> Result<Value, Error>
{
    let val = value_from_map( &from.path, context );

    if from.required && ( val.is_err() || value_is_null( val.as_ref().unwrap() ) )
    {
        return Err( Error::ParseData( format!( "Key not found in context. Key: {}. Error: {:?}", from.path, val.err() ) ) )
    }

    if val.is_err() { return Ok( Value::Null ) }

    Ok( val.unwrap() )
}

fn data_from_parsed_input( 
    context : &HashMap<String, Value>,
    parsed_input : &Value, 
    from : &FromParsedInput 
) -> Result<Value, Error>
{
    let value = match &from.path
    {
        Some( p ) =>
        {
            let path = expand_path( context, p )?;

            let value = parsed_input.pointer( path.as_str() );

            match value
            {
                Some( v ) => v.clone(),
                _ => Value::Null    
            }
        },
        _ => parsed_input.clone()
    };

    if from.required && value.is_null() 
    { 
        Err( Error::ParseData( format!( "Parsed input from {:?} is null", from.path ) ) ) 
    }
    else
    {
        Ok( value )    
    }
}

fn data_from_input( input : Option<&String>, required : bool ) -> Result<Value, Error>
{
    if required && ( input.is_none() || input.unwrap().trim() == "" )
    {
        return Err( Error::ParseData( "Input required".into() ) )
    }

    match input
    {
        Some( i ) => Ok( Value::String( i.trim().to_string() ) ),
        None => Ok( Value::Null )    
    }
}