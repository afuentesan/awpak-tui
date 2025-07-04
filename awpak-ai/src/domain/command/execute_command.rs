use std::{collections::HashMap, process::Output, time::Duration};

use serde_json::Value;
use tokio::time::sleep;

use crate::domain::{command::{command::{Command, CommandResult}, command_input::command_args, command_output::command_output}, error::Error, signals::cancel_graph::is_graph_cancelled, utils::string_utils::bytes_to_str};


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

    let result = match command_result( id, command.command.trim(), args ).await
    {
        Ok( o ) =>
        {
            Ok(
                CommandResult
                {
                    success : o.status.success(),
                    code : o.status.code(),
                    
                    out : bytes_to_str( &o.stdout ).ok(),
                    err : bytes_to_str( &o.stderr ).ok()
                }
            )
        },
        Err( e ) =>
        {
            Err( Error::Command( e.to_string() ) )
        }
    }?;

    command_output( &result, &command.output )
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