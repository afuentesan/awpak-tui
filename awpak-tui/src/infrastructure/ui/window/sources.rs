use ratatui::{layout::Rect, text::Line, Frame};

use crate::{domain::{app::model::app::{App, AppFocus}, selectable::{functions::selectable_utils::idx_current_selected_item_filter_hidden, model::selectable::Selectable}, table::model::rowable::Rowable}, infrastructure::ui::{areas::areas::Areas, color::{palette::Palette, table::TableColors}, table::from_table::{constraints_table_default, idx_visible_columns, render_cell_default, ui_from_table}}};

use super::state::WindowState;


pub fn render_sources( app : &App, areas : &Areas, frame : &mut Frame, window_state : &mut WindowState, palette : &Palette )
{
    let visible_columns = idx_visible_columns( app.sources() );

    let table_colors = match app.focus()
    {
        AppFocus::Sources => &TableColors::default_selected( palette ),
        _ => &TableColors::default( palette )
    };

    let ui_table = ui_from_table(
        app.sources(), 
        &visible_columns, 
        table_colors, 
        render_cell_default, 
        constraints_table_default
    );

    window_state.sources.select( idx_current_selected_item_filter_hidden( app.sources().rows() ) );

    frame.render_stateful_widget(ui_table, areas.sources, &mut window_state.sources );

    render_info_sources( app, areas, frame );
}

fn render_info_sources( app : &App, areas : &Areas, frame : &mut Frame ) -> Option<()>
{
    let info = app.sources().rows().iter()
    .find( | r | r.current_selected() )
    .map( | r | r.inner().cells()[ 0 ].inner().to_string() )?;

    let width = info.chars().count() as u16;

    let area = if width >= areas.info_sources.width
    {
        areas.info_sources
    } 
    else
    {
        Rect::new( 
            areas.info_sources.x + ( areas.info_sources.width / 2 ) - ( width / 2 ), 
            areas.info_sources.y, 
            width, 
            1 
        )
    };

    let line = Line::from( info );

    frame.render_widget( line, area );

    None
}