use std::fmt::{Display};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error
{
    ParseData( String ),
    MCPTool( String ),
    Agent( String ),
    Command( String ),
    WebClient( String ),
    NodeNotFound( String ),
    NodeExists( String ),
    File( String ),
    Store( String ),
    Ignore
}

impl Error
{
    pub fn append_str( self, str : impl AsRef<str> ) -> Self
    {
        match self
        {
            Error::ParseData( s ) => Error::ParseData( format!( "{}{}", s, str.as_ref() ) ),
            Error::MCPTool( s ) => Error::MCPTool( format!( "{}{}", s, str.as_ref() ) ),
            Error::Agent( s ) => Error::Agent( format!( "{}{}", s, str.as_ref() ) ),
            Error::Command( s ) => Error::Command( format!( "{}{}", s, str.as_ref() ) ),
            Error::WebClient( s ) => Error::WebClient( format!( "{}{}", s, str.as_ref() ) ),
            Error::NodeNotFound( s ) => Error::NodeNotFound( format!( "{}{}", s, str.as_ref() ) ),
            Error::NodeExists( s ) => Error::NodeExists( format!( "{}{}", s, str.as_ref() ) ),
            Error::File( s ) => Error::File( format!( "{}{}", s, str.as_ref() ) ),
            Error::Store( s ) => Error::Store( format!( "{}{}", s, str.as_ref() ) ),
            Error::Ignore => Error::Ignore
        }
    }

    pub fn prepend_str( self, str : impl AsRef<str> ) -> Self
    {
        match self
        {
            Error::ParseData( s ) => Error::ParseData( format!( "{}{}", str.as_ref(), s ) ),
            Error::MCPTool( s ) => Error::MCPTool( format!( "{}{}", str.as_ref(), s ) ),
            Error::Agent( s ) => Error::Agent( format!( "{}{}", str.as_ref(), s ) ),
            Error::Command( s ) => Error::Command( format!( "{}{}", str.as_ref(), s ) ),
            Error::WebClient( s ) => Error::WebClient( format!( "{}{}", str.as_ref(), s ) ),
            Error::NodeNotFound( s ) => Error::NodeNotFound( format!( "{}{}", str.as_ref(), s ) ),
            Error::NodeExists( s ) => Error::NodeExists( format!( "{}{}", str.as_ref(), s ) ),
            Error::File( s ) => Error::File( format!( "{}{}", str.as_ref(), s ) ),
            Error::Store( s ) => Error::Store( format!( "{}{}", str.as_ref(), s ) ),
            Error::Ignore => Error::Ignore
        }
    }
}

pub trait ChangeError
{
    fn append_err( self, str : impl AsRef<str> ) -> Self;
    fn prepend_err( self, str : impl AsRef<str> ) -> Self;
}

impl<T> ChangeError for Result<T, Error>
{
    fn append_err( self, str : impl AsRef<str> ) -> Self 
    {
        match self 
        {
            Ok( t ) => Ok( t ),
            Err( e ) => Err( e.append_str( str ) ),
        }
    }

    fn prepend_err( self, str : impl AsRef<str> ) -> Self 
    {
        match self 
        {
            Ok( t ) => Ok( t ),
            Err( e ) => Err( e.prepend_str( str ) ),
        }
    }
}

fn error_to_string( err : &Error ) -> String
{
    match err
    {
        Error::ParseData( s ) => format!( "ParseData: {}", s ),
        Error::MCPTool( s ) => format!( "MCPTool: {}", s ),
        Error::Agent( s ) => format!( "Agent: {}", s ),
        Error::Command( s ) => format!( "Command: {}", s ),
        Error::WebClient( s ) => format!( "WebClient: {}", s ),
        Error::NodeNotFound( s ) => format!( "NodeNotFound: {}", s ),
        Error::NodeExists( s ) => format!( "NodeExists: {}", s ),
        Error::File( s ) => format!( "File: {}", s ),
        Error::Store( s ) => format!( "Store: {}", s ),
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