
use serde_json::{Number, Value};

use crate::domain::{data::data_utils::value_to_string, error::Error};

pub fn string_split( v : Value, separator : impl AsRef<str> ) -> Value
{
    let str = value_to_string( &v );

    Value::Array(
        str.split( separator.as_ref() ).map( | s | Value::String( s.to_string() ) ).collect()
    )
}

pub fn substract_values( v1 : Value, v2 : Value ) -> Result<Value, Error>
{
    let n1 = f64_from_value( v1 )?;
    let n2 = f64_from_value( v2 )?;

    Ok( value_from_f64( n1 - n2 )? )
}

pub fn add_values( v1 : Value, v2 : Value ) -> Result<Value, Error>
{
    let n1 = f64_from_value( v1 )?;
    let n2 = f64_from_value( v2 )?;

    Ok( value_from_f64( n1 + n2 )? )
}

fn value_from_f64( number : f64 ) -> Result<Value, Error>
{
    if number == number.round()
    {
        Ok( Value::Number( Number::from_i128( number as i128 ).ok_or( Error::ParseData( format!( "" ) ) )? ) )
    }
    else
    {
        Ok( Value::from( number ) )
    }
}

pub fn f64_from_value( value : Value ) -> Result<f64, Error>
{
    match value
    {
        Value::Number( n ) => n.as_f64().ok_or( Error::ParseData( format!( "{:?} invalid f64", n ) ) ),
        Value::String( s ) if s.trim() != "" => s.parse().map_err( | _ | Error::ParseData( format!( "{:?} invalid f64", s ) ) ),
        v => Err( Error::ParseData( format!( "Value is not a number. {:?}", v ) ) )
    }
}