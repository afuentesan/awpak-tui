use std::sync::mpsc::Sender;

use crate::{application::sortable::sort::sort_by_focus, domain::{app::model::app::{App, AppFocus}, sortable::model::sortable::SortBy}, infrastructure::action::window::window_action::WindowAction};

use super::app_utils::app_exec_action;


pub fn app_alt_number( app : App, tx : Sender<WindowAction>, idx : usize ) -> App
{
    match app.focus()
    {
        AppFocus::Confirm( _ ) |
        AppFocus::Field => return app,
        _ => {}    
    };

    let sort_by = get_sort_by( &app, idx );

    app_exec_action( app, tx, sort_by_focus( sort_by ) )
}

fn get_sort_by( app : &App, idx : usize ) -> SortBy
{
    reverse_sort_if_equals(
        match app.focus()
        {
            AppFocus::Sources => ( app.sources_sort(), if idx == 1 { SortBy::Column( 0 ) } else { SortBy::Default } ),
            AppFocus::Content |
            AppFocus::Search |
            AppFocus::Back |
            AppFocus::Up |
            AppFocus::Next |
            AppFocus::Confirm( _ ) |
            AppFocus::Field => ( app.content_sort(), if idx == 0 { SortBy::Default } else { SortBy::Column( idx - 1 ) } )
        }
    )
}

fn reverse_sort_if_equals( ( app_sort, new_sort ) : ( SortBy, SortBy ) ) -> SortBy
{
    match ( app_sort, new_sort )
    {
        ( SortBy::Default, SortBy::Default ) => SortBy::ReverseDefault,
        ( SortBy::Column( a ), SortBy::Column( b ) ) if a == b => SortBy::ReverseColumn( a ),
        _ => new_sort    
    }
}