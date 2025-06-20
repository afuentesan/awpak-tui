use std::fmt::{Display};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error
{
    ParseData( String ),
    MCPTool( String ),
    Agent( String ),
    Command( String ),
    NodeNotFound( String ),
    NodeExists( String ),
    File( String ),
    Ignore
}

fn error_to_string( err : &Error ) -> String
{
    match err
    {
        Error::ParseData( s ) => format!( "ParseData: {}", s ),
        Error::MCPTool( s ) => format!( "MCPTool: {}", s ),
        Error::Agent( s ) => format!( "Agent: {}", s ),
        Error::Command( s ) => format!( "Command: {}", s ),
        Error::NodeNotFound( s ) => format!( "NodeNotFound: {}", s ),
        Error::NodeExists( s ) => format!( "NodeExists: {}", s ),
        Error::File( s ) => format!( "File: {}", s ),
        Error::Ignore => "Ignore".into()
    }
}

impl Display for Error
{
    fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
    {
        write!( f, "{}", error_to_string( self ) )
    }
}

impl std::error::Error for Error {}