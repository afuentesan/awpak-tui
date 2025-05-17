use serde_json::{Map, Value};

use crate::domain::error::Error;

pub fn option_value_to_string( value : Option<&Value> ) -> String
{
    match value
    {
        Some( v ) => match v
        {
            Value::String( s ) => s.clone(),
            Value::Null => "".into(),
            _ => v.to_string()
        },
        _ => "".into()
    }
}

pub fn value_to_string( value : Value ) -> Result<String, Error>
{
    match value
    {
        Value::String( s ) => Ok( s ),
        Value::Null => Ok( "".into() ),
        _ => Ok( value.to_string() )
    }
}

pub fn value_to_map( value : Value ) -> Result<Map<String, Value>, Error>
{
    match value
    {
        Value::Object( m ) => Ok( m ),
        Value::String( s ) => serde_json::from_str::<Map<String, Value>>( &s ).map_err( | e | Error::DataSelection( e.to_string() ) ),
        Value::Null => Ok( Map::new() ),
        _ => Err( Error::DataSelection( "Value to map".into() ) )
    }
}

pub fn array_from_string( str : &str ) -> Vec<Value>
{
    match serde_json::from_str::<Vec<Value>>( str )
    {
        Ok( v ) => v,
        _ => vec![ Value::String( str.to_string() ) ]
    }
}

pub fn string_from_map( key : &str, value : &Value ) -> Result<String, Error>
{
    let mut map = value_to_map( value.clone() )?;

    let value = map.remove( key );

    if let Some( v ) = value
    {
        Ok( value_to_string( v )? )
    }
    else
    {
        Ok( "".into() )    
    }
}

pub fn array_of_strings_from_string( 
    value : &Value,
    title : Option<&String>, 
    footer : Option<&String>
) -> Result<Vec<String>, Error>
{
    match value
    {
        Value::Array( a ) => array_of_strings_from_array( a.clone(), title, footer ),
        Value::String( s ) => array_of_strings_from_array( array_from_string( s ), title, footer ), 
        v => array_of_strings_from_array( vec![ v.clone() ], title, footer )
    }
}

pub fn array_of_strings_from_array(
    array : Vec<Value>,
    title : Option<&String>, 
    footer : Option<&String>
) -> Result<Vec<String>, Error>
{
    Ok( 
        array.into_iter()
        .flat_map( 
            | v |
            {
                value_to_string( v )
            } 
        )
        .map(
            | s |
            {
                add_footer( add_title( s, title ), footer )
            }
        )
        .collect()
    )
}

pub fn array_of_strings_from_map( 
    key : &str, 
    value : &Value,
    title : Option<&String>, 
    footer : Option<&String>
) -> Result<Vec<String>, Error>
{
    let mut map = value_to_map( value.clone() )?;

    let value = map.remove( key );

    if let Some( v ) = value
    {
        match v
        {
            Value::Array( a ) => array_of_strings_from_array( a, title, footer ),
            v => Ok( vec![ add_footer( add_title( value_to_string( v )?, title ), footer ) ] )    
        }
    }
    else
    {
        Ok( vec![] )    
    }
}

pub fn add_title( str : String, title : Option<&String> ) -> String
{
    match title
    {
        Some( t ) => 
        {
            let mut t = t.clone();

            t.push_str( str.as_str() );

            t
        },
        _ => str
    }
}

pub fn add_footer( mut str : String, footer : Option<&String> ) -> String
{
    match footer
    {
        Some( f ) => 
        {
            str.push_str( f );

            str
        },
        _ => str
    }
}

pub fn merge_values( from : Option<Value>, to : Value ) -> Value
{
    match from
    {
        Some( v ) => merge_some_values( v, to ),
        _ => to
    }
}

fn merge_some_values( from : Value, to : Value ) -> Value
{
    if value_is_null( &from ) { return to };
    if value_is_null( &to ) { return from };

    match ( from, to )
    {
        ( 
            Value::Object( m1 ),
            Value::Object( m2 )
        ) => merge_from_map_to_map( m1, m2 ),
        (
            Value::Array( a1 ),
            Value::Array( a2 )
        ) => merge_from_array_to_array( a1, a2 ),
        (
            v,
            Value::Array( a )
        ) => merge_from_value_to_array( v, a ),
        (
            v,
            Value::String( s )
        ) => merge_from_value_to_string( v, s ),
        ( 
            v,
            Value::Number( n )
        ) => merge_from_value_to_string( v, n.to_string() ),
        ( 
            v,
            Value::Bool( b )
        ) => merge_from_value_to_string( v, b.to_string() ),
        ( 
            _,
            v
        ) => v
    }
}

fn merge_from_value_to_string( from : Value, mut to : String ) -> Value
{
    to.push_str( from.to_string().as_str() );

    Value::String( to )
}

fn merge_from_value_to_array( from : Value, mut to : Vec<Value> ) -> Value
{
    to.push( from );

    Value::Array( to )
}

fn merge_from_map_to_map( from : Map<String, Value>, mut to : Map<String, Value> ) -> Value
{
    from.into_iter()
    .for_each(
        | ( k, v ) | { to.insert( k, v ); }
    );

    Value::Object( to )
}

fn merge_from_array_to_array( from : Vec<Value>, mut to : Vec<Value> ) -> Value
{
    from.into_iter()
    .for_each(
        | v | { to.push( v ); }
    );

    Value::Array( to )
}

fn value_is_null( v : &Value ) -> bool
{
    match v
    {
        Value::Null => true,
        _ => false    
    }
}