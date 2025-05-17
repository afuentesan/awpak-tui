use serde_json::{Map, Value};

use crate::domain::error::Error;

use super::{data::{DataMerge, DataSelection, InputData}, data_utils::{add_footer, add_title, array_of_strings_from_map, array_of_strings_from_string, string_from_map, value_to_map, value_to_string}};

pub fn array_from_input_data( prompt : &str, context : &Value, input_data : &InputData ) -> Result<Vec<String>, Error>
{
    match input_data
    {
        InputData::Prompt { selection, title, footer } =>
        {
            array_of_strings_from_selection( &Value::String( prompt.to_string() ), selection, title.as_ref(), footer.as_ref() )
        },
        InputData::Context { selection, title, footer } =>
        {
            array_of_strings_from_selection( context, selection, title.as_ref(), footer.as_ref() )
        }
    }
}

pub fn string_from_input_data( prompt : &str, context : &Value, input_data : &InputData ) -> Result<String, Error>
{
    match input_data
    {
        InputData::Prompt { title, footer, selection } =>
        {
            let str = string_from_selection( &Value::String( prompt.to_string() ), selection )?;

            Ok( add_footer( add_title( str, title.as_ref() ), footer.as_ref() ) )
        },
        InputData::Context { title, footer, selection } =>
        {
            let str = string_from_selection( context, selection )?;

            Ok( add_footer( add_title( str, title.as_ref() ), footer.as_ref() ) )
        }
    }
}

fn array_of_strings_from_selection( 
    source : &Value, 
    selection : &DataSelection,
    title : Option<&String>, 
    footer : Option<&String>
) -> Result<Vec<String>, Error>
{
    match selection
    {
        DataSelection::FromMap( k ) => array_of_strings_from_map( k, source, title, footer ),
        DataSelection::FromString => array_of_strings_from_string( source, title, footer ),
        DataSelection::None => Ok( vec![] )
    }
}

fn string_from_selection( source : &Value, selection : &DataSelection ) -> Result<String, Error>
{
    match selection
    {
        DataSelection::FromMap( k ) => string_from_map( k, source ),
        DataSelection::FromString => value_to_string( source.clone() ),
        DataSelection::None => Ok( "".into() )
    }
}

pub fn merge_data( data_merge : &DataMerge, from : Value, to : Value ) -> Result<Value, Error>
{
    match data_merge
    {
        DataMerge::FromMapToMap { parent_key, keys } =>
        {
            let mut from = value_to_map( from )?;
            let mut to = value_to_map( to )?;

            let mut selection : Map<String, Value> = Map::new();

            for k in keys
            {
                match from.remove( k )
                {
                    Some( v ) => 
                    {
                        selection.insert( k.to_string(), v );
                    },
                    _ => continue
                }
            }

            match parent_key
            {
                Some( k ) => 
                {
                    to.insert( k.to_string(), serde_json::to_value( selection ).map_err( | e | Error::DataSelection( e.to_string() ) )? );
                },
                _ =>
                {
                    selection.into_iter().for_each(
                        | ( k, v ) |
                        {
                            to.insert( k, v );
                        }
                    );
                }
            }
            
            Ok( serde_json::to_value( to ).map_err( | e | Error::DataSelection( e.to_string() ) )? )
        },
        DataMerge::FromStringToString { sep } =>
        {
            Ok(
                Value::String(
                    format!( 
                        "{}{}{}", 
                        value_to_string( to )?, 
                        sep.as_ref().unwrap_or( &"".to_string() ), 
                        value_to_string( from )? 
                    )
                )
            )
        },
        DataMerge::FromStringToMap( s ) =>
        {
            let mut to = value_to_map( to )?;
            let str = value_to_string( from )?;

            to.insert( s.to_string(), Value::String( str ) );

            Ok( serde_json::to_value( to ).map_err( | e | Error::DataSelection( e.to_string() ) )? )
        },
        DataMerge::None => Ok( to )
    }
}

