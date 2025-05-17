use std::path::PathBuf;

use crate::domain::{error::Error, path::path_utils::path_for_file, selectable::model::selectable_item::SelectableItem, table::model::{cell::Cell, rowable::Rowable}};

#[derive(Clone)]
pub struct File
{
    path : std::path::PathBuf,
    cells : Vec<SelectableItem<Cell>>
}

impl File
{
    pub fn new( path : &str, cells : Vec<Cell> ) -> Result<Self, Error>
    {
        if cells.len() == 0
        {
            return Err( Error::DirWithoutCells( format!( "FileWithoutCells: {}", path ) ) )
        }

        let path = path_for_file( path )?;

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

impl Rowable for File
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

#[cfg(test)]
mod tests
{
    use crate::domain::table::model::cell::CellType;

    use super::*;

    #[test]
    fn test_new_file()
    {
        let _ = std::fs::File::create( "/tmp/hola.txt" );

        let file = File::new( "/tmp/hola.txt", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( file.is_ok() );
    }

    #[test]
    fn test_new_file_empty_cells()
    {
        let _ = std::fs::File::create( "/tmp/hola.txt" );

        let file = File::new( "/tmp/hola.txt", vec![] );

        assert!( file.is_err() );
    }

    #[test]
    fn test_new_file_invalid_path()
    {
        let file = File::new( "/tmp/inventado_sdf", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( file.is_err() );
    }

    #[test]
    fn test_new_file_dir_path()
    {
        let file = File::new( "/tmp", vec![ Cell::Read( CellType::String( "Prueba".to_string() ) ) ] );

        assert!( file.is_err() );
    }
}