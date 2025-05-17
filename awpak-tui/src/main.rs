use std::sync::mpsc::Sender;

use application::{app::init_app::init_app, source::init_sources::table_sources};
use domain::{app::model::app::App, error::Error, selectable::model::selectable_item::SelectableItem};
use infrastructure::{action::{action_channel::{init_channels, ActionChannel}, app::action::Action}, channel::channel::init_global_channels, config::functions::{ai_config::ai_config, config::{rows_from_sources_config, sources_config}}, controller::{app_controller::app_controller, async_controller::async_controller, window_controller::window_controller}, event::handle_events::init_handle_events};
use ratatui::DefaultTerminal;



mod domain;
mod application;
mod infrastructure;

fn main() -> Result<(), Error>
{
    let app = new_app()?;

    init_global_channels();

    let ( mut window, app_sender ) = init_controllers( app );

    init_handle_events( app_sender );

    let mut terminal = ratatui::init();

    let app_result = window( &mut terminal );

    ratatui::restore();

    app_result
}

fn new_app() -> Result<App, Error>
{
    let sources = table_sources( rows_from_sources_config( sources_config() ) )?;

    init_app( sources ).map( 
        | a |
        {
            match ai_config()
            {
                Some( c ) => a.change_ai_agents( 
                    c.agents.into_iter().map( | a | SelectableItem::Idle( a ) ).collect()
                ),
                None => a
            }
        }
    )
}

fn init_controllers( app : App ) -> 
( impl FnMut( &mut DefaultTerminal ) -> Result<(), Error>, Sender<Action> )
{
    let ActionChannel {
        app_sender,
        app_recv,
        window_sender,
        window_recv,
        chat_sender,
        chat_recv
    } = init_channels();

    app_controller( app, app_recv, window_sender, chat_sender );

    async_controller( app_sender.clone(), chat_recv );

    ( window_controller( window_recv ), app_sender )
}
