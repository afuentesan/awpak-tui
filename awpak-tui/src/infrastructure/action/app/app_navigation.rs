use std::sync::mpsc::Sender;

use crate::{application::{field::field::change_edit_field, navigation::navigate::{back_from_chat, back_from_detail, history_back, history_next, parent_navigation}, search::search::{search_line_down, search_line_up}, selectable::change_selection::{append_or_remove_next_in_focus, append_or_remove_previous_in_focus, select_next_in_focus, select_previous_in_focus}}, domain::{app::model::app::{App, AppContent, AppFocus}, content_generator::model::content_generator::ContentGenerator, input::model::input::InputModifier}, infrastructure::action::window::window_action::{CursorDirection, WindowAction}};

use super::app_utils::app_exec_action;


pub fn app_up( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Field => app_exec_action( app, tx, change_edit_field( vec![ InputModifier::LineUp ] ) ),
        AppFocus::Content => app_up_content_focus( app, tx ),
        AppFocus::Search => app_exec_action( app, tx, search_line_up ),
        _ => app_exec_action( app, tx, select_previous_in_focus )
    }
}

fn app_up_content_focus( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.content()
    {
        AppContent::Chat( _ ) =>
        {
            let _ = tx.send( WindowAction::MoveCursorContent( CursorDirection::Up ) );

            app
        },
        _ => app_exec_action( app, tx, select_previous_in_focus )
    }
}

pub fn app_down( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Field => app_exec_action( app, tx, change_edit_field( vec![ InputModifier::LineDown ] ) ),
        AppFocus::Content => app_down_content_focus( app, tx ),
        AppFocus::Search => app_exec_action( app, tx, search_line_down ),
        _ => app_exec_action( app, tx, select_next_in_focus )
    }
}

fn app_down_content_focus( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.content()
    {
        AppContent::Chat( _ ) =>
        {
            let _ = tx.send( WindowAction::MoveCursorContent( CursorDirection::Down ) );

            app
        },
        _ => app_exec_action( app, tx, select_next_in_focus )
    }
}

pub fn app_shift_down( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, append_or_remove_next_in_focus )
}

pub fn app_shift_up( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, append_or_remove_previous_in_focus )
}

pub fn app_alt_left( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.content_generator()
    {
        ContentGenerator::Directory( _ ) |
        ContentGenerator::Expandable( _ ) |
        ContentGenerator::ExecutableExpandable( _ ) |
        ContentGenerator::Empty => app_exec_action( app, tx, history_back ),
        ContentGenerator::Detail( _, _ ) => app_exec_action( app, tx, back_from_detail ),
        ContentGenerator::Chat( _, _ ) => app_exec_action( app, tx, back_from_chat )
    }
}

pub fn app_alt_right( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, history_next )
}

pub fn app_alt_up( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, parent_navigation )
}