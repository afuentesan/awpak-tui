use std::collections::HashMap;

use serde_json::Value;
use tokio::process::Command;

use crate::domain::{chat::chat::ChatChannel, command::command::CommandResult, error::Error, node::{node::NodeOutputDestination, node_functions::{response_header, trace_node_prompt}}};


pub async fn send_prompt_to_command<U: ChatChannel + Send + Sync>(
    prompt : &str,
    command : &str,
    args : &Vec<String>,
    output_ok : &NodeOutputDestination,
    output_err : &NodeOutputDestination,
    chat_channel : &U
) -> Result<CommandResult, Error>
{
    let mut args = args.clone();

    args.append( &mut prompt_to_args( prompt ) );

    response_header( chat_channel, output_ok );

    match Command::new( command )
            .args( args )
            .output().await
    {
        Ok( o ) =>
        {
            send_output( &o, output_ok, output_err, chat_channel );

            Ok(
                CommandResult
                {
                    success : o.status.success(),
                    code : o.status.code(),
                    
                    out : std_str( &o.stdout ),
                    err : std_str( &o.stderr )
                }
            )
        },
        Err( e ) =>
        {
            trace_node_prompt( output_err, e.to_string().as_str(), chat_channel );

            Err( Error::CommandErr( e.to_string() ) )
        }
    }
}

fn std_str( output : &Vec<u8> ) -> Option<String>
{
    match core::str::from_utf8( output )
    {
        Ok( s ) => Some( s.trim().to_string() ),
        _ => None
    }
}

fn send_output<U: ChatChannel + Send + Sync>( 
    output : &std::process::Output, 
    output_ok : &NodeOutputDestination,
    output_err : &NodeOutputDestination,
    chat_channel : &U 
)
{
    send_output_to( &output.stdout, output_ok, output_err, chat_channel );
    send_output_to( &output.stderr, output_err, output_err, chat_channel );
}

fn send_output_to<U: ChatChannel + Send + Sync>( 
    output : &Vec<u8>, 
    output_dest : &NodeOutputDestination,
    output_err : &NodeOutputDestination,
    chat_channel : &U 
)
{
    if output.len() == 0 { return }

    match core::str::from_utf8( output )
    {
        Ok( s ) if s.trim() != "" => trace_node_prompt( output_dest, s, chat_channel ),
        Ok( _ ) => return,
        Err( e ) => trace_node_prompt( output_err, e.to_string().as_str(), chat_channel ),
    }
}

fn prompt_to_args( prompt : &str ) -> Vec<String>
{
    if prompt.trim() == "" { return vec![] }

    match serde_json::from_str::<Vec<String>>( prompt )
    {
        Ok( l ) => l,
        _ => prompt_to_args_map( prompt )
    }
}

fn prompt_to_args_map( prompt : &str ) -> Vec<String>
{
    match serde_json::from_str::<HashMap<String, Value>>( prompt )
    {
        Ok( m ) => args_from_map( m ),
        _ => vec![ prompt.to_string() ]
    }
}

fn args_from_map( map : HashMap<String, Value> ) -> Vec<String>
{
    map.iter()
    .flat_map(
        | ( k, v ) |
        {
            if k.trim() == ""
            {
                let v = v.to_string();

                if v.trim() == "" { vec![] } else { vec![ format!( "{}", v.trim() ) ] }
            }
            else
            {
                let v = v.to_string();

                if k.trim().len() == 1
                {
                    let mut ret = vec![ format!( "-{}", k.trim() ) ];

                    if v.trim() != "" { ret.push( format!( "{}", v.trim() ) ); }

                    ret
                }
                else
                {
                    if v.trim() == ""
                    {
                        vec![ format!( "--{}", k.trim() ) ]
                    }
                    else
                    {
                        vec![ format!( "--{}={}", k.trim(), v.trim() ) ]
                    }
                }
            }
        }
    )
    .collect()
}