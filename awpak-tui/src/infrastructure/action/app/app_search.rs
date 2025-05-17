use std::sync::mpsc::Sender;

use crate::{application::{field::field::{change_edit_field, clear_field}, search::search::{change_regex_content_search, clear_search}}, domain::{app::model::app::{App, AppContent, AppFocus}, input::model::input::InputModifier}, infrastructure::action::window::window_action::WindowAction};

use super::app_utils::app_exec_action;

pub fn app_alt_x( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Search => app_exec_action( app, tx, clear_search ),
        AppFocus::Field => app_exec_action( app, tx, clear_field ),
        _ => app
    }
}

pub fn app_backspace( app : App, tx : Sender<WindowAction> ) -> App
{
    exec_in_input_field( app, tx, vec![ InputModifier::Delete ] )
}

pub fn app_char( app : App, tx : Sender<WindowAction>, c : char ) -> App
{
    exec_in_input_field( app, tx, vec![ InputModifier::Char( c ) ] )
}

pub fn app_alt_enter( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.content()
    {
        AppContent::Chat( _ ) => exec_in_input_field( app, tx, vec![ InputModifier::Char( '\n' ) ] ),
        _ => app
    }
}

fn exec_in_input_field( app : App, tx : Sender<WindowAction>, modifiers : Vec<InputModifier> ) -> App
{
    match app.focus()
    {
        AppFocus::Sources |
        AppFocus::Content |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Confirm( _ ) => app,
        AppFocus::Field => app_exec_action( app, tx, change_edit_field( modifiers ) ),
        AppFocus::Search => app_exec_action( app, tx, change_regex_content_search( modifiers ) )
    }
}