use std::fmt::Display;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error
{
    InvalidPath( String ),
    ReadDir( String ),
    NotDir( String ),
    NotFile( String ),
    NotExec( String ),
    DirWithoutCells( String ),
    ExpandableWithoutCells( String ),
    InvalidTableData( String ),
    FailReadFile( String ),
    Ignore,
    NotHomeDir,
    NoSelectableItems,
    NoSelectedItems,
    EmptyContentGenerator,
    NoParent,
    InvalidParserFormat,
    InvalidJSON( String ),
    InvalidStrDate( String ),
    PathWithoutExtension( String ),
    InvalidParserExtension( String ),
    CSVParser( String ),
    ErrorExecuteCommand( String ),
    DestinationPathExists( String ),
    CopyFile( String ),
    CopyDirectory( String ),
    DeleteFile( String ),
    DeleteDirectory( String ),
    RenameDirectory( String ),
    Graph( String )
}

fn error_to_string( err : &Error ) -> String
{
    match err
    {
        Error::InvalidPath( s ) |    
        Error::ReadDir( s ) |
        Error::NotDir( s ) |
        Error::NotFile( s ) |
        Error::NotExec( s ) |
        Error::DirWithoutCells( s ) |
        Error::InvalidTableData( s ) |
        Error::ExpandableWithoutCells( s ) |
        Error::FailReadFile( s ) |
        Error::InvalidJSON( s ) |
        Error::InvalidStrDate( s ) |
        Error::PathWithoutExtension( s ) |
        Error::InvalidParserExtension( s ) |
        Error::ErrorExecuteCommand( s ) |
        Error::CSVParser( s ) |
        Error::DestinationPathExists( s ) |
        Error::CopyFile( s ) |
        Error::CopyDirectory( s ) |
        Error::DeleteFile( s ) |
        Error::DeleteDirectory( s ) |
        Error::RenameDirectory( s ) |
        Error::Graph( s ) => s.clone(),
        Error::NotHomeDir => "Not home dir".to_string(),
        Error::NoSelectableItems => "No visible items".to_string(),
        Error::NoSelectedItems => "No selected items".to_string(),
        Error::EmptyContentGenerator => "Empty content generator".to_string(),
        Error::NoParent => "No parent".to_string(),
        Error::InvalidParserFormat => "Invalid parser format".to_string(),
        Error::Ignore => "Ignore".to_string()
    }
}

impl Display for Error
{
    fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
    {
        write!( f, "{}", error_to_string( self ) )
    }
}

impl std::error::Error for Error
{
    
}