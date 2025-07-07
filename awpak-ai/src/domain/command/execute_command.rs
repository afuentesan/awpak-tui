use std::{collections::HashMap, process::Output, time::Duration};

use serde_json::Value;
use tokio::time::sleep;
use tracing::info;

use crate::domain::{command::{command::{Command, CommandResult}, command_input::command_args, command_output::command_output}, error::Error, signals::cancel_graph::is_graph_cancelled, tracing::filter_layer::{COMMAND_AND_ARGS, COMMAND_RESULT}, utils::string_utils::{bytes_to_str, option_string_to_str}};


pub async fn execute_command(
    id : Option<&String>,
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    command : &Command
) -> Result<String, Error>
{
    if command.command.trim() == "" { return Err( Error::Command( "Empty command".to_string() ) ) }

    let args = command_args( input, parsed_input, context, &command.args )?;

    trace_command_and_args( id, &command.command, &args );

    let result = match command_result( id, command.command.trim(), args ).await
    {
        Ok( o ) =>
        {
            let result = CommandResult
            {
                success : o.status.success(),
                code : o.status.code(),
                
                out : bytes_to_str( &o.stdout ).ok(),
                err : bytes_to_str( &o.stderr ).ok()
            };

            info!( target:COMMAND_RESULT, id=option_string_to_str( id ), text=result.to_string() );

            Ok( result )
        },
        Err( e ) =>
        {
            Err( Error::Command( e.to_string() ) )
        }
    }?;

    command_output( &result, &command.output )
}

fn trace_command_and_args( graph_id : Option<&String>, command : &str, args : &Vec<String> )
{
    info!(
        target:COMMAND_AND_ARGS, 
        id=option_string_to_str( graph_id ), 
        text=format!( 
            "{} {}", 
            command,
            args.iter().fold(
                "".to_string(), 
                | ac, ar | format!( "{}{} ", ac, ar )
            )
        )
    )
}

async fn command_result( 
    id : Option<&String>,
    command : &str, 
    args : Vec<String> 
) -> Result<Output, Error>
{
    match id
    {
        Some( id ) =>
        {
            tokio::select! 
            {
                v = tokio::process::Command::new( command )
                    .args( args )
                    .output() =>
                    // .await.map_err( | e | Error::Command( e.to_string() ) )
                    {
                        v.map_err( | e | Error::Command( e.to_string() ) )
                    },
                _ = async {
                    loop
                    {
                        if is_graph_cancelled( id ) { break; }

                        sleep( Duration::from_millis( 1000 ) ).await
                    }
                } => return Err( Error::Command( "Command cancelled".into() ) )
            }
        },
        _ => tokio::process::Command::new( command )
            .args( args )
            .output()
            .await.map_err( | e | Error::Command( e.to_string() ) )
    }
}