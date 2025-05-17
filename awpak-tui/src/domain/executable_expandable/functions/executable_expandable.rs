use std::process::{Command, Output};

use crate::domain::{app::model::app::AppContent, error::Error, executable_expandable::model::executable_expandable::ExecutableExpandable, parser::functions::parser::str_to_content, util::executable_utils::parse_params};


pub fn expand_excutable_expandable( executable : &ExecutableExpandable ) -> Result<AppContent, Error>
{
    let output = execute_expandable_command( 
        executable.to_string().as_str(), 
        executable.params() 
    )?;

    str_to_content( &output )
}

fn execute_expandable_command( command : &str, params : Option<&String> ) -> Result<String, Error>
{
    match Command::new( command )
    .args( parse_params( params ) )
    .output()
    {
        Ok( o ) => proccess_output( o ),
        Err( e ) => Err( Error::ErrorExecuteCommand( e.to_string() ) )
    }
}

fn proccess_output( output : Output ) -> Result<String, Error>
{
    match core::str::from_utf8( &output.stdout )
    {
        Ok( s ) => Ok( s.to_string() ),
        Err( e ) => Err( Error::ErrorExecuteCommand( e.to_string() ) )
    }
}