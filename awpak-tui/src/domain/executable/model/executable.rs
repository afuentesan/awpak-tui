use crate::domain::{error::Error, path::path_utils::path_for_exec, selectable::model::selectable_item::SelectableItem, table::model::{cell::Cell, rowable::Rowable}};


#[derive(Clone)]
pub struct Executable
{
    path : std::path::PathBuf,
    params : Option<String>,
    cells : Vec<SelectableItem<Cell>>
}

impl Executable
{
    pub fn new( path : &str, params : Option<String>, cells : Vec<Cell> ) -> Result<Self, Error>
    {
        if cells.len() == 0
        {
            return Err( Error::ExpandableWithoutCells( format!( "Executable: {}", path ) ) )
        }

        let path = path_for_exec( path )?;

        Ok(
            Self
            {
                path,
                cells : cells.into_iter().map( | c | SelectableItem::Idle( c ) ).collect(),
                params
            }
        )
    }

    pub fn path( &self ) -> &std::path::Path
    {
        &self.path
    }

    pub fn params( &self ) -> Option<&String>
    {
        self.params.as_ref()
    }
}

impl Rowable for Executable
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

impl ToString for Executable
{
    fn to_string( &self ) -> String 
    {
        self.path().to_str().unwrap_or( &self.path().to_string_lossy() ).to_string()
    }
}