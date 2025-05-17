use ratatui::{layout::{Constraint, Layout}, style::Stylize, text::Line, Frame};

use crate::{domain::{app::model::app::{App, AppContent, AppFocus}, util::string_utils::str_len}, infrastructure::ui::{areas::areas::Areas, color::palette::Palette, text_area::text_area::text_area}};

use super::state::WindowState;


pub fn render_search_input( 
    app : &App, 
    areas : &Areas, 
    frame : &mut Frame,
    palette : &Palette,
    state : &mut WindowState
)
{
    state.search.cursor_position = app.content_search().cursor_position;

    let ( bg, fg, bg_text, fg_text ) = match app.focus()
    {
        AppFocus::Search => ( palette.bg, palette.fg, palette.bg_focus_sel, palette.fg_focus_sel ),
        _ => ( palette.bg, palette.fg, palette.bg, palette.fg )
    };

    let ( text, prepend ) = match app.content()
    {
        AppContent::Chat( _ ) => ( "Prompt:", "│" ),
        _ => ( "Search:", " " )
    };

    let ( text_area, _ ) = text_area(
        app.content_search().text.as_str(), 
        &mut state.search, 
        None, 
        bg_text, 
        fg_text, 
        prepend
    );

    let layout = Layout::horizontal( [ Constraint::Length( str_len( text ) as u16 + 1 ), Constraint::Fill( 0 ) ] );

    let [ search_text, search_area ] = layout.areas( areas.search );

    frame.render_widget(
        Line::from( text.bold() ).fg( fg ).bg( bg ), 
        search_text 
    );

    frame.render_stateful_widget( text_area, search_area, &mut state.search );
}