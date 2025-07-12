use std::collections::HashMap;

use serde_json::{Map, Number, Value};

use crate::domain::{data::{data::DataType, data_operations::f64_from_value}, error::Error, path::expand_path::expand_path};

pub fn values_are_equals( val_1 : &Value, val_2 : &Value ) -> bool
{
    let n1 = f64_from_value( val_1.clone() );
    let n2 = if n1.is_ok() { f64_from_value( val_2.clone() ) } else { Err( Error::Ignore ) };

    match ( n1, n2 )
    {
        ( Ok( n1 ), Ok( n2 ) ) => n1 == n2,
        _ => val_1.to_string() == val_2.to_string()    
    }
}

pub fn value_to_string( val : &Value ) -> String
{
    match val
    {
        Value::Null => "".into(),
        Value::String( s ) => s.clone(),
        _ => val.to_string()    
    }
}

pub fn value_is_null( val : &Value ) -> bool
{
    match val
    {
        Value::Null => true,
        _ => false    
    }
}

pub fn value_from_map( path : &str, map : &HashMap<String, Value> ) -> Result<Value, Error>
{
    let expanded_path = expand_path( map, path )?;

    value_from_map_path_expanded( &expanded_path, map )
}

pub fn value_from_map_path_expanded(
    path : &str,
    map : &HashMap<String, Value>
) -> Result<Value, Error>
{
    if path.trim() == "" || path.trim() == "/"
    {
        return Ok( 
            Value::Object(
                map.iter().fold(
                    Map::new(), 
                    | mut a, ( k, v ) |
                    {
                        a.insert( k.clone(), v.clone() );

                        a
                    }
                )
            )
        )
    }

    let parts = path.split( "/" ).filter( | s | *s != "" ).collect::<Vec<_>>();

    if parts.len() == 0 { return Err( Error::ParseData( format!( "Invalid path: {}", path ) ) ) }

    if parts.len() == 1
    {
        return match map.get( parts[ 0 ] )
        {
            Some( v ) => Ok( v.clone() ),
            _ => Err( Error::ParseData( format!( "Path not found in context: {}", path ) ) )
        }
    }

    match map.get( parts[ 0 ] )
    {
        Some( v ) =>
        {
            let pointer = format!( "/{}", parts[ 1.. ].join( "/" ) );

            match v.pointer( pointer.as_str() )
            {
                Some( v ) => Ok( v.clone() ),
                _ => Err( Error::ParseData( format!( "Path not found in context: {}", path ) ) )
            }
        },
        _ => Err( Error::ParseData( format!( "Path not found in context: {}", path ) ) )
    }
}

pub fn str_to_value( 
    input : &str,
    val_type : &DataType,
    optional : bool
) -> Result<Value, Error>
{
    if optional && input == "" { return Ok( Value::Null ) }

    match val_type
    {
        DataType::Null => Ok( Value::Null ),
        DataType::String => Ok( Value::String( input.to_string() ) ),
        DataType::Bool => str_to_bool_value( input ),
        DataType::Object => str_to_object_value( input ),
        DataType::Array => str_to_array_value( input ),
        DataType::Number => str_to_number_value( input )
    }
}

fn str_to_number_value( input : &str ) -> Result<Value, Error>
{
    Ok(
        Value::Number( serde_json::from_str::<Number>( input.trim() ).map_err( | e | Error::ParseData( e.to_string() ) )? )
    )
}

fn str_to_array_value( input : &str ) -> Result<Value, Error>
{
    Ok(
        Value::Array( serde_json::from_str::<Vec<Value>>( input ).map_err( | e | Error::ParseData( e.to_string() ) )? )
    )
}

fn str_to_object_value( input : &str ) -> Result<Value, Error>
{
    Ok(
        Value::Object( serde_json::from_str::<Map<String, Value>>( input ).map_err( | e | Error::ParseData( e.to_string() ) )? )
    )
}

fn str_to_bool_value( input : &str ) -> Result<Value, Error>
{
    match input.trim().parse::<bool>()
    {
        Ok( b ) => Ok( Value::Bool( b ) ),
        Err( e ) => Err( Error::ParseData( e.to_string() ) )
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_value_from_map_inner_map()
    {
        let mut map = HashMap::new();

        map.insert( "x".to_string(), Value::Number( Number::from_u128( 0 ).unwrap() ) );

        map.insert( "first_key".into(), Value::String( "inner".into() ) );

        let mut inner_map_1 = Map::new();

        inner_map_1.insert( "result".into(), Value::String( "Ok".into() ) );
        inner_map_1.insert( "second_key".into(), Value::String( "result".into() ) );

        map.insert( "inner".into(), Value::Object( inner_map_1 ) );

        let path = "{/first_key}/{/inner/second_key/}";

        let value = value_from_map( path, &map );

        assert!( value.is_ok() );

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )
        }

        let path = "{first_key}/{inner/second_key/}";

        let value = value_from_map( path, &map );

        assert!( value.is_ok() );

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )
        }

        let path = "{first_key/}/{/inner/second_key}";

        let value = value_from_map( path, &map );

        assert!( value.is_ok() );

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )
        }
    }

    #[test]
    fn test_value_from_map_inner_array()
    {
        let mut map = HashMap::new();

        map.insert( "x".to_string(), Value::Number( Number::from_u128( 3 ).unwrap() ) );

        map.insert( "first_key".into(), Value::String( "inner".into() ) );

        let mut inner_map_1 = Map::new();

        inner_map_1.insert( "result".into(), Value::String( "Fake Ok".into() ) );
        inner_map_1.insert( "second_key".into(), Value::String( "result_2".into() ) );

        let inner_array = vec![ 
            Value::String( "Error".into() ),
            Value::String( "Error".into() ),
            Value::String( "Error".into() ),
            Value::String( "Ok".into() ),
            Value::String( "Error".into() )
        ];

        inner_map_1.insert( "result_2".into(), Value::Array( inner_array ) );

        map.insert( "inner".into(), Value::Object( inner_map_1 ) );

        let path = "{/first_key}/{/inner/second_key/}/{x}";

        let value = value_from_map( path, &map );

        assert!( value.is_ok() );

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )
        }
    }
}