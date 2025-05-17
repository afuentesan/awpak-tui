use csv::StringRecord;

use crate::domain::{app::model::app::AppContent, error::Error, parser::{functions::json_parser::parser_table_header, model::json_content::{JSONContentCell, JSONContentCellAccess, JSONContentCellType, JSONContentHeader, JSONContentRow, JSONRowType}}, selectable::model::selectable_item::SelectableItem, table::model::{cell::Cell, header::Header, row::Row, table::Table}};

use super::json_parser::{parser_table_cell, parser_table_row_from_cells};

const INDEX_INFO : usize = 3;

pub fn csv_parser( input : &str ) -> Result<AppContent, Error>
{
    let mut csv = csv::ReaderBuilder::new()
        .has_headers( false )
        .delimiter( b';' )
        .from_reader( input.as_bytes() );

    let mut csv_iter = csv.records().into_iter();

    let binding = csv_iter.next();
    let headers = binding
    .iter().flat_map( | c | c )
    .next()
    .ok_or( Error::CSVParser( "Headers error".to_string() ) )?;

    let binding = csv_iter.next();
    let config = binding
    .iter().flat_map( | c | c )
    .next()
    .ok_or( Error::CSVParser( "Config error".to_string() ) )?;

    let ( headers, config ) = headers_config( headers, config )?;

    let rows = csv_iter
    .filter_map( | r | r.ok() )
    .map( | r | table_row( r, &config ) )
    .filter_map( | r | r.ok() )
    .map( | r | SelectableItem::Idle( r ) )
    .collect::<Vec<_>>();

    Ok( AppContent::Table( Table::new( headers, rows )? ) )
}

struct CellConfig
{
    access : JSONContentCellAccess,
    ty : JSONContentCellType
}

fn headers_config( headers : &StringRecord, config : &StringRecord ) -> Result<(Vec<Header>, Vec<CellConfig>), Error>
{
    Ok(
        headers.iter().zip( config.iter() ).skip( INDEX_INFO )
        .flat_map( | ( h, c ) | header_config( h, c ) )
        .unzip()
    )
}

fn header_config( header : &str, config : &str ) -> Result<(Header, CellConfig), Error>
{
    let ( cell_config, visibility ) = config_from_str( config );

    let header = parser_table_header( 
        &JSONContentHeader
        {
            id : None, // TODO: Dar la posibilidad de que se personalice el id
            text : header.to_string(),
            visible : visibility
        }
    );

    Ok( ( header, cell_config ) )
}

fn config_from_str( config : &str ) -> ( CellConfig, bool )
{
    let ( ca, ct, v ) = config.split( ":" )
    .map( | s | item_config_from_str( s ) )
    .fold( ( None, None, None ), 
        | mut a, c |
        {
            if let Some( ac ) = c.0
            {
                a.0 = Some( ac );
            }

            if let Some( ct ) = c.1
            {
                a.1 = Some( ct );
            }

            if let Some( v ) = c.2
            {
                a.2 = Some( v );
            }

            a
        }
    );

    (
        CellConfig
        {
            access : ca.unwrap_or( JSONContentCellAccess::Read ),
            ty : ct.unwrap_or( JSONContentCellType::String )
        },
        v.unwrap_or( true )
    )
}

fn item_config_from_str( config : &str ) -> ( Option<JSONContentCellAccess>, Option<JSONContentCellType>, Option<bool> )
{
    match config.to_lowercase().trim()
    {
        "visible" => ( None, None, Some( true ) ),
        "hidden" => ( None, None, Some( false ) ),
        "read" => ( Some( JSONContentCellAccess::Read ), None, None ),
        "write" => ( Some( JSONContentCellAccess::Write ), None, None ),
        "string" => ( None, Some( JSONContentCellType::String ), None ),
        "date" => ( None, Some( JSONContentCellType::Date ), None ),
        "empty" => ( None, Some( JSONContentCellType::Empty ), None ),
        _ => ( None, None, None )
    }
}

fn table_row( record : StringRecord, config : &Vec<CellConfig> ) -> Result<Row, Error>
{
    let ( row_type, path, params ) = type_path_and_params( &record )?;

    let cells = config.iter().zip( record.iter().skip( INDEX_INFO ) )
    .flat_map( 
        | ( config, str_cell ) | 
        {
            cell_from_str_and_config( str_cell, config )
        }
    )
    .collect::<Vec<_>>();

    parser_table_row_from_cells( 
        cells, 
        &JSONContentRow
        {
            path,
            params,
            row_type,
            cells : vec![]
        }
    )
}

fn cell_from_str_and_config( str_cell : &str, config : &CellConfig ) -> Result<Cell, Error>
{
    parser_table_cell(
        &JSONContentCell
        {
            access : config.access,
            text : str_cell.to_string(),
            ty : config.ty
        }
    )
}

fn type_path_and_params( record : &StringRecord ) -> Result<(JSONRowType, String, Option<String>), Error>
{
    Ok(
        (
            row_type_from_str( record.get( 0 ).ok_or( Error::CSVParser( "Record less than 3 cells".to_string() ) )? )?,
            record.get( 1 ).ok_or( Error::CSVParser( "Record less than 3 cells".to_string() ) )?.to_string(),
            parse_params( record.get( 2 ).ok_or( Error::CSVParser( "Record less than 3 cells".to_string() ) )?.to_string() )
        )
    )
}

fn parse_params( params : String ) -> Option<String>
{
    let params = params.trim();

    if params == ""
    {
        None
    }
    else
    {
        Some( params.to_string() )    
    }
}

fn row_type_from_str( str : &str ) -> Result<JSONRowType, Error>
{
    match str.to_lowercase().trim()
    {
        "directory" => Ok( JSONRowType::Directory ),
        "file" => Ok( JSONRowType::File ),
        "expandable" => Ok( JSONRowType::Expandable ),
        "executableexpandable" => Ok( JSONRowType::ExecutableExpandable ),
        "executable" => Ok( JSONRowType::Executable ),
        "readonly" => Ok( JSONRowType::ReadOnly ),
        _ => Err( Error::CSVParser( "Row type not valid".to_string() ) )
    }
}

#[cfg(test)]
mod tests
{
    use crate::domain::table::model::{cell::CellType, rowable::Rowable};

    use super::*;

    #[test]
    fn test_json_parser()
    {
        let str_csv = r#";;;Header 1 visible;Header 2 hidden;Header 3 visible
;;;;string:read:hidden;write:date
Directory;/tmp;;Cell 1 visible;Cell 2 hidden;16/04/2025
"#;

        let content = csv_parser( str_csv );

        assert!( content.is_ok() );

        let content = content.unwrap();

        match content
        {
            AppContent::Table( t ) =>
            {
                assert_eq!( t.rows().len(), 1 );

                match &t.rows()[ 0 ]
                {
                    SelectableItem::Idle( r ) => match r
                    {
                        Row::Directory( d ) => 
                        {
                            assert_eq!( d.cells().len(), 3 );

                            match &d.cells()[ 0 ].inner()
                            {
                                Cell::Read( c ) => match c
                                {
                                    CellType::String( s ) => assert_eq!( s, "Cell 1 visible" ),
                                    _ => assert!( false, "Cell is not string" )
                                },
                                _ => assert!( false, "Cell is not read" )
                            }

                            match &d.cells()[ 1 ].inner()
                            {
                                Cell::Read( c ) => match c
                                {
                                    CellType::String( s ) => assert_eq!( s, "Cell 2 hidden" ),
                                    _ => assert!( false, "Cell is not string" )
                                },
                                _ => assert!( false, "Cell is not read" )
                            }

                            match &d.cells()[ 2 ].inner()
                            {
                                Cell::Write( c ) => match c
                                {
                                    CellType::Date( d ) => assert_eq!( d.format( "%d/%m/%Y" ).to_string(), "16/04/2025" ),
                                    _ => assert!( false, "Cell is not date" )
                                },
                                _ => assert!( false, "Cell is not write" )
                            }
                        },
                        _ => assert!( false, "Row is not a dir" )
                    },
                    _ => assert!( false, "Row is not iddle" )
                }
            },
            _ => assert!( false, "Content is not a table" ) 
        }
    }
}