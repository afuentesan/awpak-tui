use std::path::Path;

use crate::domain::{app::model::app::AppContent, error::Error, parser::model::parser_format::ParserFormat, path::path_utils::{extension_from_path, string_from_path}};

use super::{csv_parser::csv_parser, json_parser::json_parser};


pub fn path_to_content( path : &Path ) -> Result<AppContent, Error>
{
    parser_format_to_content( 
        parser_format_from_extension( 
            extension_from_path( 
                path 
            )?,
            string_from_path( path )? 
        )?
    )
}

pub fn str_to_content( str : &str ) -> Result<AppContent, Error>
{
    parser_format_to_content(
        parser_format_from_str( str )?
    )
}

fn parser_format_to_content( parser : ParserFormat ) -> Result<AppContent, Error>
{
    match parser
    {
        ParserFormat::CSV( s ) => csv_parser( s.as_str() ),
        ParserFormat::JSON( s ) => json_parser( s.as_str() )
    }
}

fn parser_format_from_extension( ext : String, content : String ) -> Result<ParserFormat, Error>
{
    let ext = ext.to_lowercase();
    let ext = ext.trim();

    match ext
    {
        "json" => Ok( ParserFormat::JSON( content ) ),
        "csv" => Ok( ParserFormat::CSV( content ) ),
        _ => Err( Error::InvalidParserExtension( ext.to_string() ) )
    }
}

fn parser_format_from_str( str : &str ) -> Result<ParserFormat, Error>
{
    let str = str.trim();

    match ( str.starts_with( "CSV" ) || str.starts_with( "csv" ), str.starts_with( "JSON" ) || str.starts_with( "json" ) )
    {
        ( true, false ) => Ok( ParserFormat::CSV( str[ 3.. ].to_string() ) ),
        ( false, true ) => Ok( ParserFormat::JSON( str[ 4.. ].to_string() ) ),
        _ => return Err( Error::InvalidParserFormat )
    }
}