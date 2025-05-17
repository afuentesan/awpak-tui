use std::sync::mpsc::Receiver;

use ratatui::{buffer::Buffer, DefaultTerminal};

use crate::{domain::{app::model::app::App, error::Error}, infrastructure::{action::window::window_action::WindowAction, ui::{color::palette::Palette, loading::loading::{hide_loading, render_loading}, window::{content::move_cursor_content, state::WindowState, window::render_window}}}};


pub fn window_controller(
    rx : Receiver<WindowAction>
) -> impl FnMut( &mut DefaultTerminal ) -> Result<(), Error>
{
    move | terminal |
    {
        let mut window_state = WindowState::default();

        let mut last_buffer : Option<Buffer> = None;
        let mut last_action : Option<WindowAction> = None;
        let mut last_app : Option<App> = None;

        loop
        {
            let 
            ( 
                new_buffer, 
                new_last_action,
                new_last_app
            ) = match rx.recv()
            {
                Ok( a ) => match a
                {
                    WindowAction::Exit => break,
                    WindowAction::Render( app ) => ( render_window( &app, terminal, &mut window_state ), None, Some( app ) ),
                    WindowAction::ShowLoading => ( None, Some( a ), None ),
                    WindowAction::HideLoading =>
                    {
                        hide_loading( terminal, last_buffer.as_ref() );
                        
                        ( None, None, None )
                    },
                    WindowAction::ConfirmShowLoading =>
                    {
                        if let Some( WindowAction::ShowLoading ) = last_action
                        {
                            render_loading( terminal, last_buffer.as_ref(), &Palette::default() );
                        }

                        ( None, None, None )
                    },
                    WindowAction::MoveCursorContent( d ) =>
                    {
                        match last_app.as_ref()
                        {
                            Some( a ) =>
                            {
                                if move_cursor_content( a, &mut window_state, d, terminal.get_frame().area() )
                                {
                                    ( render_window( a, terminal, &mut window_state ), last_action, None )
                                }
                                else
                                {
                                    ( None, last_action, None )
                                }
                            },
                            _ => ( None, last_action, None )
                        }
                    }
                },
                _ => ( None, None, None )
            };

            last_action = new_last_action;

            if let Some( b ) = new_buffer
            {
                last_buffer = Some( b );
            }

            if let Some( a ) = new_last_app
            {
                last_app = Some( a );
            }
        }

        Ok( () )
    }
}