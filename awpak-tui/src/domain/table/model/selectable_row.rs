use crate::domain::selectable::model::{selectable::Selectable, selectable_item::{default_selectable_item_current_selected, default_selectable_item_hidden, default_selectable_item_idle, default_selectable_item_selected, default_selectable_item_to_current_selected, default_selectable_item_to_hidden, default_selectable_item_to_idle, default_selectable_item_to_selected, SelectableItem}};

use super::{row::Row, rowable::Rowable};

impl Selectable for SelectableItem<Row>
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

impl Rowable for SelectableItem<Row>
{
    fn cells( &self ) -> &Vec<SelectableItem<super::cell::Cell>>
    {
        match self
        {
            SelectableItem::CurrentSelected( r ) |
            SelectableItem::Selected( r ) |
            SelectableItem::Idle( r ) |
            SelectableItem::Hidden( r ) => r.cells()
        }
    }

    fn own_cells( self ) -> ( Box<Self>, Vec<SelectableItem<super::cell::Cell>> )
    {
        match self
        {
            SelectableItem::CurrentSelected( r ) =>
            {
                let ( r, cells ) = r.own_cells();

                (
                    Box::new( SelectableItem::CurrentSelected( *r ) ),
                    cells
                )
            },
            SelectableItem::Selected( r ) =>
            {
                let ( r, cells ) = r.own_cells();

                (
                    Box::new( SelectableItem::Selected( *r ) ),
                    cells
                )
            },
            SelectableItem::Idle( r ) =>
            {
                let ( r, cells ) = r.own_cells();

                (
                    Box::new( SelectableItem::Idle( *r ) ),
                    cells
                )
            },
            SelectableItem::Hidden( r ) =>
            {
                let ( r, cells ) = r.own_cells();

                (
                    Box::new( SelectableItem::Hidden( *r ) ),
                    cells
                )
            }
        }
    }

    fn change_cells( self, cells : Vec<SelectableItem<super::cell::Cell>> ) -> Box<Self>
    {
        match self
        {
            SelectableItem::CurrentSelected( r ) =>
            {
                Box::new( SelectableItem::CurrentSelected( *r.change_cells( cells ) ) )   
            },
            SelectableItem::Selected( r ) =>
            {
                Box::new( SelectableItem::Selected( *r.change_cells( cells ) ) )   
            },
            SelectableItem::Idle( r ) =>
            {
                Box::new( SelectableItem::Idle( *r.change_cells( cells ) ) ) 
            },
            SelectableItem::Hidden( r ) =>
            {
                Box::new( SelectableItem::Hidden( *r.change_cells( cells ) ) ) 
            }
        }
    }
}