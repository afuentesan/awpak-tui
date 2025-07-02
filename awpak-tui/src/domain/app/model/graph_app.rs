use crate::domain::{graph::graph::AwpakTUIGraph, selectable::model::{selectable::Selectable, selectable_item::{default_selectable_item_current_selected, default_selectable_item_hidden, default_selectable_item_idle, default_selectable_item_selected, default_selectable_item_to_current_selected, default_selectable_item_to_hidden, default_selectable_item_to_idle, default_selectable_item_to_selected, SelectableItem}}};


impl Selectable for SelectableItem<AwpakTUIGraph>
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
}