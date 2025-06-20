use std::sync::mpsc::{Receiver, Sender};

use crate::{domain::app::model::app::App, infrastructure::action::{app::{action::Action, app_chat::{app_alt_a, app_alt_s, app_append_text_to_content, app_end_chat_response, chat_post_processing}, app_detail::{app_alt_i, app_esc}, app_exec::{app_ctrl_s, app_enter}, app_focus::{app_alt_tab, app_left, app_right, app_tab}, app_movible::{app_ctrl_c, app_ctrl_d, app_ctrl_v, app_ctrl_x}, app_navigation::{app_alt_left, app_alt_right, app_alt_up, app_down, app_shift_down, app_shift_up, app_up}, app_search::{app_alt_enter, app_alt_x, app_backspace, app_char}, app_sort::app_alt_number}, async_action::async_action::AsyncAction, window::window_action::WindowAction}};


pub fn app_controller( app : App, rx : Receiver<Action>, tx : Sender<WindowAction>, chat_sender : Sender<AsyncAction> )
{
    std::thread::spawn( move || handle_app_actions( app, rx, tx, chat_sender ) );
}

fn handle_app_actions( mut app : App, rx : Receiver<Action>, tx : Sender<WindowAction>, chat_sender : Sender<AsyncAction> )
{
    loop
    {
        match rx.recv()
        {
            Ok( a ) => 
            {
                app = execute_app_action( a, app, tx.clone() );

                app = app_post_processing( app, tx.clone(), chat_sender.clone() );
            },
            _ => break
        }
    }
}

fn app_post_processing( app : App, tx : Sender<WindowAction>, chat_sender : Sender<AsyncAction> ) -> App
{
    chat_post_processing( app, tx, chat_sender )
}

fn execute_app_action( action : Action, app : App, tx : Sender<WindowAction> ) -> App
{
    match action
    {
        Action::Exit => app_exit( app, tx ),
        Action::Render => app_render( app, tx ),
        Action::Down => app_down( app, tx ),
        Action::ShiftDown => app_shift_down( app, tx ),
        Action::Up => app_up( app, tx ),
        Action::ShiftUp => app_shift_up( app, tx ),
        Action::Left => app_left( app, tx ),
        Action::Right => app_right( app, tx ),
        Action::Enter => app_enter( app, tx ),
        Action::AltEnter => app_alt_enter( app, tx ),
        Action::Tab => app_tab( app, tx ),
        Action::AltTab => app_alt_tab( app, tx ),
        Action::AltX => app_alt_x( app, tx ),
        Action::AltLeft => app_alt_left( app, tx ),
        Action::AltRight => app_alt_right( app, tx ),
        Action::AltUp => app_alt_up( app, tx ),
        Action::AltI => app_alt_i( app, tx ),
        Action::AltA => app_alt_a( app, tx ),
        Action::AltS => app_alt_s( app, tx ),
        Action::AltNumber( n ) => app_alt_number( app, tx, n ),
        Action::Backspace => app_backspace( app, tx ),
        Action::Char( c ) => app_char( app, tx, c ),
        Action::Esc => app_esc( app, tx ),
        Action::CtrlC => app_ctrl_c( app, tx ),
        Action::CtrlV => app_ctrl_v( app, tx ),
        Action::CtrlX => app_ctrl_x( app, tx ),
        Action::CtrlD => app_ctrl_d( app, tx ),
        Action::CtrlS => app_ctrl_s( app, tx ),
        Action::AppendTextToContent( t ) => app_append_text_to_content( app, tx, t.replace( "\t", "    " ) ),
        Action::EndChatResponse => app_end_chat_response( app, tx )
    }
}

fn app_exit( app : App, tx : Sender<WindowAction> ) -> App
{
    let _ = tx.send( WindowAction::Exit );

    app
}

fn app_render( app : App, tx : Sender<WindowAction> ) -> App
{
    let _ = tx.send( WindowAction::Render( app.clone() ) );

    app
}