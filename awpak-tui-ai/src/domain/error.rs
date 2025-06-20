use std::fmt::{Display};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error
{
    DataSelection( String ),
    MCPToolErr( String ),
    AgentErr( String ),
    CommandErr( String ),
    Ignore
}

fn error_to_string( err : &Error ) -> String
{
    match err
    {
        Error::DataSelection( s ) => format!( "DataSelection: {}", s ),
        Error::MCPToolErr( s ) => format!( "MCPToolErr: {}", s ),
        Error::AgentErr( s ) => format!( "AgentErr: {}", s ),
        Error::CommandErr( s ) => format!( "CommandErr: {}", s ),
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