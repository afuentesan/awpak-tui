
#[derive(Debug, Clone)]
pub enum SelectableItem<T>
where T: Default
{
    CurrentSelected( T ),
    Selected( T ),
    Idle( T ),
    Hidden( T )
}

impl<T> SelectableItem<T>
where T: Default
{
    pub fn inner( &self ) -> &T
    {
        match self
        {
            SelectableItem::Selected( i ) | 
            SelectableItem::Idle( i ) |
            SelectableItem::Hidden( i ) |
            SelectableItem::CurrentSelected( i ) => i
        }
    }

    pub fn own_inner( self ) -> ( Self, T )
    {
        match self
        {
            SelectableItem::Selected( i ) => ( SelectableItem::Selected( T::default() ), i ),
            SelectableItem::Idle( i ) => ( SelectableItem::Idle( T::default() ), i ),
            SelectableItem::Hidden( i ) => ( SelectableItem::Hidden( T::default() ), i ),
            SelectableItem::CurrentSelected( i ) => ( SelectableItem::CurrentSelected( T::default() ), i )
        }
    }

    pub fn change_inner( self, new : T ) -> Self
    {
        match self
        {
            SelectableItem::Selected( _ ) => SelectableItem::Selected( new ),
            SelectableItem::Idle( _ ) => SelectableItem::Idle( new ),
            SelectableItem::Hidden( _ ) => SelectableItem::Hidden( new ),
            SelectableItem::CurrentSelected( _ ) => SelectableItem::CurrentSelected( new )
        }
    }
}

impl<T> Default for SelectableItem<T>
where T: Default
{
    fn default() -> Self
    {
        Self::Hidden( T::default() )
    }
}

pub fn default_selectable_item_current_selected<T>( selectable : &SelectableItem<T> ) -> bool
where T: Default
{
    if let SelectableItem::CurrentSelected( _ ) = selectable { true } else { false }
}

pub fn default_selectable_item_selected<T>( selectable : &SelectableItem<T> ) -> bool
where T: Default
{
    if let SelectableItem::Selected( _ ) = selectable { true } else { false }
}

pub fn default_selectable_item_idle<T>( selectable : &SelectableItem<T> ) -> bool
where T: Default
{
    if let SelectableItem::Idle( _ ) = selectable { true } else { false }
}

pub fn default_selectable_item_hidden<T>( selectable : &SelectableItem<T> ) -> bool
where T: Default
{
    if let SelectableItem::Hidden( _ ) = selectable { true } else { false }
}

pub fn default_selectable_item_to_current_selected<T>( selectable : SelectableItem<T> ) -> SelectableItem<T>
where T: Default
{
    match selectable
    {
        SelectableItem::Idle( s ) | SelectableItem::CurrentSelected( s ) | SelectableItem::Selected( s ) | SelectableItem::Hidden( s ) => SelectableItem::CurrentSelected( s )
    }
}

pub fn default_selectable_item_to_selected<T>( selectable : SelectableItem<T> ) -> SelectableItem<T>
where T: Default
{
    match selectable
    {
        SelectableItem::Idle( s ) | SelectableItem::CurrentSelected( s ) | SelectableItem::Selected( s ) | SelectableItem::Hidden( s ) => SelectableItem::Selected( s )
    }
}

pub fn default_selectable_item_to_idle<T>( selectable : SelectableItem<T> ) -> SelectableItem<T>
where T: Default
{
    match selectable
    {
        SelectableItem::Idle( s ) | SelectableItem::CurrentSelected( s ) | SelectableItem::Selected( s ) | SelectableItem::Hidden( s ) => SelectableItem::Idle( s )
    }
}

pub fn default_selectable_item_to_hidden<T>( selectable : SelectableItem<T> ) -> SelectableItem<T>
where T: Default
{
    match selectable
    {
        SelectableItem::Idle( s ) | SelectableItem::CurrentSelected( s ) | SelectableItem::Selected( s ) | SelectableItem::Hidden( s ) => SelectableItem::Hidden( s )
    }
}

// impl<T> Selectable for SelectableItem<T>
// where T: Default
// {
//     fn current_selected( &self ) -> bool
//     {
//         if let Self::CurrentSelected( _ ) = self { true } else { false }
//     }

//     fn selected( &self ) -> bool
//     {
//         if let Self::Selected( _ ) = self { true } else { false }
//     }

//     fn idle( &self ) -> bool
//     {
//         if let Self::Idle( _ ) = self { true } else { false }
//     }

//     fn hidden( &self ) -> bool
//     {
//         if let Self::Hidden( _ ) = self { true } else { false }
//     }

//     fn to_current_selected( self ) -> Self
//     {
//         match self
//         {
//             Self::Idle( s ) | Self::CurrentSelected( s ) | Self::Selected( s ) | Self::Hidden( s ) => Self::CurrentSelected( s )
//         }
//     }

//     fn to_selected( self ) -> Self
//     {
//         match self
//         {
//             Self::Idle( s ) | Self::CurrentSelected( s ) | Self::Selected( s ) | Self::Hidden( s ) => Self::Selected( s )
//         }
//     }

//     fn to_idle( self ) -> Self
//     {
//         match self
//         {
//             Self::Idle( s ) | Self::CurrentSelected( s ) | Self::Selected( s ) | Self::Hidden( s ) => Self::Idle( s )
//         }
//     }

//     fn to_hidden( self ) -> Self
//     {
//         match self
//         {
//             Self::Idle( s ) | Self::CurrentSelected( s ) | Self::Selected( s ) | Self::Hidden( s ) => Self::Hidden( s )
//         }
//     }
// }

#[cfg(test)]
mod tests
{
    use crate::domain::selectable::model::selectable::Selectable;

    use super::*;

    impl Selectable for SelectableItem<&str>
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

    #[test]
    fn test_selected()
    {
        let s = SelectableItem::Selected( "s" );

        assert!( s.selected() );
        assert!( ! s.idle() );
        assert!( ! s.hidden() );
    }

    #[test]
    fn test_idle()
    {
        let s = SelectableItem::Idle( "s" );

        assert!( ! s.selected() );
        assert!( s.idle() );
        assert!( ! s.hidden() );
    }

    #[test]
    fn test_hidden()
    {
        let s = SelectableItem::Hidden( "s" );

        assert!( ! s.selected() );
        assert!( ! s.idle() );
        assert!( s.hidden() );
    }

    #[test]
    fn test_to_selected()
    {
        let s = SelectableItem::Hidden( "s" ).to_selected();

        assert!( s.selected() );
        assert!( ! s.idle() );
        assert!( !s.hidden() );

        let s = SelectableItem::Idle( "s" ).to_selected();

        assert!( s.selected() );
        assert!( ! s.idle() );
        assert!( !s.hidden() );

        let s = SelectableItem::Selected( "s" ).to_selected();

        assert!( s.selected() );
        assert!( ! s.idle() );
        assert!( !s.hidden() );
    }

    #[test]
    fn test_to_idle()
    {
        let s = SelectableItem::Hidden( "s" ).to_idle();

        assert!( ! s.selected() );
        assert!( s.idle() );
        assert!( !s.hidden() );

        let s = SelectableItem::Idle( "s" ).to_idle();

        assert!( ! s.selected() );
        assert!( s.idle() );
        assert!( !s.hidden() );

        let s = SelectableItem::Selected( "s" ).to_idle();

        assert!( ! s.selected() );
        assert!( s.idle() );
        assert!( !s.hidden() );
    }

    #[test]
    fn test_to_hidden()
    {
        let s = SelectableItem::Hidden( "s" ).to_hidden();

        assert!( ! s.selected() );
        assert!( ! s.idle() );
        assert!( s.hidden() );

        let s = SelectableItem::Idle( "s" ).to_hidden();

        assert!( ! s.selected() );
        assert!( ! s.idle() );
        assert!( s.hidden() );

        let s = SelectableItem::Selected( "s" ).to_hidden();

        assert!( ! s.selected() );
        assert!( ! s.idle() );
        assert!( s.hidden() );
    }
}