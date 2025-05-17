use crate::domain::{app::model::app::AppContent, directory::model::directory::Directory, error::Error, executable::model::executable::Executable, executable_expandable::model::executable_expandable::ExecutableExpandable, expandable::model::expandable::Expandable, file::model::file::File, parser::model::json_content::{JSONContent, JSONContentCell, JSONContentCellAccess, JSONContentCellType, JSONContentHeader, JSONContentRow, JSONContentTable, JSONRowType}, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, header::{Header, HeaderData}, row::Row, table::Table}, util::date_utils::naive_date_from_str};


pub fn json_parser( input : &str ) -> Result<AppContent, Error>
{
    to_content( serde_json::from_str( input ).map_err( | e | Error::InvalidJSON( e.to_string() ) )? )
}

fn to_content( json : JSONContent ) -> Result<AppContent, Error>
{
    if let Some( t ) = json.table
    {
        json_table_to_content( t )
    }
    else
    {
        Err( Error::Ignore )    
    }
}

fn json_table_to_content( json_table : JSONContentTable ) -> Result<AppContent, Error>
{
    let headers = table_headers( &json_table.headers )?;
    let rows = table_rows( &json_table.rows )?;

    let table = Table::new( headers, rows )?;

    Ok( AppContent::Table( table ) )
}

fn table_headers( json_headers : &Vec<JSONContentHeader> ) -> Result<Vec<Header>, Error>
{
    Ok(
        json_headers.iter()
        .map( | j | parser_table_header( j ) )
        .collect()
    )
}

pub fn parser_table_header( j : &JSONContentHeader ) -> Header
{
    let header_data = HeaderData::new(
        j.id.as_ref().unwrap_or( &j.text ), 
        &j.text
    );

    if j.visible
    {
        Header::Visible( header_data )
    }
    else
    {
        Header::Hidden( header_data )    
    }
}

fn table_rows( json_rows : &Vec<JSONContentRow> ) -> Result<Vec<SelectableItem<Row>>, Error>
{
    Ok(
        json_rows.iter()
        .flat_map(
            | r | table_row( r )
        )
        .map( | r | SelectableItem::Idle( r ) )
        .collect()
    )
}

fn table_row( json_row : &JSONContentRow ) -> Result<Row, Error>
{
    let cells = json_row.cells.iter()
    .flat_map( | c | parser_table_cell( c ) )
    .collect::<Vec<_>>();

    parser_table_row_from_cells( cells, json_row )
}

pub fn parser_table_row_from_cells( cells : Vec<Cell>, json_row : &JSONContentRow ) -> Result<Row, Error>
{
    match json_row.row_type
    {
        JSONRowType::Directory => table_row_dir( cells, json_row ),
        JSONRowType::File => table_row_file( cells, json_row ),
        JSONRowType::Expandable => table_row_expandable( cells, json_row ),
        JSONRowType::ExecutableExpandable => table_row_executable_expandable( cells, json_row ),
        JSONRowType::Executable => table_row_executable( cells, json_row ),
        JSONRowType::ReadOnly => Ok( Row::Data( cells.into_iter().map( | c | SelectableItem::Idle( c ) ).collect() ) )
    }
}

pub fn parser_table_cell( json_cell : &JSONContentCell ) -> Result<Cell, Error>
{
    let cell_type = table_cell_type( json_cell )?;

    match json_cell.access
    {
        JSONContentCellAccess::Read => Ok( Cell::Read( cell_type ) ),
        JSONContentCellAccess::Write => Ok( Cell::Write( cell_type ) )
    }
}

fn table_cell_type( json_cell : &JSONContentCell ) -> Result<CellType, Error>
{
    match json_cell.ty
    {
        JSONContentCellType::String => Ok( CellType::String( json_cell.text.clone() ) ),
        JSONContentCellType::Date => Ok( CellType::Date( naive_date_from_str( &json_cell.text )? ) ),
        JSONContentCellType::Empty => Ok( CellType::Empty )
    }
}

fn table_row_dir( cells : Vec<Cell>, json_row : &JSONContentRow ) -> Result<Row, Error>
{
    let dir = Directory::new( &json_row.path, cells )?;

    Ok( Row::Directory( dir ) )
}

fn table_row_file( cells : Vec<Cell>, json_row : &JSONContentRow ) -> Result<Row, Error>
{
    let file = File::new( &json_row.path, cells )?;

    Ok( Row::File( file ) )
}

fn table_row_expandable( cells : Vec<Cell>, json_row : &JSONContentRow ) -> Result<Row, Error>
{
    let expandable = Expandable::new( &json_row.path, cells )?;

    Ok( Row::Expandable( expandable ) )
}

fn table_row_executable_expandable( cells : Vec<Cell>, json_row : &JSONContentRow ) -> Result<Row, Error>
{
    let executable_expandable = ExecutableExpandable::new( &json_row.path, json_row.params.clone(), cells )?;

    Ok( Row::ExecutableExpandable( executable_expandable ) )
}

fn table_row_executable( cells : Vec<Cell>, json_row : &JSONContentRow ) -> Result<Row, Error>
{
    let executable = Executable::new( &json_row.path, json_row.params.clone(), cells )?;

    Ok( Row::Executable( executable ) )
}

#[cfg(test)]
mod tests
{
    use crate::domain::table::model::rowable::Rowable;

    use super::*;

    #[test]
    fn test_json_parser()
    {
        let str_json = r#"
{
    "table" : {
        "headers" : [
            {
                "text" : "Header 1 visible",
                "visible" : true
            },
            {
                "text" : "Header 2 hidden",
                "visible" : false
            },
            {
                "text" : "Header 3 visible",
                "visible" : true
            }
        ],
        "rows" : [
            {
                "path" : "/tmp",
                "row_type" : "Directory",
                "cells" : [
                    {
                        "text" : "Cell 1 visible",
                        "access" : "Read",
                        "ty" : "String"
                    },
                    {
                        "text" : "Cell 2 hidden",
                        "access" : "Read",
                        "ty" : "String"
                    },
                    {
                        "text" : "16/04/2025",
                        "access" : "Write",
                        "ty" : "Date"
                    }
                ]
            }
        ]
    }
}
        "#;

        let content = json_parser( str_json );

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