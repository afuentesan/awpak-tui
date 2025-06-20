use std::collections::HashMap;

use serde_json::Value;

use crate::domain::{data::{data::DataFrom, data_selection::data_selection, data_utils::value_to_string}, error::Error};


pub fn command_args( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    args : &Vec<DataFrom> 
) -> Result<Vec<String>, Error>
{
    let mut ret = vec![];

    for arg in args
    {
        ret.push( command_arg( input, parsed_input, context, arg )? );
    }

    Ok( ret )
}

fn command_arg( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    arg : &DataFrom 
) -> Result<String, Error>
{
    Ok( value_to_string( &data_selection( input, parsed_input, context, arg )? ) )
}