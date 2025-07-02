use ratatui::{layout::Rect, style::Color, text::Line, Frame};

use crate::{domain::{app::model::app::{App, AppContent, AppFocus}, graph::graph_functions::is_graph_request_empty}, infrastructure::ui::{areas::areas::Areas, color::palette::Palette, text_area::text_area::render_text_area_from_vec_of_str}};

use super::state::WindowState;


pub fn render_content_graph( app : &App, areas : &Areas, frame : &mut Frame, window_state : &mut WindowState, palette : &Palette )
{
    match app.content()
    {
        AppContent::Graph( c ) => 
        {
            let colors = colors( app, palette );

            render_text_area_from_vec_of_str( 
                &c.response, 
                &mut window_state.chat_response, 
                areas.content, 
                frame, 
                true,
                colors.0,
                colors.1,
                " "
            );

            render_info( app, areas, frame );
        },
        _ => {}    
    }
}

fn render_info( app : &App, areas : &Areas, frame : &mut Frame )
{
    if is_graph_request_empty( &app.graph_content().unwrap().request )
    {
        return;
    }

    let area = areas.info_content;

    let loading_text = "Loading...".to_string();

    let width = loading_text.chars().count() as u16;

    let area = Rect::new( area.x + ( area.width / 2 ) - ( width / 2 ), area.y, width, 1 );

    let line : Line = loading_text.into();

    frame.render_widget( line, area );
}

fn colors( app : &App, palette : &Palette ) -> ( Color, Color )
{
    match app.focus()
    {
        AppFocus::Content => ( palette.bg_focus, palette.fg_focus ),
        _ => ( palette.bg, palette.fg )    
    }
}