use std::process::{Command, Stdio};

use crate::domain::{error::Error, util::executable_utils::parse_params};


pub fn execute_command( command : &str, params : Option<&String> ) -> Result<(), Error>
{
    match Command::new( command )
    .args( parse_params( params ) )
    .stderr(Stdio::null() )
    .stdout(Stdio::null() )
    .spawn()
    {
        Ok( _ ) => Ok( () ),
        Err( e ) => Err( Error::ErrorExecuteCommand( e.to_string() ) )
    }
}