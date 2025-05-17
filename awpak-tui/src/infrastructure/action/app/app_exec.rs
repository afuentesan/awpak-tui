use std::sync::mpsc::Sender;

use crate::{application::{app::exec_focus::exec_focus, field::field::change_edit_field}, domain::{app::model::app::{App, AppFocus}, input::model::input::InputModifier}, infrastructure::action::window::window_action::WindowAction};

use super::app_utils::{app_exec_action, app_exec_action_allways_refresh};


pub fn app_enter( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Confirm( _ ) => app_exec_action_allways_refresh( app, tx, exec_focus ),
        AppFocus::Field => app_exec_action( app, tx, change_edit_field( vec![ InputModifier::Char( '\n' ) ] ) ),
        AppFocus::Sources |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Search |
        AppFocus::Content => app_exec_action( app, tx, exec_focus )
    }
}

pub fn app_ctrl_s( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Field => app_exec_action( app, tx, exec_focus ),
        _ => app
    }
}