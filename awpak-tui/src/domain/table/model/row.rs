use std::cmp::Ordering;

use crate::domain::{directory::model::directory::Directory, executable::model::executable::Executable, executable_expandable::model::executable_expandable::ExecutableExpandable, expandable::model::expandable::Expandable, file::model::file::File, selectable::model::selectable_item::SelectableItem, sortable::model::sortable::{SortBy, Sortable}, table::functions::table_sort::{default_row_sort, sort_row_by_column}};

use super::{cell::Cell, rowable::Rowable};


#[derive(Clone)]
pub enum Row
{
    Directory( Directory ),
    File( File ),
    Expandable( Expandable ),
    ExecutableExpandable( ExecutableExpandable ),
    Executable( Executable ),
    Data( Vec<SelectableItem<Cell>> ),
    DataRef { id : String, cells : Vec<SelectableItem<Cell>> }
}

impl Default for Row
{
    fn default() -> Self
    {
        Self::Data( vec![] )
    }
}

impl Row
{
    pub fn sort_preference( &self ) -> usize
    {
        match self
        {
            Self::Directory( _ ) => 0,
            Self::Expandable( _ ) => 1,
            Self::ExecutableExpandable( _ ) => 2,
            Self::Executable( _ ) => 3,
            Self::File( _ ) => 4,
            Self::Data( _ ) => 5,
            Self::DataRef { id : _, cells : _ } => 6
        }
    }

    // pub fn as_ref_id( &self ) -> Option<&str>
    // {
    //     match self
    //     {
    //         Row::DataRef { id, .. } => Some( id ),
    //         _ => None
    //     }
    // }
}

impl Rowable for Row
{
    fn cells( &self ) -> &Vec<SelectableItem<Cell>>
    {
        match self
        {
            Row::Directory( d ) => d.cells(),
            Row::File( f ) => f.cells(),
            Row::Expandable( e ) => e.cells(),
            Row::ExecutableExpandable( e ) => e.cells(),
            Row::Executable( e ) => e.cells(),
            Row::Data( c ) => c,
            Self::DataRef { id : _, cells } => cells
        }
    }
    
    fn own_cells( self ) -> ( Box<Self>, Vec<SelectableItem<Cell>> ) 
    {
        match self
        {
            Row::Directory( d ) => 
            {
                let ( d, cells ) = d.own_cells();

                ( Box::new( Self::Directory( *d ) ), cells )
            },
            Row::File( f ) => 
            {
                let ( f, cells ) = f.own_cells();

                ( Box::new( Self::File( *f ) ), cells )
            },
            Row::Expandable( e ) => 
            {
                let ( e, cells ) = e.own_cells();

                ( Box::new( Self::Expandable( *e ) ), cells )
            },
            Row::ExecutableExpandable( e ) => 
            {
                let ( e, cells ) = e.own_cells();

                ( Box::new( Self::ExecutableExpandable( *e ) ), cells )
            },
            Row::Executable( e ) => 
            {
                let ( e, cells ) = e.own_cells();

                ( Box::new( Self::Executable( *e ) ), cells )
            },
            Row::Data( c ) => 
            {
                ( Box::new( Self::Data( vec![] ) ), c )
            },
            Row::DataRef { id, cells } =>
            {
                (
                    Box::new( Self::DataRef 
                        { 
                            id, 
                            cells : vec![] 
                        }
                    ),
                    cells
                )
            }
        }
    }
    
    fn change_cells( self, cells : Vec<SelectableItem<Cell>> ) -> Box<Self> 
    {
        match self
        {
            Row::Directory( d ) => Box::new( Self::Directory( *d.change_cells( cells ) ) ),
            Row::File( f ) => Box::new( Self::File( *f.change_cells( cells ) ) ),
            Row::Expandable( e ) => Box::new( Self::Expandable( *e.change_cells( cells ) ) ),
            Row::ExecutableExpandable( e ) => Box::new( Self::ExecutableExpandable( *e.change_cells( cells ) ) ),
            Row::Executable( e ) => Box::new( Self::Executable( *e.change_cells( cells ) ) ),
            Row::Data( _ ) => Box::new( Self::Data( cells ) ),
            Row::DataRef { id, cells : _ } =>
            {
                Box::new(
                    Self::DataRef 
                    { 
                        id, 
                        cells
                    }
                )
            }
        }
    }
}

impl Sortable for Row
{
    fn sort( &self, other : &Self, sort_by : SortBy ) -> Ordering 
    {
        match sort_by
        {
            SortBy::Default => default_row_sort( self, other ),
            SortBy::ReverseDefault => Self::reverse( default_row_sort( self, other ) ),
            SortBy::Column( idx ) => sort_row_by_column( self, other, idx ),
            SortBy::ReverseColumn( idx ) => Self::reverse( sort_row_by_column( self, other, idx ) )
        }
    }
}