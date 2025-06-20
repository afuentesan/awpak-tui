use std::collections::HashMap;

use regex::Regex;
use serde_json::{Number, Value};

use crate::domain::{data::{data::DataComparator, data_operations::f64_from_value, data_selection::data_selection, data_utils::value_to_string}, error::Error};


pub fn compare_data(
    input : Option<&String>, 
    parsed_input : &Value,
    context : &HashMap<String, Value>,
    comparator : &DataComparator
) -> Result<bool, Error>
{
    match comparator
    {
        DataComparator::Eq { from_1, from_2 } =>
        {
            let a = data_selection( input, parsed_input, context, from_1 )?;
            let b = data_selection( input, parsed_input, context, from_2 )?;

            let n1 = f64_from_value( a.clone() );
            let n2 = f64_from_value( b.clone() );

            match ( n1, n2 )
            {
                ( Ok( n1 ), Ok( n2 ) ) => Ok( n1 == n2 ),
                _ => Ok( a.to_string() == b.to_string() )
            }
        },
        DataComparator::NotEq { from_1, from_2 } =>
        {
            let a = data_selection( input, parsed_input, context, from_1 )?;
            let b = data_selection( input, parsed_input, context, from_2 )?;

            Ok( a.to_string() != b.to_string() )
        },
        DataComparator::Gt { from_1, from_2 } =>
        {
            let a = number_from_value( data_selection( input, parsed_input, context, from_1 )? )?;
            let b = number_from_value( data_selection( input, parsed_input, context, from_2 )? )?;

            Ok( 
                a.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", a ) ) )? 
                > 
                b.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", b ) ) )? 
            )
        },
        DataComparator::Lt { from_1, from_2 } =>
        {
            let a = number_from_value( data_selection( input, parsed_input, context, from_1 )? )?;
            let b = number_from_value( data_selection( input, parsed_input, context, from_2 )? )?;

            Ok( 
                a.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", a ) ) )? 
                < 
                b.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", b ) ) )? 
            )
        },
        DataComparator::Regex { regex, from} =>
        {
            let str = value_to_string( &data_selection( input, parsed_input, context, from )? );

            match Regex::new( regex )
            {
                Ok( r ) =>
                {
                    Ok( r.is_match( format!( r#"{}"#, str ).as_str() ) )
                },
                Err( e ) => Err( Error::ParseData( format!( "Invalid regex: {}. Error: {:?}", regex, e ) ) )    
            }
        }
        DataComparator::And { comp_1, comp_2 } =>
        {
            Ok(
                compare_data( input, parsed_input, context, comp_1 )?
                &&
                compare_data( input, parsed_input, context, comp_2 )?
            )
        },
        DataComparator::Or { comp_1, comp_2 } =>
        {
            Ok(
                compare_data( input, parsed_input, context, comp_1 )?
                ||
                compare_data( input, parsed_input, context, comp_2 )?
            )
        },
        DataComparator::Not( c ) =>
        {
            Ok(
                ! compare_data( input, parsed_input, context, c )?
            )
        },
        DataComparator::True => Ok( true ),
        DataComparator::False => Ok( false )
    }
}

fn number_from_value( value : Value ) -> Result<Number, Error>
{
    match value
    {
        Value::Number( n ) => Ok( n ),
        _ => Err( Error::ParseData( format!( "{:?} is not a number", value ) ) )    
    }
}