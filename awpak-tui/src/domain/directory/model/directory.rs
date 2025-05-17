use std::path::PathBuf;

use crate::domain::{error::Error, path::path_utils::path_for_dir, selectable::model::selectable_item::SelectableItem, table::model::{cell::Cell, rowable::Rowable}};

#[derive(Clone)]
pub struct Directory
{
    path : std::path::PathBuf,
    cells : Vec<SelectableItem<Cell>>
}

impl Directory
{
    pub fn new( path : &str, cells : Vec<Cell> ) -> Result<Self, Error>
    {
        Self::new_from_path( path_for_dir( path )?, cells )
    }

    pub fn new_from_path( path : PathBuf, cells : Vec<Cell> ) -> Result<Self, Error>
    {
        if cells.len() == 0
        {
            return Err( Error::DirWithoutCells( format!( "DirWithoutCells: {}", path.to_string_lossy() ) ) )
        }

        Ok(
            Self
            {
                path,
                cells : cells.into_iter().map( | c | SelectableItem::Idle( c ) ).collect()
            }
        )
    }

    pub fn path( &self ) -> &std::path::Path
    {
        &self.path
    }

    pub fn change_path( mut self, new : PathBuf ) -> Self
    {
        self.path = new;

        self
    }
}

impl Rowable for Directory
{
    fn cells( &self ) -> &Vec<SelectableItem<Cell>>
    {
        &self.cells
    }
    
    fn own_cells( mut self ) -> ( Box<Self>, Vec<SelectableItem<Cell>> ) 
    {
        let old = std::mem::replace( &mut self.cells, vec![] );

        ( Box::new( self ), old )
    }
    
    fn change_cells( mut self, cells : Vec<SelectableItem<Cell>> ) -> Box<Self> 
    {
        self.cells = cells;

        Box::new( self )
    }
}

impl ToString for Directory
{
    fn to_string( &self ) -> String 
    {
        self.path().to_str().unwrap_or( &self.path().to_string_lossy() ).to_string()
    }
}

#[cfg(test)]
mod tests
{
    use std::fs::File;

    use crate::domain::table::model::cell::CellType;

    use super::*;

    #[test]
    fn test_new_dir()
    {
        let dir = Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( dir.is_ok() );
    }

    #[test]
    fn test_new_dir_empty_cells()
    {
        let dir = Directory::new( "/tmp", vec![] );

        assert!( dir.is_err() );
    }

    #[test]
    fn test_new_dir_invalid_path()
    {
        let dir = Directory::new( "/tmp/inventado_sdf", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( dir.is_err() );
    }

    #[test]
    fn test_new_dir_file_path()
    {
        let _ = File::create( "/tmp/hola.txt" );

        let dir = Directory::new( "/tmp/hola.txt", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( dir.is_err() );
    }
}