use crate::domain::selectable::model::selectable_item::SelectableItem;

use super::cell::Cell;


pub trait Rowable
{
    fn cells( &self ) -> &Vec<SelectableItem<Cell>>;
    fn own_cells( self ) -> ( Box<Self>, Vec<SelectableItem<Cell>> );
    fn change_cells( self, cells : Vec<SelectableItem<Cell>> ) -> Box<Self>;
}