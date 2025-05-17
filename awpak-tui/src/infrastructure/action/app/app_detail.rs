use std::sync::mpsc::Sender;

use crate::{application::{confirm::confirm::discard_action, detail::show_detail::show_detail, message::message::hide_message, navigation::navigate::{back_from_chat, back_from_detail}}, domain::{app::model::app::{App, AppContent, AppFocus}, chat::functions::chat::is_chat_request_empty, error::Error, result::result::AwpakResult}, infrastructure::{action::window::window_action::WindowAction, channel::channel::send_abort_chat}};

use super::app_utils::{app_exec_action, app_exec_action_allways_refresh, app_exec_actions_while_err};



pub fn app_alt_i( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Content => app_exec_action( app, tx, show_detail ),
        AppFocus::Search => app,
        AppFocus::Sources |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Confirm( _ ) |
        AppFocus::Field => app
    }
}

pub fn app_esc( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Confirm( _ ) |
        AppFocus::Field => app_exec_action_allways_refresh( app, tx, discard_action ),
        AppFocus::Content |
        AppFocus::Search |
        AppFocus::Sources |
        AppFocus::Back |
        AppFocus::Next |
        AppFocus::Up => app_exec_actions_while_err( app, tx, vec![ cancel_chat, hide_message, back_from_detail, back_from_chat ] )
    }
}

fn cancel_chat( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( c ) if ! is_chat_request_empty( c.request() ) =>
        {
            send_abort_chat( true );

            AwpakResult::new( app )
        },
        _ => AwpakResult::new_err( app, Error::Ignore )
    }
}