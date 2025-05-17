use std::fs::DirEntry;

use crate::domain::{app::model::app::AppContent, directory::model::directory::Directory, error::Error, file::model::file::File, path::path_utils::{get_row_from_path, path_parent_of_dir}, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, header::{Header, HeaderData}, row::Row, table::Table}};

pub fn parent_directory( dir : &Directory ) -> Result<Directory, Error>
{
    let path = path_parent_of_dir( dir.path() )?;

    Directory::new_from_path( path, vec![ Cell::Read( CellType::String( format!( "Parent of {:?}", dir.path() ) ) ) ] )
}

pub fn dir_expand( directory : &Directory ) -> Result<AppContent, Error>
{
    let paths = std::fs::read_dir( directory.path() ).map_err( | e | Error::ReadDir( e.to_string() ) )?;

    let rows = paths.into_iter()
        .flat_map( | r | r.ok() )
        .flat_map( 
            | e | expanded_node_from_dir( e ).map( | e | SelectableItem::Idle( e ) )
        )
        .collect::<Vec<_>>();

    let headers = dir_headers();

    Ok( AppContent::Table( Table::new( headers, rows )? ) )
}

fn expanded_node_from_dir( entry : DirEntry ) -> Option<Row>
{
    let path = entry.path();

    if path.is_dir()
    {
        Some( 
            Row::Directory( Directory::new( path.to_str().unwrap(), get_row_from_path( &path ).ok()? ).ok()? ) 
        )
    }
    else if path.is_file()
    {
        Some( 
            Row::File( File::new( path.to_str().unwrap(), get_row_from_path( &path ).ok()? ).ok()? ) 
        )
    }
    else
    {
        None    
    }
}

fn dir_headers() -> Vec<Header>
{
    vec![ 
        Header::Visible( HeaderData::new( "name", "Name" ) ),
        Header::Visible( HeaderData::new( "modified", "Modified" ) ),
        Header::Visible( HeaderData::new( "size", "Size" ) )
    ]
}

#[cfg(test)]
mod tests
{
    use std::fs::create_dir;

    use crate::domain::table::model::{cell::{Cell, CellType}, rowable::Rowable};

    use super::*;

    #[test]
    fn test_dir_expand()
    {
        let dir = Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( dir.is_ok() );

        let dir = dir.unwrap();

        let expanded = dir_expand( &dir );

        assert!( expanded.is_ok() );

        let expanded = expanded.unwrap();

        match expanded
        {
            AppContent::Table( t ) => {
                assert!( t.headers().len() == 3 );
                assert!( t.rows().len() > 0 );

                for row in t.rows()
                {
                    assert!( row.inner().cells().len() == 3 );
                }
            },
            _ => assert!( false, "expanded is not table" )    
        }
    }

    #[test]
    fn test_empty_dir_expand()
    {
        let _ = create_dir( "/tmp/prueba_empty" );

        let dir = Directory::new( "/tmp/prueba_empty", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( dir.is_ok() );

        let dir = dir.unwrap();

        let expanded = dir_expand( &dir );

        assert!( expanded.is_ok() );

        let expanded = expanded.unwrap();

        match expanded
        {
            AppContent::Table( t ) => {
                assert!( t.headers().len() == 3 );
                assert!( t.rows().len() == 0 );
            },
            _ => assert!( false, "expanded is not table" )    
        }
    }
}