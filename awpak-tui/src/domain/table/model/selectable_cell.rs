use crate::domain::{result::result::AwpakResult, selectable::model::{selectable::Selectable, selectable_item::{default_selectable_item_current_selected, default_selectable_item_hidden, default_selectable_item_idle, default_selectable_item_selected, default_selectable_item_to_current_selected, default_selectable_item_to_hidden, default_selectable_item_to_idle, default_selectable_item_to_selected, SelectableItem}}};

use super::cell::Cell;

impl SelectableItem<Cell>
{
    pub fn change_value( self, value : &str ) -> AwpakResult<Self>
    {
        match self
        {
            Self::CurrentSelected( c ) => c.change_value( value ).finalize()
            .unzip( | c | Self::CurrentSelected( c ) ).read(), 
            Self::Selected( c ) => c.change_value( value ).finalize()
            .unzip( | c | Self::Selected( c ) ).read(),
            Self::Idle( c ) => c.change_value( value ).finalize()
            .unzip( | c | Self::Idle( c ) ).read(),
            Self::Hidden( c ) => c.change_value( value ).finalize()
            .unzip( | c | Self::Hidden( c ) ).read(), 
        }
    }
}

impl Selectable for SelectableItem<Cell>
{
    fn current_selected( &self ) -> bool
    {
        default_selectable_item_current_selected( self )
    }

    fn selected( &self ) -> bool 
    {
        default_selectable_item_selected( self )
    }

    fn idle( &self ) -> bool 
    {
        default_selectable_item_idle( self )
    }

    fn hidden( &self ) -> bool 
    {
        default_selectable_item_hidden( self )
    }

    fn to_current_selected( self ) -> Self 
    {
        default_selectable_item_to_current_selected( self )
    }

    fn to_selected( self ) -> Self 
    {
        default_selectable_item_to_selected( self )
    }

    fn to_idle( self ) -> Self 
    {
        default_selectable_item_to_idle( self )
    }

    fn to_hidden( self ) -> Self 
    {
        default_selectable_item_to_hidden( self )
    }

    fn can_be_selected( &self ) -> bool 
    {
        match self.inner()
        {
            Cell::Read( _ ) => true,
            Cell::Write( _ ) => true    
        }
    }
}