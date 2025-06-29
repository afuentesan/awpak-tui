use std::collections::HashMap;

use serde_json::Value;

use crate::domain::{command::{command::{Command, CommandResult}, command_input::command_args, command_output::command_output}, error::Error, utils::string_utils::bytes_to_str};


pub async fn execute_command(
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    command : &Command
) -> Result<String, Error>
{
    if command.command.trim() == "" { return Err( Error::Command( "Empty command".to_string() ) ) }

    let args = command_args( input, parsed_input, context, &command.args )?;

    let result = match tokio::process::Command::new( command.command.trim() )
            .args( args )
            .output()
            .await
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
    }
    .map_err( | e | Error::Command( e.to_string() ) )?;

    command_output( &result, &command.output )
}