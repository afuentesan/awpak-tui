use ratatui::{buffer::Buffer, DefaultTerminal, Frame};

use crate::{domain::app::model::app::App, infrastructure::ui::{areas::areas::create_areas, color::palette::Palette}};

use super::{confirm::render_confirm, container::render_container, content::{render_content, render_content_generator}, edit_field::render_edit_field, lines::render_lines, message::render_message, navigation::render_navigation, search::render_search_input, sources::render_sources, state::WindowState};


pub fn render_window<'a>( app : &'a App, terminal : &'a mut DefaultTerminal, state : &'a mut WindowState ) -> Option<Buffer>
{
    let r = terminal.draw( | frame |
        {
            draw_frame( app, frame, state );
        } 
    );

    match r
    {
        Ok( c ) => Some( c.buffer.clone() ),
        _ => None    
    }
}

fn draw_frame( app : &App, frame : &mut Frame, state : &mut WindowState )
{
    let palette = Palette::default();

    let areas = create_areas( app, frame.area() );

    render_container( &areas, frame, &palette );
    
    render_lines( app, &areas, frame, &palette );

    render_navigation( app, &areas, frame );

    render_sources( app, &areas, frame, state, &palette );

    render_search_input( app, &areas, frame, &palette, state );

    render_content( app, &areas, frame, state, &palette );

    render_content_generator( app, &areas, frame, &palette );

    render_message( app, &areas, frame, &palette );

    render_confirm( app, &areas, frame, state, &palette );

    render_edit_field( app, &areas, frame, state, &palette );
}