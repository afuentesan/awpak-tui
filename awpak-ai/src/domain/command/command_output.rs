use crate::domain::{command::command::{CommandOutput, CommandResult}, error::Error, utils::string_utils::{prefix_str_suffix, str_from_option}};


pub fn command_output(
    result : &CommandResult,
    output : &Vec<CommandOutput>
) -> Result<String, Error>
{
    let mut ret = String::new();

    for out in output
    {
        ret.push_str( item_command_output( result, out )?.as_str() );
    }

    Ok( ret )
}

fn item_command_output(
    result : &CommandResult,
    output : &CommandOutput
) -> Result<String, Error>
{
    match output
    {
        CommandOutput::Code { prefix, suffix } => 
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), str_from_option( result.code ).as_str() ) )
        },
        CommandOutput::Success { prefix, suffix } =>
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), str_from_option( Some( result.success ) ).as_str() ) )
        },
        CommandOutput::Out { prefix, suffix } =>
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), str_from_option( result.out.clone() ).as_str() ) )
        },
        CommandOutput::Err { prefix, suffix } =>
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), str_from_option( result.err.clone() ).as_str() ) )
        },
        CommandOutput::Object { prefix, suffix } =>
        {
            Ok( 
                prefix_str_suffix( 
                    prefix.as_ref(), 
                    suffix.as_ref(), 
                    &serde_json::to_string( result ).map_err( | e | Error::ParseData( e.to_string() ) )?
                ) 
            )
        }
    }
}