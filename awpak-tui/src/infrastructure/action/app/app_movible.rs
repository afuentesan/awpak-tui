use std::sync::mpsc::Sender;

use crate::{application::{field::field::paste_on_field, movible::movible::{copy, cut, delete, paste}, search::search::paste_on_search}, domain::{app::model::app::{App, AppFocus}, result::result::AwpakResult}, infrastructure::{action::window::window_action::WindowAction, clipboard::clipboard::text_from_clipboard}};

use super::app_utils::{app_exec_action, app_exec_action_allways_refresh};


pub fn app_ctrl_c( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, copy )
}

pub fn app_ctrl_v( app : App, tx : Sender<WindowAction> ) -> App
{
    match app.focus()
    {
        AppFocus::Search => ctrl_v_on_input( app, tx, paste_on_search ),
        AppFocus::Field => ctrl_v_on_input( app, tx, paste_on_field ),
        _ => app_exec_action_allways_refresh( app, tx, paste )
    }
    
}

fn ctrl_v_on_input( app : App, tx : Sender<WindowAction>, fn_paste : fn( String ) -> Box<dyn Fn( App ) -> AwpakResult<App>> ) -> App
{
    match text_from_clipboard()
    {
        Some( t ) => app_exec_action( app, tx, fn_paste( t ) ),
        _ => app
    }
}

pub fn app_ctrl_x( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, cut )
}

pub fn app_ctrl_d( app : App, tx : Sender<WindowAction> ) -> App
{
    app_exec_action( app, tx, delete )
}