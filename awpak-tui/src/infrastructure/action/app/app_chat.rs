use std::sync::mpsc::Sender;

use crate::{application::chat::chat::{append_text_to_content, chat_to_waiting, finalize_chat_response, open_saved_chat, open_new_chat, pending_chat}, domain::app::model::app::App, infrastructure::action::{async_action::async_action::AsyncAction, window::window_action::{CursorDirection, WindowAction}}};

use super::app_utils::app_exec_action;

pub fn app_end_chat_response( app : App, tx : Sender<WindowAction> ) -> App
{
    let app = app_exec_action( app, tx.clone(), finalize_chat_response );

    let _ = tx.send( WindowAction::MoveCursorContent( CursorDirection::End ) );

    app
}

pub fn app_append_text_to_content( app : App, tx : Sender<WindowAction>, text : String ) -> App
{
    app_exec_action( app, tx, append_text_to_content( text ) )
}

pub fn app_alt_a( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, open_new_chat )
}

pub fn app_alt_s( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, open_saved_chat )
}

pub fn chat_post_processing( app : App, tx : Sender<WindowAction>, chat_sender : Sender<AsyncAction> ) -> App
{
    match pending_chat( &app )
    {
        Some( c ) =>
        {
            let app = chat_to_waiting( app );

            match app.collect()
            {
                ( a, None ) =>
                {
                    let _ = chat_sender.send( AsyncAction::SendChatRequest( c ) );

                    let _ = tx.send( WindowAction::Render( a.clone() ) );

                    let _ = tx.send( WindowAction::MoveCursorContent( CursorDirection::End ) );

                    a
                },
                ( a, Some( _ ) ) => a
            }
        },
        _ => app
    }
}