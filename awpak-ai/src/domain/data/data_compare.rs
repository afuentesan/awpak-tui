use regex::Regex;
use serde_json::{Number, Value};

use crate::domain::{data::{data::DataComparator, data_selection::data_selection, data_utils::{is_value_empty, value_to_string, values_are_equals}}, error::Error, graph::graph::Graph};


pub fn compare_data(
    graph : &Graph,
    comparator : &DataComparator
) -> Result<bool, Error>
{
    match comparator
    {
        DataComparator::Eq { from_1, from_2 } =>
        {
            let a = data_selection( graph, from_1 )?;
            let b = data_selection( graph, from_2 )?;

            Ok( values_are_equals( &a, &b ) )
        },
        DataComparator::NotEq { from_1, from_2 } =>
        {
            let a = data_selection( graph, from_1 )?;
            let b = data_selection( graph, from_2 )?;

            Ok( ! values_are_equals( &a, &b ) )
        },
        DataComparator::Gt { from_1, from_2 } =>
        {
            let a = number_from_value( data_selection( graph, from_1 )? )?;
            let b = number_from_value( data_selection( graph, from_2 )? )?;

            Ok( 
                a.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", a ) ) )? 
                > 
                b.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", b ) ) )? 
            )
        },
        DataComparator::Lt { from_1, from_2 } =>
        {
            let a = number_from_value( data_selection( graph, from_1 )? )?;
            let b = number_from_value( data_selection( graph, from_2 )? )?;

            Ok( 
                a.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", a ) ) )? 
                < 
                b.as_f64().ok_or( Error::ParseData( format!( "{} could not be converted to f64", b ) ) )? 
            )
        },
        DataComparator::Regex { regex, from} =>
        {
            let str = value_to_string( &data_selection( graph, from )? );

            match Regex::new( regex )
            {
                Ok( r ) =>
                {
                    Ok( r.is_match( format!( r#"{}"#, str ).as_str() ) )
                },
                Err( e ) => Err( Error::ParseData( format!( "Invalid regex: {}. Error: {:?}", regex, e ) ) )    
            }
        },
        DataComparator::Empty( from ) =>
        {
            let value = data_selection( graph, from )?;

            is_value_empty( &value )
        },
        DataComparator::NotEmpty( from ) =>
        {
            let value = data_selection( graph, from )?;

            Ok( ! is_value_empty( &value )? )
        },
        DataComparator::And { comp_1, comp_2 } =>
        {
            Ok(
                compare_data( graph, comp_1 )?
                &&
                compare_data( graph, comp_2 )?
            )
        },
        DataComparator::Or { comp_1, comp_2 } =>
        {
            Ok(
                compare_data( graph, comp_1 )?
                ||
                compare_data( graph, comp_2 )?
            )
        },
        DataComparator::Xor { comp_1, comp_2 } =>
        {
            let r1 = compare_data( graph, comp_1 )?;
            let r2 = compare_data( graph, comp_2 )?;

            Ok(
                ( r1 && ! r2 ) || ( ! r1 && r2 )
            )
        },
        DataComparator::Nand { comp_1, comp_2 } =>
        {
            let r1 = compare_data( graph, comp_1 )?;
            let r2 = compare_data( graph, comp_2 )?;

            Ok(
                ! r1 && ! r2
            )
        },
        DataComparator::Not( c ) =>
        {
            Ok(
                ! compare_data( graph, c )?
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
        Value::String( s ) if s.trim() != "" => 
        {
            Ok(
                Number::from_f64(
                    s.parse::<f64>().map_err( | _ | Error::ParseData( format!( "{:?} is not a number", s ) ) )?
                ).ok_or( Error::ParseData( format!( "{:?} is not a number", s ) ) )?
            )
        },
        _ => Err( Error::ParseData( format!( "{:?} is not a number", value ) ) )    
    }
}