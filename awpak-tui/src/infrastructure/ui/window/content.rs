use std::collections::HashMap;

use awpak_tui_ai::domain::chat::chat::Chat;
use ratatui::{layout::{Constraint, Rect}, style::Stylize, text::Line, widgets::Cell, Frame};

use crate::{domain::{app::model::app::{App, AppContent, AppFocus}, detail::model::detail::{Detail, DetailContent}, selectable::{functions::selectable_utils::idx_current_selected_item_filter_hidden, model::{selectable::Selectable, selectable_item::SelectableItem}}, table::model::table::Table, util::string_utils::str_len}, infrastructure::{action::window::window_action::CursorDirection, ui::{areas::areas::{content_area, Areas}, color::{palette::Palette, table::TableColors}, table::from_table::{constraints_table_default, constraints_table_detail, idx_visible_columns, render_cell_default, render_cell_detail, ui_from_table}, util::ui_utils::str_lines_width_limited, window::graph::render_content_graph}}};

use super::{chat::render_content_chat, state::WindowState};

pub fn move_cursor_content( app : &App, window_state : &mut WindowState, direction : CursorDirection, full_area : Rect ) -> bool
{
    match app.content()
    {
        AppContent::Chat( c ) =>
        {
            let content_area = content_area( app, full_area );

            move_chat_cursor( c, window_state, direction, content_area )
        },
        _ => false
    }
}

fn move_chat_cursor( chat : &Chat, window_state : &mut WindowState, direction : CursorDirection, area : Rect ) -> bool
{
    match direction
    {
        CursorDirection::Up => move_chat_cursor_up( chat, window_state, area ),
        CursorDirection::Down => move_chat_cursor_down( chat, window_state, area ),
        CursorDirection::End => move_chat_cursor_to_end( chat, window_state, area )
    }
}

fn move_chat_cursor_up( chat : &Chat, window_state : &mut WindowState, area : Rect ) -> bool
{
    let lines = str_lines_width_limited( chat.response(), area.width as usize - 3 );

    let mut new_position = 0;

    if lines.len() > 1 && window_state.chat_response.cursor_position > 0
    {
        new_position = window_state.chat_response.cursor_position;

        let mut len = 0;

        for i in 0..lines.len()
        {
            let len_line = str_len( lines[ i ] ) as u16 + 1;

            if window_state.chat_response.cursor_position >= len && window_state.chat_response.cursor_position < ( len + len_line )
            {
                if i > 0
                {
                    new_position -= str_len( lines[ i - 1 ] ) as u16 + 1;
                }
                else
                {
                    new_position = 0;
                }
                
                break;
            }

            len += len_line;
        }
    }

    update_new_position( new_position, window_state )
}

fn move_chat_cursor_down( chat : &Chat, window_state : &mut WindowState, area : Rect ) -> bool
{
    let lines = str_lines_width_limited( chat.response(), area.width as usize - 3 );

    let mut new_position = 0;

    if lines.len() > 1
    {
        new_position = window_state.chat_response.cursor_position;

        let mut len = 0;

        for i in 0..( lines.len() - 1 )
        {
            let len_line = str_len( lines[ i ] ) as u16 + 1;

            if window_state.chat_response.cursor_position >= len && window_state.chat_response.cursor_position < ( len + len_line )
            {
                new_position += len_line;

                break;
            }

            len += len_line;
        }
    }

    update_new_position( new_position, window_state )
}

fn move_chat_cursor_to_end( chat : &Chat, window_state : &mut WindowState, area : Rect ) -> bool
{
    let lines = str_lines_width_limited( chat.response(), area.width as usize - 3 );

    let mut new_position = 0;

    if lines.len() > 1
    {
        let last = lines.len() - 1;

        new_position = lines.iter().enumerate().take_while( | i | i.0 < last )
        .fold(
            0, 
            | mut a, i |
            {
                a += str_len( i.1 ) as u16 + 1;

                a
            }
        );
    }    

    update_new_position( new_position, window_state )
}

fn update_new_position( new_position : u16, window_state : &mut WindowState ) -> bool
{
    if new_position != window_state.chat_response.cursor_position
    {
        window_state.chat_response.cursor_position = new_position;

        true
    }
    else
    {
        false    
    }
}

pub fn render_content_generator( app : &App, areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    frame.render_widget(
        Line::from( app.content_generator().to_string() ).fg( palette.fg ).bg( palette.bg ), 
        areas.content_generator
    );
}

pub fn render_content( 
    app : &App, 
    areas : &Areas, 
    frame : &mut Frame, 
    window_state : &mut WindowState,
    palette : &Palette
)
{
    match app.content()
    {
        AppContent::Table( t ) => 
            render_content_table( 
                app, 
                t, 
                areas.content, 
                areas.info_content, 
                frame, 
                window_state, 
                constraints_table_default, 
                render_cell_default,
                palette
            ),
        AppContent::Detail( d ) => render_content_detail( d, app, areas, frame, window_state, palette ),
        AppContent::Empty => {},
        AppContent::Chat( _ ) => render_content_chat( app, areas, frame, window_state, palette ),
        AppContent::Graph( _ ) => render_content_graph( app, areas, frame, window_state, palette )
    };
}

fn render_content_detail( 
    detail : &Detail, 
    app : &App, 
    areas : &Areas, 
    frame : &mut Frame, 
    window_state : &mut WindowState,
    palette : &Palette
)
{
    match detail.content()
    {
        DetailContent::Table( t ) => 
            render_content_table( 
                app, 
                t, 
                areas.content, 
                areas.info_content, 
                frame, 
                window_state, 
                constraints_table_detail, 
                render_cell_detail,
                palette
            ),
        DetailContent::Empty => {}
    }
}

fn render_content_table( 
    app : &App, 
    table : &Table, 
    area : Rect, 
    area_info : Rect, 
    frame : &mut Frame, 
    window_state : &mut WindowState,
    fn_constraints : impl Fn( &HashMap<usize, bool>, &Table ) -> Vec<Constraint>,
    fn_render_cell : impl for<'a> Fn( usize, &'a SelectableItem<crate::domain::table::model::cell::Cell>, &'a TableColors, bool ) -> Cell<'a>,
    palette : &Palette
)
{
    let visible_columns = idx_visible_columns( table );

    let table_colors = match app.focus()
    {
        AppFocus::Content => &TableColors::default_selected( palette ),
        _ => &TableColors::default( palette )
    };

    let ui_table = ui_from_table(
        table, 
        &visible_columns, 
        table_colors, 
        fn_render_cell, 
        fn_constraints
    );
    
    window_state.content.select( idx_current_selected_item_filter_hidden( table.rows() ) );

    frame.render_stateful_widget(ui_table, area, &mut window_state.content );

    render_info_table( table, area_info, frame );
}

fn render_info_table( table : &Table, area : Rect, frame : &mut Frame )
{
    let selected = table.rows().iter().enumerate()
    .find( | r | r.1.current_selected() )
    .map( | r | ( r.0 + 1 ).to_string() ).unwrap_or( "-".to_string() );

    let selected = format!( "{} of {}", selected, table.rows().len() );

    let width = selected.chars().count() as u16;

    let area = Rect::new( area.x + ( area.width / 2 ) - ( width / 2 ), area.y, width, 1 );

    let line : Line = selected.into();

    frame.render_widget( line, area );
}