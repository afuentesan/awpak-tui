use crate::domain::selectable::model::selectable::Selectable;

pub fn idx_current_selected_item<T>( items : &Vec<T> ) -> Option<usize>
where T: Selectable
{
    items.iter().enumerate()
    .find( | s | s.1.current_selected() )
    .map( | s | s.0 )
}

// pub fn idx_first_selected_item<T>( items : &Vec<T> ) -> Option<usize>
// where T: Selectable
// {
//     items.iter().enumerate()
//     .find( | s | s.1.selected() )
//     .map( | s | s.0 )
// }

// pub fn idx_last_selected_item<T>( items : &Vec<T> ) -> Option<usize>
// where T: Selectable
// {
//     items.iter().enumerate()
//     .rev()
//     .find( | s | s.1.selected() )
//     .map( | s | s.0 )
// }

pub fn idx_current_selected_item_filter_hidden<T>( items : &Vec<T> ) -> Option<usize>
where T: Selectable
{
    let no_hiddens = items.iter().filter( | i | ! i.hidden() ).collect::<Vec<_>>();

    no_hiddens.iter().enumerate()
    .rev()
    .find( | s | s.1.current_selected() )
    .map( | s | s.0 )
}

// pub fn idx_first_visible_item<T>( items : &Vec<T> ) -> Option<usize>
// where T: Selectable
// {
//     items.iter().enumerate().find( | i | i.1.selected() || i.1.idle() ).map( | i | i.0 )
// }

// pub fn idx_last_visible_item<T>( items : &Vec<T> ) -> Option<usize>
// where T: Selectable
// {
//     items.iter().enumerate().rev().find( | i | i.1.selected() || i.1.idle() ).map( | i | i.0 )
// }

pub fn idx_first_selectable_item<T>( items : &Vec<T> ) -> Option<usize>
where T: Selectable
{
    items.iter().enumerate()
    .find( | i | i.1.can_be_selected() && ! i.1.hidden() )
    .map( | i | i.0 )
}

pub fn idx_next_or_first_selectable_item<T>( items : &Vec<T> ) -> Option<usize>
where T: Selectable
{
    match idx_current_selected_item( items )
    {
        Some( idx ) => idx_next_selectable_item_stop_at_end( items, idx ),
        None => idx_first_selectable_item( items )
    }
}

pub fn idx_next_selectable_item_stop_at_end<T>( items : &Vec<T>, ind : usize ) -> Option<usize>
where T: Selectable
{
    items.iter().enumerate().skip( ind + 1 )
    .filter( | i | i.1.can_be_selected() )
    .find( | i | ! i.1.hidden() )
    .map( | i | i.0 )
}

pub fn idx_previous_selectable_item_stop_at_start<T>( items : &Vec<T>, ind : usize ) -> Option<usize>
where T: Selectable
{
    items.iter().enumerate().take( ind ).rev()
    .filter( | i | i.1.can_be_selected() )
    .find( | i | ! i.1.hidden() )
    .map( | i | i.0 )
}

pub fn idx_next_selectable_item<T>( items : &Vec<T>, ind : usize ) -> Option<usize>
where T: Selectable
{
    let around = items.iter()
        .enumerate()
        .fold( ( -1, -1 ), | mut a, v |
            {
                if v.0 == ind || items[ v.0 ].hidden() || ! items[ v.0 ].can_be_selected()
                {
                    a
                }
                else if v.0 > ind
                {
                    if a.0 >= 0 { a } else { a.0 = v.0 as i32; a }
                }
                else
                {
                    if a.1 >= 0 { a } else { a.1 = v.0 as i32; a }
                }
            } 
        );

    match around
    {
        ( r, _ ) if r >= 0 => Some( r as usize ),
        ( _, l ) if l >= 0 => Some( l as usize ),
        _ => None
    }
}

pub fn idx_previous_selectable_item<T>( items : &Vec<T>, ind : usize ) -> Option<usize>
where T: Selectable
{
    let around = items.iter()
        .enumerate()
        .fold( ( -1, -1 ), | mut a, v |
            {
                if v.0 == ind || items[ v.0 ].hidden() || ! items[ v.0 ].can_be_selected()
                {
                    a
                }
                else if v.0 > ind
                {
                    a.1 = v.0 as i32; 
                    a
                }
                else
                {
                    a.0 = v.0 as i32;
                    a
                }
            } 
        );

    match around
    {
        ( l, _ ) if l >= 0 => Some( l as usize ),
        ( _, r ) if r >= 0 => Some( r as usize ),
        _ => None
    }
}

#[cfg(test)]
mod tests
{
    use crate::domain::selectable::model::selectable_item::{default_selectable_item_current_selected, default_selectable_item_hidden, default_selectable_item_idle, default_selectable_item_selected, default_selectable_item_to_current_selected, default_selectable_item_to_hidden, default_selectable_item_to_idle, default_selectable_item_to_selected, SelectableItem};

    use super::*;

    impl Selectable for SelectableItem<String>
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
    fn test_idx_previous_visible_item_all_hidden()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Hidden( "2".to_string() )
        ];

        let items = idx_previous_selectable_item( &items, 0 );

        assert!( items.is_none() );
    }

    #[test]
    fn test_idx_previous_visible_item()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Hidden( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Hidden( "5".to_string() ),
            SelectableItem::Idle( "6".to_string() ),
        ];

        let idx = idx_previous_selectable_item( &items, 3 );

        assert!( idx.is_some() );

        let idx = idx.unwrap();

        assert_eq!( idx, 1 );

        let items = vec![ 
            SelectableItem::Idle( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Selected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Hidden( "5".to_string() ),
            SelectableItem::Idle( "6".to_string() ),
        ];

        let idx = idx_previous_selectable_item( &items, 0 );

        assert!( idx.is_some() );

        let idx = idx.unwrap();

        assert_eq!( idx, 5 );

        let items = vec![ 
            SelectableItem::Selected( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Selected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Idle( "5".to_string() ),
            SelectableItem::Hidden( "6".to_string() ),
        ];

        let idx = idx_previous_selectable_item( &items, 0 );

        assert!( idx.is_some() );

        let idx = idx.unwrap();

        assert_eq!( idx, 4 );
    }

    #[test]
    fn test_idx_next_visible_item_all_hidden()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Hidden( "2".to_string() )
        ];

        let items = idx_next_selectable_item( &items, 0 );

        assert!( items.is_none() );
    }

    #[test]
    fn test_idx_next_visible_item()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Selected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Hidden( "5".to_string() ),
            SelectableItem::Idle( "6".to_string() ),
        ];

        let idx = idx_next_selectable_item( &items, 3 );

        assert!( idx.is_some() );

        let idx = idx.unwrap();

        assert_eq!( idx, 5 );

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Selected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Hidden( "5".to_string() ),
            SelectableItem::Idle( "6".to_string() ),
        ];

        let idx = idx_next_selectable_item( &items, 5 );

        assert!( idx.is_some() );

        let idx = idx.unwrap();

        assert_eq!( idx, 1 );

        let items = vec![ 
            SelectableItem::Selected( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Selected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Idle( "5".to_string() ),
            SelectableItem::Hidden( "6".to_string() ),
        ];

        let idx = idx_next_selectable_item( &items, 4 );

        assert!( idx.is_some() );

        let idx = idx.unwrap();

        assert_eq!( idx, 0 );
    }

    // #[test]
    // fn test_idx_last_selected_item_all_hidden()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Hidden( "2".to_string() )
    //     ];

    //     let items = idx_last_selected_item( &items );

    //     assert!( items.is_none() );
    // }

    // #[test]
    // fn test_idx_last_selected_item_none_selected()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Idle( "2".to_string() )
    //     ];

    //     let items = idx_last_selected_item( &items );

    //     assert!( items.is_none() );
    // }

    // #[test]
    // fn test_idx_last_selected_item()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Idle( "2".to_string() ),
    //         SelectableItem::Selected( "3".to_string() ),
    //         SelectableItem::Idle( "4".to_string() ),
    //         SelectableItem::Selected( "5".to_string() ),
    //         SelectableItem::Idle( "6".to_string() ),
    //     ];

    //     let idx = idx_last_selected_item( &items );

    //     assert!( idx.is_some() );

    //     let idx = idx.unwrap();

    //     assert_eq!( idx, 4 );
    // }

    // #[test]
    // fn test_idx_no_first_selected_item()
    // {
    //     let table = vec![ 
    //         SelectableItem::Idle(
    //             Row::Directory( 
    //                 Directory::new( 
    //                     "/tmp", 
    //                     vec![ Cell::Read( CellType::String( "Item 1".to_string() ) ) ] 
    //                 ).unwrap() 
    //             ) 
    //         ),
    //         SelectableItem::Idle(
    //             Row::Directory( 
    //                 Directory::new( 
    //                     "/home/angel", 
    //                     vec![ Cell::Read( CellType::String( "Item 2".to_string() ) ) ] 
    //                 ).unwrap() 
    //             ) 
    //         )
    //     ];

    //     assert!( idx_first_selected_item( &table ).is_none() );

    //     let table = vec![
    //         SelectableItem::Idle(
    //             Row::Directory(
    //                 Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Cell prueba".to_string() ) ) ] ).unwrap()
    //             )
    //         )
    //     ];

    //     assert!( idx_first_selected_item( &table ).is_none() );
    // }

    // #[test]
    // fn test_idx_first_selected_item()
    // {
    //     let table = vec![ 
    //         SelectableItem::Idle(
    //             Row::Directory( 
    //                 Directory::new( 
    //                     "/tmp", 
    //                     vec![ Cell::Read( CellType::String( "Item 1".to_string() ) ) ] 
    //                 ).unwrap() 
    //             ) 
    //         ),
    //         SelectableItem::Selected(
    //             Row::Directory( 
    //                 Directory::new( 
    //                     "/home/angel", 
    //                     vec![ Cell::Read( CellType::String( "Item 2".to_string() ) ) ] 
    //                 ).unwrap() 
    //             ) 
    //         )
    //     ];

    //     let first_selected = idx_first_selected_item( &table );

    //     assert!( first_selected.is_some() );

    //     let first_selected = first_selected.unwrap();

    //     assert_eq!( first_selected, 1 );

    // }

    // #[test]
    // fn test_idx_no_last_visible_item()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Hidden( "2".to_string() )
    //     ];

    //     let idx = idx_last_visible_item( &items );

    //     assert!( idx.is_none() );

    //     let items : Vec<SelectableItem<String>> = vec![];

    //     let idx = idx_last_visible_item( &items );

    //     assert!( idx.is_none() );
    // }

    // #[test]
    // fn test_idx_last_visible_item()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Idle( "2".to_string() )
    //     ];

    //     let idx = idx_last_visible_item( &items );

    //     assert!( idx.is_some() );

    //     assert_eq!( idx.unwrap(), 1 );

    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Hidden( "2".to_string() ),
    //         SelectableItem::Selected( "3".to_string() ),
    //         SelectableItem::Idle( "4".to_string() )
    //     ];

    //     let idx = idx_last_visible_item( &items );

    //     assert!( idx.is_some() );

    //     assert_eq!( idx.unwrap(), 3 );

    //     let items = vec![ 
    //         SelectableItem::Selected( "1".to_string() ),
    //         SelectableItem::Hidden( "2".to_string() ),
    //         SelectableItem::Hidden( "3".to_string() ),
    //         SelectableItem::Hidden( "4".to_string() )
    //     ];

    //     let idx = idx_last_visible_item( &items );

    //     assert!( idx.is_some() );

    //     assert_eq!( idx.unwrap(), 0 );
    // }

    // #[test]
    // fn test_idx_no_first_visible_item()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Hidden( "2".to_string() )
    //     ];

    //     let idx = idx_first_visible_item( &items );

    //     assert!( idx.is_none() );

    //     let items : Vec<SelectableItem<String>> = vec![];

    //     let idx = idx_first_visible_item( &items );

    //     assert!( idx.is_none() );
    // }

    // #[test]
    // fn test_idx_first_visible_item()
    // {
    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Idle( "2".to_string() )
    //     ];

    //     let idx = idx_first_visible_item( &items );

    //     assert!( idx.is_some() );

    //     assert_eq!( idx.unwrap(), 1 );

    //     let items = vec![ 
    //         SelectableItem::Hidden( "1".to_string() ),
    //         SelectableItem::Hidden( "2".to_string() ),
    //         SelectableItem::Selected( "3".to_string() ),
    //         SelectableItem::Idle( "4".to_string() )
    //     ];

    //     let idx = idx_first_visible_item( &items );

    //     assert!( idx.is_some() );

    //     assert_eq!( idx.unwrap(), 2 );
    // }
}