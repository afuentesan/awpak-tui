use std::sync::mpsc::Sender;

use crate::{application::{app::change_focus::{next_focus, previous_focus}, field::field::change_edit_field, search::search::change_regex_content_search}, domain::{app::model::app::{App, AppFocus}, input::model::input::InputModifier}, infrastructure::action::window::window_action::WindowAction};

use super::app_utils::app_exec_action;

pub fn app_tab( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Sources |
        AppFocus::Content |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Search => app_exec_action( app, tx, next_focus ),
        AppFocus::Confirm( _ ) |
        AppFocus::Field => app
    }
    // app_exec_action( app, tx, next_focus )
}

pub fn app_alt_tab( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Sources |
        AppFocus::Content |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Search => app_exec_action( app, tx, previous_focus ),
        AppFocus::Confirm( _ ) |
        AppFocus::Field => app
    }
}

pub fn app_right( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Sources |
        AppFocus::Content |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next => app_exec_action( app, tx, next_focus ),
        AppFocus::Search => app_exec_action( app, tx, change_regex_content_search( vec![ InputModifier::Right ] ) ),
        AppFocus::Field => app_exec_action( app, tx, change_edit_field( vec![ InputModifier::Right ] ) ),
        AppFocus::Confirm( _ ) => app
    }
}

pub fn app_left( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Sources |
        AppFocus::Content |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next => app_exec_action( app, tx, previous_focus ),
        AppFocus::Search => app_exec_action( app, tx, change_regex_content_search( vec![ InputModifier::Left ] ) ),
        AppFocus::Field => app_exec_action( app, tx, change_edit_field( vec![ InputModifier::Left ] ) ),
        AppFocus::Confirm( _ ) => app
    }
}