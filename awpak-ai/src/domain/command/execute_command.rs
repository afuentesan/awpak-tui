use std::{os::unix::process::ExitStatusExt, process::{ExitStatus, Output, Stdio}, time::Duration};

use tokio::{process::Child, select, time::sleep};
use tracing::info;

use crate::domain::{command::{command::{Command, CommandResult}, command_input::command_args, command_output::command_output}, data::{data_selection::data_selection, data_utils::value_to_string}, error::{ChangeError, Error}, graph::graph::Graph, signals::cancel_graph::is_graph_cancelled, tracing::filter_layer::{COMMAND_AND_ARGS, COMMAND_RESULT}, utils::string_utils::{bytes_to_str, option_string_to_str}};

pub async fn execute_command(
    graph : &Graph,
    command : &Command
) -> Result<String, Error>
{
    let id = graph.id.as_ref();

    let command_str = value_to_string( 
        &data_selection( graph, &command.command ).prepend_err( "Command.\n" )? 
    );
    
    if command_str.trim() == "" { return Err( Error::Command( "Empty command".to_string() ) ) }

    let args = command_args( graph, &command.args ).prepend_err( "Command args.\n" )?;

    trace_command_and_args( id, &command_str, &args );

    let result = match alternate_command_exec( id, command_str.trim(), args, command.timeout ).await
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
            Err( Error::Command( format!( "Command execution. {:?}", e ) ) )
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

async fn alternate_command_exec( 
    id : Option<&String>,
    command : &str, 
    args : Vec<String>,
    timeout : Option<u64>
) -> Result<Output, Error>
{
    match tokio::process::Command::new( command )
    .stdout( Stdio::piped() )
    .stderr( Stdio::piped() )
    .args( args ).spawn()
    {
        Ok( c ) => command_child_exec( id, c, timeout ).await,
        Err( e ) => Err( Error::Command( e.to_string() ) )
    }
}

async fn command_child_exec(
    id : Option<&String>,
    mut child : Child,
    timeout : Option<u64>
) -> Result<Output, Error>
{
    select! 
    {
        c = child.wait() =>
        {
            match c
            {
                Ok( _ ) =>
                {
                    child.wait_with_output().await.map_err( | e | Error::Command( e.to_string() ) )
                },
                Err( e ) => Err( Error::Command( e.to_string() ) )
            }
        },
        _ = async {
            match timeout
            {
                Some( t ) => sleep( Duration::from_secs( t ) ).await,
                None =>
                {
                    loop { sleep( Duration::from_secs( 1000 ) ).await }
                }
            }
        } =>
        {
            let _ = child.kill().await;

            Ok(
                Output
                {
                    status : ExitStatus::from_raw( 1 ),
                    stdout : vec![],
                    stderr : format!( "Command timeout. {} secs.", timeout.unwrap_or( 0 ) ).as_bytes().to_vec()
                }
            )
        }
        _ = async {
            loop
            {
                match id
                {
                    Some( id ) =>
                    {
                        if is_graph_cancelled( id ) { break; }

                        sleep( Duration::from_millis( 1000 ) ).await
                    },
                    _ =>
                    {
                        sleep( Duration::from_secs( 1000 ) ).await
                    }
                }
                
            }
        } =>
        {
            let _ = child.kill().await;

            Err( Error::Command( "Command cancelled".into() ) )
        }
    }
}