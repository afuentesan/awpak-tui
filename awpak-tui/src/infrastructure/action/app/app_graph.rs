use std::sync::mpsc::Sender;

use crate::{application::graph::graph::{append_text_to_graph_content, finalize_graph_response, graph_to_waiting, open_new_graph, open_saved_graph, pending_graph}, domain::app::model::app::App, infrastructure::action::{async_action::async_action::AsyncAction, window::window_action::{CursorDirection, WindowAction}}};

use super::app_utils::app_exec_action;

pub fn app_end_graph_response( app : App, tx : Sender<WindowAction> ) -> App
{
    let app = app_exec_action( app, tx.clone(), finalize_graph_response );

    let _ = tx.send( WindowAction::MoveCursorContent( CursorDirection::End ) );

    app
}

pub fn app_append_text_to_graph_content( app : App, tx : Sender<WindowAction>, text : String ) -> App
{
    app_exec_action( app, tx, append_text_to_graph_content( text ) )
}

pub fn app_alt_a( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, open_new_graph )
}

pub fn app_alt_s( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, open_saved_graph )
}

pub fn graph_post_processing( app : App, tx : Sender<WindowAction>, chat_sender : Sender<AsyncAction> ) -> App
{
    match pending_graph( &app )
    {
        Some( g ) =>
        {
            let app = graph_to_waiting( app );

            match app.collect()
            {
                ( a, None ) =>
                {
                    let _ = chat_sender.send( AsyncAction::SendGraphRequest( g ) );

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