use crate::domain::{error::Error, path::path_utils::path_for_file, selectable::model::selectable_item::SelectableItem, table::model::{cell::Cell, rowable::Rowable}};


#[derive(Clone)]
pub struct Expandable
{
    path : std::path::PathBuf,
    cells : Vec<SelectableItem<Cell>>
}

impl Expandable
{
    pub fn new( path : &str, cells : Vec<Cell> ) -> Result<Self, Error>
    {
        if cells.len() == 0
        {
            return Err( Error::ExpandableWithoutCells( format!( "ExpandableWithoutCells: {}", path ) ) )
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
}

impl Rowable for Expandable
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

impl ToString for Expandable
{
    fn to_string( &self ) -> String 
    {
        self.path().to_str().unwrap_or( &self.path().to_string_lossy() ).to_string()
    }
}