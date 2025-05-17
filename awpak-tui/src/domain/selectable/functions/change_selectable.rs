use crate::domain::{error::Error, result::{functions::result_utils::bool_err, result::AwpakResult}, selectable::model::selectable::Selectable};

use super::selectable_utils::{idx_current_selected_item, idx_next_selectable_item, idx_next_selectable_item_stop_at_end, idx_previous_selectable_item, idx_previous_selectable_item_stop_at_start};

pub fn change_selection_from_to<T>( items : Vec<T>, from : Option<usize>, to : Option<usize> ) -> AwpakResult<Vec<T>>
where T: Selectable + Default
{
    AwpakResult::new( items )
    .validate()
    .map_result( | i | bool_err( to.is_none() || to.unwrap() >= i.len(), Error::Ignore ) )
    .map_result( | i | bool_err( from.is_some() && from.unwrap() >= i.len(), Error::Ignore ) )
    .write()
    .map(
        | i |
        {
            match from
            {
                Some( idx ) => change_to_idle( i, idx ),
                None => i
            }
        }
    )
    .map( | i | change_to_current_selected( i, to.unwrap() ) )
    .read()
}

pub fn change_to_current_selected<T>( items : Vec<T>, idx : usize ) -> Vec<T>
where T: Selectable + Default
{
    change_to( items, idx, | i | i.to_current_selected() )
}

pub fn change_to_idle<T>( items : Vec<T>, idx : usize ) -> Vec<T>
where T: Selectable + Default
{
    change_to( items, idx, | i | i.to_idle() )
}

fn change_to<T>( 
    mut items : Vec<T>, 
    idx : usize,
    fn_to : impl Fn( T ) -> T
) -> Vec<T>
where T: Selectable + Default
{
    let old = std::mem::replace( &mut items[ idx ], T::default() );

    let new = fn_to( old );

    items[ idx ] = new;

    items
}

pub fn append_or_remove_next_selection<T>( items : Vec<T> ) -> Vec<T>
where T: Selectable + Default
{
    append_or_remove_multiple_selection( items, idx_next_selectable_item_stop_at_end )
}

pub fn append_or_remove_previous_selection<T>( items : Vec<T> ) -> Vec<T>
where T: Selectable + Default
{
    append_or_remove_multiple_selection( items, idx_previous_selectable_item_stop_at_start )
}

fn append_or_remove_multiple_selection<T>(
    items : Vec<T>,
    fn_other_idx : impl Fn( &Vec<T>, usize ) -> Option<usize>
) -> Vec<T>
where T: Selectable + Default
{
    match idx_current_selected_item( &items )
    {
        Some( idx ) =>
        {
            let idx_next = fn_other_idx( &items, idx );

            if let Some( idx_next ) = idx_next
            {
                map_multiple_selection( idx, idx_next, items )
            }
            else
            {
                items
            }
        },
        _ => items
    }
}

fn map_multiple_selection<T>( current_idx : usize, other_idx : usize, mut items : Vec<T> ) -> Vec<T>
where T: Selectable + Default
{
    let other_selected = items[ other_idx ].selected();

    let current_old = std::mem::replace(&mut items[ current_idx ], T::default() );
    let other_old = std::mem::replace(&mut items[ other_idx ], T::default() );

    if other_selected
    {
        items[ current_idx ] = current_old.to_idle();
    }
    else
    {
        items[ current_idx ] = current_old.to_selected();
    }

    items[ other_idx ] = other_old.to_current_selected();

    items
}

pub fn select_next_or_first_or_none_if_all_hidden<T>( items : Vec<T> ) -> Vec<T>
where T: Selectable
{
    match select_next_selectable_item( items )
    {
        Ok( r ) => r,
        Err( ( r, _ ) ) =>
        {
            match select_first_selectable_item( r )
            {
                Ok( r ) |
                Err( ( r, _ ) ) => r    
            }
        }
    }
}

pub fn select_previous_or_last_or_none_if_all_hidden<T>( items : Vec<T> ) -> Vec<T>
where T: Selectable
{
    match select_previous_selectable_item( items )
    {
        Ok( r ) => r,
        Err( ( r, _ ) ) =>
        {
            match select_last_selectable_item( r )
            {
                Ok( r ) |
                Err( ( r, _ ) ) => r    
            }
        }
    }
}

pub fn select_next_selectable_item<T>( items : Vec<T> ) -> Result<Vec<T>, ( Vec<T>, Error )>
where T: Selectable
{
    let idx = idx_current_selected_item( &items );

    if idx.is_none()
    {
        return Err( ( items, Error::NoSelectedItems ) )
    }

    let idx = idx.unwrap();

    let idx = idx_next_selectable_item( &items, idx ).unwrap_or( idx );

    Ok(
        items.into_iter().enumerate()
        .map( 
            | ( i, e ) | select_if_none_selected_and_selectable( ( i != idx, e ) ).1
        )
        .collect::<Vec<_>>()
    )
}

fn select_previous_selectable_item<T>( items : Vec<T> ) -> Result<Vec<T>, ( Vec<T>, Error )>
where T: Selectable
{
    let idx = idx_current_selected_item( &items );

    if idx.is_none()
    {
        return Err( ( items, Error::NoSelectedItems ) )
    }

    let idx = idx.unwrap();

    let idx = idx_previous_selectable_item( &items, idx ).unwrap_or( idx );

    Ok(
        items.into_iter().enumerate()
        .map( 
            | ( i, e ) | select_if_none_selected_and_selectable( ( i != idx, e ) ).1
        )
        .collect::<Vec<_>>()
    )
}

pub fn select_first_selectable_item<T>( items : Vec<T> ) -> Result<Vec<T>, ( Vec<T>, Error )>
where T: Selectable
{
    let result = items.into_iter()
    .fold( 
        ( false, vec![] ), 
        | mut a, i |
        {
            let item = select_if_none_selected_and_selectable( ( a.0, i ) );

            a.0 = item.0;
            a.1.push( item.1 );

            a
        }
    );

    match result.0
    {
        true => Ok( result.1 ),
        false => Err( ( result.1, Error::NoSelectableItems ) )
    }
}

fn select_last_selectable_item<T>( items : Vec<T> ) -> Result<Vec<T>, ( Vec<T>, Error )>
where T: Selectable
{
    let mut result = items.into_iter().rev()
    .fold( 
        ( false, vec![] ), 
        | mut a, i |
        {
            let item = select_if_none_selected_and_selectable( ( a.0, i ) );

            a.0 = item.0;
            a.1.push( item.1 );

            a
        }
    );

    match result.0
    {
        true => 
        {
            result.1.reverse();

            Ok( result.1 )
        },
        false => Err( ( result.1, Error::NoSelectableItems ) )
    }
}

fn select_if_none_selected_and_selectable<T>( ( some_selected, item ) : ( bool, T ) ) -> ( bool, T )
where T: Selectable
{
    match some_selected
    {
        true => if item.current_selected() || item.selected() { ( true, item.to_idle() ) } else { ( true, item ) },
        false => if item.can_be_selected() && ( item.current_selected() || item.selected() || item.idle() ) { ( true, item.to_current_selected() ) } else { ( false, item ) }
    }
}

#[cfg(test)]
mod tests
{
    use crate::domain::selectable::model::selectable_item::SelectableItem;

    use super::*;
    
    #[test]
    fn test_select_previous_visible_item_no_visibles()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Hidden( "2".to_string() )
        ];

        let items = select_previous_selectable_item( items );

        assert!( items.is_err() );
    }

    #[test]
    fn test_select_previous_visible_item_no_selected()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() )
        ];

        let items = select_previous_selectable_item( items );

        assert!( items.is_err() );

        let err = items.err().unwrap();

        assert_eq!( err.0.len(), 2 );
    }

    #[test]
    fn test_select_previous_visible_item()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::CurrentSelected( "2".to_string() )
        ];

        let items = select_previous_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 2 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::CurrentSelected( "2".to_string() ),
            SelectableItem::Idle( "3".to_string() )
        ];

        let items = select_previous_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 3 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 2 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No selected item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "1.5".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::CurrentSelected( "3".to_string() )
        ];

        let items = select_previous_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 4 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 1 ].inner(), "1.5" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 2 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 2 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        match items[ 3 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 3 ].inner(), "3" ),
            _ => assert!( false, "No idle item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::CurrentSelected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::Selected( "5".to_string() ),
        ];

        let items = select_previous_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 5 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        match items[ 2 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 3 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 3 ].inner(), "4" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 4 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 4 ].inner(), "5" ),
            _ => assert!( false, "No idle item" )
        }
    }

    #[test]
    fn test_select_next_visible_item_no_visibles()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Hidden( "2".to_string() )
        ];

        let items = select_next_selectable_item( items );

        assert!( items.is_err() );

        let err = items.err().unwrap();

        assert_eq!( err.0.len(), 2 );
    }

    #[test]
    fn test_select_next_visible_item_no_selected()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() )
        ];

        let items = select_next_selectable_item( items );

        assert!( items.is_err() );
    }

    #[test]
    fn test_select_next_visible_item()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::CurrentSelected( "2".to_string() )
        ];

        let items = select_next_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 2 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::CurrentSelected( "2".to_string() ),
            SelectableItem::Idle( "3".to_string() )
        ];

        let items = select_next_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 3 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 2 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No selected item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::CurrentSelected( "3".to_string() )
        ];

        let items = select_next_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 3 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        match items[ 2 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No idle item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() ),
            SelectableItem::Selected( "3".to_string() ),
            SelectableItem::Idle( "4".to_string() ),
            SelectableItem::CurrentSelected( "5".to_string() ),
        ];

        let items = select_next_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 5 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        match items[ 2 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 3 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 3 ].inner(), "4" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 4 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 4 ].inner(), "5" ),
            _ => assert!( false, "No idle item" )
        }
    }

    #[test]
    fn test_select_last_visible_item_none_visible()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Hidden( "2".to_string() )
        ];

        let items = select_last_selectable_item( items );

        assert!( items.is_err() );

        assert_eq!( items.err().unwrap().0.len(), 2 );
    }

    #[test]
    fn test_select_last_visible_item()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() )
        ];

        let items = select_last_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 2 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::CurrentSelected( "2".to_string() ),
            SelectableItem::Idle( "3".to_string() )
        ];

        let items = select_last_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 3 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No idle item" )
        }

        match items[ 2 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No selected item" )
        }
    }

    #[test]
    fn test_select_first_visible_item_none_visible()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Hidden( "2".to_string() )
        ];

        let items = select_first_selectable_item( items );

        assert!( items.is_err() );

        assert_eq!( items.err().unwrap().0.len(), 2 );
    }

    #[test]
    fn test_select_first_visible_item()
    {
        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Idle( "2".to_string() )
        ];

        let items = select_first_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 2 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        let items = vec![ 
            SelectableItem::Hidden( "1".to_string() ),
            SelectableItem::Selected( "2".to_string() ),
            SelectableItem::CurrentSelected( "3".to_string() )
        ];

        let items = select_first_selectable_item( items );

        assert!( items.is_ok() );

        let items = items.unwrap();

        assert_eq!( items.len(), 3 );

        match items[ 0 ]
        {
            SelectableItem::Hidden( _ ) => assert_eq!( items[ 0 ].inner(), "1" ),
            _ => assert!( false, "No hidden item" )
        }

        match items[ 1 ]
        {
            SelectableItem::CurrentSelected( _ ) => assert_eq!( items[ 1 ].inner(), "2" ),
            _ => assert!( false, "No selected item" )
        }

        match items[ 2 ]
        {
            SelectableItem::Idle( _ ) => assert_eq!( items[ 2 ].inner(), "3" ),
            _ => assert!( false, "No idle item" )
        }
    }
}