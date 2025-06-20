use std::collections::HashMap;

use awpak_utils::result::result::AwpakResult;
use serde_json::{Map, Value};

use crate::domain::{data::{data::{DataMerge, DataToContext}, data_utils::str_to_value}, error::Error, path::expand_path::expand_path};


pub fn str_to_context(
    output : String,
    context : HashMap<String, Value>,
    conf : &DataToContext
) -> AwpakResult<HashMap<String, Value>, Error>
{
    AwpakResult::new( context )
    .validate()
    .zip_result(
        | _ |
        {
            str_to_value( &output, &conf.ty )
        }
    )
    .write()
    .flat_map(
        | ( c, v ) | 
        {
            value_to_context( c, v.unwrap(), conf )
        }
    )
    .finalize()
    .unzip( | ( c, _ ) | c )
    .read()
}

pub fn value_to_context( 
    context : HashMap<String, Value>, 
    value : Value,
    conf : &DataToContext 
) -> AwpakResult<( HashMap<String, Value>, Result<Value, Error> ), Error>
{
    let path = match expand_path( &context, &conf.path )
    {
        Ok( p ) => p,
        Err( e ) => return AwpakResult::new_err( ( context, Ok( Value::Null ) ), e )
    };

    match conf.merge
    {
        DataMerge::Insert => insert_allways( context, value, path.as_str() ),
        DataMerge::Append => append_to_context( context, value, path.as_str() )
    }
}

fn append_to_context(
    mut context : HashMap<String, Value>,
    value : Value,
    path : &str
) -> AwpakResult<( HashMap<String, Value>, Result<Value, Error> ), Error>
{
    let parts = path.split( "/" ).filter( | s | *s != "" ).collect::<Vec<_>>();

    if parts.len() == 0 { return AwpakResult::new_err( ( context, Ok( Value::Null ) ), Error::ParseData( "Invalid key".into() ) ) }

    if parts.len() == 1
    {
        match context.remove( parts[ 0 ] )
        {
            Some( mut v ) => 
            {
                append_value_to_another( &mut v, value );

                context.insert( parts[ 0 ].to_string(), v );

                return AwpakResult::new( ( context, Ok( Value::Null ) ) );
            },
            None => return insert_allways( context, value, path )
        }
    }

    let mut root = match context.remove( parts[ 0 ] )
    {
        Some( v ) => v,
        None => return insert_allways( context, value, path )
    };

    let subpath = format!( "/{}", parts[ 1.. ].join( "/" ) );

    match root.pointer_mut( subpath.as_str() )
    {
        Some( v ) =>
        {
            append_value_to_another( v, value );

            context.insert( parts[ 0 ].into(), root );

            return AwpakResult::new( ( context, Ok( Value::Null ) ) );
        },
        _ =>
        {
            context.insert( parts[ 0 ].into(), root );

            insert_allways( context, value, path )
        }
    }
}

fn append_value_to_another( v1 : &mut Value, v2 : Value )
{
    let old = v1.take();

    let new = merge_values( old, v2 );

    let _ = std::mem::replace( v1, new );
}

pub fn merge_values( v1 : Value, v2 : Value ) -> Value
{
    match ( v1, v2 )
    {
        ( Value::Null, v2 ) => v2,
        ( Value::Array( mut a1 ), Value::Array( mut a2 ) ) =>
        {
            a1.append( &mut a2 );

            Value::Array( a1 )
        },
        ( Value::Object( mut m1 ), Value::Object( m2 ) ) =>
        {
            m2.into_iter().for_each( | ( k, v ) | { m1.insert( k, v ); } );

            Value::Object( m1 )
        },
        ( Value::String( mut s1 ), Value::String( s2 ) ) =>
        {
            s1.push_str( s2.as_str() );

            Value::String( s1 )
        },
        ( Value::Number( n1 ), Value::Number( n2 ) ) =>
        {
            let parsed_n1 = n1.as_f64();
            let parsed_n2 = n2.as_f64();

            if parsed_n1.is_none() { return Value::Number( n2 ) }
            if parsed_n2.is_none() { return Value::Number( n1 ) }

            let next = parsed_n1.unwrap() + parsed_n2.unwrap();

            Value::from( next )
        },
        ( Value::Bool( b1 ), Value::Bool( b2 ) ) => Value::Bool( b1 && b2 ),
        ( _, v2 ) => v2
    }
}

fn insert_allways(
    mut context : HashMap<String, Value>,
    value : Value,
    path : &str
) -> AwpakResult<( HashMap<String, Value>, Result<Value, Error> ), Error>
{
    let parts = path.split( "/" ).filter( | s | *s != "" ).collect::<Vec<_>>();

    if parts.len() == 0 { return AwpakResult::new_err( ( context, Ok( Value::Null ) ), Error::ParseData( "Invalid key".into() ) ) }

    if parts.len() == 1
    {
        context.insert( parts[ 0 ].to_string(), value );

        return AwpakResult::new( ( context, Ok( Value::Null ) ) )
    }

    let parent = match context.get( parts[ 0 ] )
    {
        Some( _ ) => context.remove( parts[ 0 ] ).unwrap(),
        None => Value::Object( Map::new() )
    };

    let parent = insert_allways_recursive( parent, &parts[ 1.. ], value );

    context.insert( parts[ 0 ].to_string(), parent );

    AwpakResult::new( ( context, Ok( Value::Null ) ) )
}

fn insert_allways_recursive( parent : Value, parts : &[&str], value : Value ) -> Value
{
    if parts.len() == 1
    {
        return insert_value_in_parent( parent, parts[ 0 ], value )
    }

    let ( parent, new_parent ) = parent_and_child( parent, parts[ 0 ] );

    let child = insert_allways_recursive( new_parent, &parts[ 1.. ], value );

    insert_value_in_parent( parent, parts[ 0 ], child )
}

fn parent_and_child( parent : Value, key : &str ) -> ( Value, Value )
{
    match key.parse::<usize>()
    {
        Ok( idx ) => parent_and_child_from_numeric_key( parent, idx ),
        _ => parent_and_child_from_str_key( parent, key )
    }
}

fn parent_and_child_from_str_key( parent : Value, key : &str ) -> ( Value, Value )
{
    match parent
    {
        Value::Object( mut m ) => match m.remove( key )
        {
            Some( v ) => ( Value::Object( m ), v ),
            None => ( Value::Object( m ), Value::Null )
        },
        _ => ( Value::Object( Map::new() ), Value::Null )
    }
}

fn parent_and_child_from_numeric_key( parent : Value, key : usize ) -> ( Value, Value )
{
    match &parent
    {
        Value::Object( _ ) => parent_and_child_from_str_key( parent, key.to_string().as_str() ),
        Value::Array( _ ) =>
        {
            
            parent_and_child_from_array( parent, key )
        },
        _ => 
        {
            let a = value_array_of_len_idx( vec![], key );

            ( Value::Array( a ), Value::Null )
        }
    }
}

fn parent_and_child_from_array( parent : Value, key : usize ) -> ( Value, Value )
{
    match parent
    {
        Value::Array( a ) =>
        {
            let mut a = value_array_of_len_idx( a, key );

            let child = std::mem::replace( &mut a[ key ], Value::Null );
                
            ( Value::Array( a ), child )

        },
        _ => unreachable!()    
    }
}

fn insert_value_in_parent( parent : Value, key : &str, value : Value ) -> Value
{
    match key.parse::<usize>()
    {
        Ok( idx ) => insert_value_in_parent_numeric_key( parent, idx, value ),
        _ => insert_value_in_parent_str_key( parent, key, value )
    }
}

fn insert_value_in_parent_numeric_key( parent : Value, key : usize, value : Value ) -> Value
{
    match &parent
    {
        Value::Object( _ ) => insert_value_in_parent_str_key( parent, key.to_string().as_str(), value ),
        Value::Array( a ) if key < a.len() =>
        {
            if let Value::Array( mut a ) = parent
            {
                a[ key ] = value;

                Value::Array( a )
            }
            else
            {
                unreachable!()    
            }
        },
        _ =>
        {
            let mut a = value_array_of_len_idx( vec![], key );

            a[ key ] = value;

            Value::Array( a )
        }
    }
}

fn value_array_of_len_idx( mut a : Vec<Value>, idx : usize ) -> Vec<Value>
{
    if a.len() == 0 { a.push( Value::Null ); }

    while idx > ( a.len() - 1 ) { a.push( Value::Null ); }

    a
}

fn insert_value_in_parent_str_key( parent : Value, key : &str, value : Value ) -> Value
{
    let mut map = match parent
    {
        Value::Object( m ) =>
        {
            m
        },
        _ =>
        {
            Map::new()
        }
    };

    map.insert( key.to_string(), value );

    Value::Object( map )
}

#[cfg(test)]
mod tests
{
    use serde_json::Number;

    use crate::domain::data::data::DataType;

    use super::*;

    #[test]
    fn test_value_to_context_empty_context()
    {
        let value = Value::String( "Ok".into() );

        let context : HashMap<String, Value> = HashMap::new();

        let conf = &DataToContext 
        { 
            path : "/root/inner/3".to_string(), 
            ty : DataType::String, 
            merge : DataMerge::Insert
        };

        let context = value_to_context( context, value, conf );

        assert!( context.is_ok() );

        let mut context = context.own().0;

        let value = context.remove( "root" );

        assert!( value.is_some() );

        let value = value.unwrap();

        let value_arr = value.pointer( "/inner" );

        let value = value.pointer( "/inner/3" );

        assert!( value_arr.is_some() );

        assert!( value.is_some() );

        let value_arr = value_arr.unwrap();

        match value_arr
        {
            Value::Array( a ) => assert_eq!( a.len(), 4 ),
            _ => assert!( false, "Value is not an array" ) 
        };

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )    
        }
    }

    #[test]
    fn test_value_to_context_fill_context()
    {
        let value = Value::String( "Ok".into() );

        let mut context : HashMap<String, Value> = HashMap::new();

        context.insert( "other".into(), Value::String( "Other root".into() ) );

        let mut root_map = Map::new();

        root_map.insert( "other".into(), Value::String( "Other".into() ) );

        let inner_arr = vec![ 
            Value::Number( Number::from_u128( 7 ).unwrap() ),
            Value::String( "ERROR_1".into() ), 
            Value::String( "ERROR_2".into() ),
            Value::String( "OK_REPLACED".into() ),
            Value::String( "ERROR_LAST".into() )
        ];

        root_map.insert( "inner".into(), Value::Array( inner_arr ) );

        context.insert( "root".into(), Value::Object( root_map ) );

        let conf = &DataToContext 
        { 
            path : "/root/inner/3".to_string(), 
            ty : DataType::String, 
            merge : DataMerge::Insert
        };

        let context = value_to_context( context, value, conf );

        assert!( context.is_ok() );

        let mut context = context.own().0;

        let other_root = context.get( "other" );

        assert!( other_root.is_some() );

        let other_root = other_root.unwrap();

        match other_root
        {
            Value::String( s ) => assert_eq!( s, "Other root" ),
            _ => assert!( false, "Other root is not an String" )    
        }

        let value = context.remove( "root" );

        assert!( value.is_some() );

        let value = value.unwrap();

        let value_other = value.pointer( "/other" );

        assert!( value_other.is_some() );

        let value_other = value_other.unwrap();

        match value_other
        {
            Value::String( s ) => assert_eq!( s, "Other" ),
            _ => assert!( false, "value_other is not a String" )
        }

        let value_arr = value.pointer( "/inner" );

        let value = value.pointer( "/inner/3" );

        assert!( value_arr.is_some() );

        assert!( value.is_some() );

        let value_arr = value_arr.unwrap();

        match value_arr
        {
            Value::Array( a ) => 
            {
                assert_eq!( a.len(), 5 );

                assert!( a[ 0 ].is_number() );
                assert!( a[ 1 ].is_string() );
                assert!( a[ 2 ].is_string() );
                assert!( a[ 3 ].is_string() );
                assert!( a[ 4 ].is_string() );

                assert_eq!( a[ 3 ].as_str().unwrap(), "Ok" );
                assert_eq!( a[ 1 ].as_str().unwrap(), "ERROR_1" );
                assert_eq!( a[ 4 ].as_str().unwrap(), "ERROR_LAST" );
            },
            _ => assert!( false, "Value is not an array" ) 
        };

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )    
        }
    }

    #[test]
    fn test_value_to_context_empty_context_append()
    {
        let value = Value::String( "Ok".into() );

        let context : HashMap<String, Value> = HashMap::new();

        let conf = &DataToContext 
        { 
            path : "/root/inner/3".to_string(), 
            ty : DataType::String, 
            merge : DataMerge::Append
        };

        let context = value_to_context( context, value, conf );

        assert!( context.is_ok() );

        let mut context = context.own().0;

        let value = context.remove( "root" );

        assert!( value.is_some() );

        let value = value.unwrap();

        let value_arr = value.pointer( "/inner" );

        let value = value.pointer( "/inner/3" );

        assert!( value_arr.is_some() );

        assert!( value.is_some() );

        let value_arr = value_arr.unwrap();

        match value_arr
        {
            Value::Array( a ) => assert_eq!( a.len(), 4 ),
            _ => assert!( false, "Value is not an array" ) 
        };

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "Ok" ),
            _ => assert!( false, "Value is not a String" )    
        }
    }

    #[test]
    fn test_value_to_context_fill_context_append()
    {
        let value = Value::String( "Ok".into() );

        let mut context : HashMap<String, Value> = HashMap::new();

        context.insert( "other".into(), Value::String( "Other root".into() ) );

        let mut root_map = Map::new();

        root_map.insert( "other".into(), Value::String( "Other".into() ) );

        let inner_arr = vec![ 
            Value::Number( Number::from_u128( 7 ).unwrap() ),
            Value::String( "ERROR_1".into() ), 
            Value::String( "ERROR_2".into() ),
            Value::String( "OK_REPLACED".into() ),
            Value::String( "ERROR_LAST".into() )
        ];

        root_map.insert( "inner".into(), Value::Array( inner_arr ) );

        context.insert( "root".into(), Value::Object( root_map ) );

        let conf = &DataToContext 
        { 
            path : "/root/inner/3".to_string(), 
            ty : DataType::String, 
            merge : DataMerge::Append
        };

        let context = value_to_context( context, value, conf );

        assert!( context.is_ok() );

        let mut context = context.own().0;

        let other_root = context.get( "other" );

        assert!( other_root.is_some() );

        let other_root = other_root.unwrap();

        match other_root
        {
            Value::String( s ) => assert_eq!( s, "Other root" ),
            _ => assert!( false, "Other root is not an String" )    
        }

        let value = context.remove( "root" );

        assert!( value.is_some() );

        let value = value.unwrap();

        let value_other = value.pointer( "/other" );

        assert!( value_other.is_some() );

        let value_other = value_other.unwrap();

        match value_other
        {
            Value::String( s ) => assert_eq!( s, "Other" ),
            _ => assert!( false, "value_other is not a String" )
        }

        let value_arr = value.pointer( "/inner" );

        let value = value.pointer( "/inner/3" );

        assert!( value_arr.is_some() );

        assert!( value.is_some() );

        let value_arr = value_arr.unwrap();

        match value_arr
        {
            Value::Array( a ) => 
            {
                assert_eq!( a.len(), 5 );

                assert!( a[ 0 ].is_number() );
                assert!( a[ 1 ].is_string() );
                assert!( a[ 2 ].is_string() );
                assert!( a[ 3 ].is_string() );
                assert!( a[ 4 ].is_string() );

                assert_eq!( a[ 3 ].as_str().unwrap(), "OK_REPLACEDOk" );
                assert_eq!( a[ 1 ].as_str().unwrap(), "ERROR_1" );
                assert_eq!( a[ 4 ].as_str().unwrap(), "ERROR_LAST" );
            },
            _ => assert!( false, "Value is not an array" ) 
        };

        let value = value.unwrap();

        match value
        {
            Value::String( s ) => assert_eq!( s, "OK_REPLACEDOk" ),
            _ => assert!( false, "Value is not a String" )    
        }
    }
}