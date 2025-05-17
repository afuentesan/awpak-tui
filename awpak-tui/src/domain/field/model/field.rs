use crate::domain::{input::model::input::Input, table::model::cell::Cell, util::string_utils::str_len};

#[derive(Clone)]
pub struct Field
{
    pub cell : Cell,

    pub id : String,
    pub name : String,

    pub input : Input,
}

impl Field
{
    pub fn own_input( mut self ) -> ( Self, Input )
    {
        let old = std::mem::replace( &mut self.input, Input::default() );

        ( self, old )
    }

    pub fn change_input( mut self, new : Input ) -> Self
    {
        self.input = new;

        self
    }
}

impl From<( &Cell, String, String )> for Field
{
    fn from( ( cell, id, name ) : ( &Cell, String, String ) ) -> Self 
    {
        let text = cell.to_string();
        let cursor_position = str_len( &text ) as u16;

        Self 
        { 
            cell : cell.clone(), 
            id,
            name,
            input : Input 
            { 
                text, 
                cursor_position
            }
        }
    }
}