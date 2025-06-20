use std::collections::HashMap;

use serde_json::Value;

use crate::domain::{data::data_utils::{value_from_map_path_expanded, value_to_string}, error::Error};


pub fn expand_path(
    context : &HashMap<String, Value>,
    path : &str
) -> Result<String, Error>
{
    let chars = path.chars().collect::<Vec<_>>();

    let mut positions : Vec<usize> = vec![];

    let mut ret = "".to_string();

    let mut i = 0;

    while i < chars.len()
    {
        if chars[ i ] == '{'
        {
            if i < ( chars.len() - 1 ) && chars[ i + 1 ] == '{'
            {
                ret.push( chars[ i ] );

                i += 1;
            }
            else
            {
                positions.push( i );    
            }
        }
        else if chars[ i ] == '}'
        {
            if positions.len() == 0
            {
                ret.push( chars[ i ] );
            }
            else if i < ( chars.len() - 1 ) && chars[ i + 1 ] == '}'
            {
                ret.push( chars[ i ] );

                i += 1;
            }
            else
            {
                let start = positions.pop().unwrap();

                if start < ( i - 1 )
                {
                    let inner : String = chars[ ( start + 1 )..=( i - 1 ) ].iter().collect();

                    let expanded = value_from_map_path_expanded( inner.as_str(), context )?;

                    let expanded = value_to_string( &expanded );

                    ret.push_str( &expanded );
                }
            }
        }
        else if positions.len() == 0
        {
            ret.push( chars[ i ] );
        }

        i += 1;
    }

    if ! ret.starts_with( "/" )
    {
        ret = format!( "/{}", ret );
    }

    Ok( ret )
}