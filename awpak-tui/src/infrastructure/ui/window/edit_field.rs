use ratatui::{layout::{Alignment, Rect}, style::{Style, Stylize}, text::Line, widgets::{Block, Clear}, Frame};

use crate::{domain::{app::model::app::App, field::model::field::Field, util::string_utils::str_len}, infrastructure::ui::{areas::areas::Areas, color::{line::LineColors, palette::Palette}, icons::line::LineIcons, text_area::text_area::text_area, util::ui_utils::{line_with_chars, render_widgets, BorderPosition}}};

use super::state::WindowState;

pub fn render_edit_field( app : &App, areas : &Areas, frame : &mut Frame, state : &mut WindowState, palette : &Palette )
{
    match app.field()
    {
        Some( f ) => render_field( f, areas, frame, state, palette ),
        None => {}
    }
}

fn render_field( field : &Field, areas : &Areas, frame : &mut Frame, state : &mut WindowState, palette : &Palette )
{
    state.edit_field.cursor_position = field.input.cursor_position;

    let ( text_area, ( width, height ) ) = text_area(
        field.input.text.as_str(), 
        &mut state.edit_field, 
        None, 
        palette.bg_focus_sel, 
        palette.fg_focus_sel, 
        "│"
    );

    let ( area_block, area_name, area_text, area_footer ) = areas_field( areas, height as usize, width as usize + 3, str_len( &field.name ) );

    let block = Block::bordered()
    .title( "Edit field".fg( palette.bg ) ).title_alignment( Alignment::Center )
    .style( Style::new().bg( palette.fg ).fg( palette.fg ) );

    let name = Line::from( field.name.as_str() ).bg( palette.bg_sel ).fg( palette.fg_sel );
    let footer = Line::from( "ESC to Cancel. Ctrl+s to Save" ).bg( palette.bg_sel ).fg( palette.fg_sel );

    frame.render_widget( Clear, area_block );
    frame.render_widget( block, area_block );
    frame.render_widget( name, area_name );
    frame.render_stateful_widget( text_area, area_text, &mut state.edit_field );
    frame.render_widget( footer, area_footer );

    render_field_lines( area_text, frame, palette );
}

fn render_field_lines( area : Rect, frame : &mut Frame, palette : &Palette )
{
    let horizontal = line_with_chars( LineIcons::horizontal_dotted_icons() )( LineColors::colors( palette ) );

    render_widgets( horizontal( area, vec![ BorderPosition::Top, BorderPosition::Bottom ] ), frame );
}

fn areas_field( areas : &Areas, num_lines : usize, max_line_width : usize, name_width : usize ) -> ( Rect, Rect, Rect, Rect )
{
    let block_width = u16::min(
        areas.full.width - 2,
        u16::max( 50, u16::max( max_line_width as u16, name_width as u16 ) + 2 )
    );

    let block_height = u16::min( num_lines  as u16 + 6, areas.full.height - 2 );

    let block_x = ( areas.full.width / 2 ) - ( block_width / 2 );
    let block_y = u16::min( ( areas.full.height / 2 ) - ( block_height / 2 ), areas.content.y + 2 );

    let body_width = block_width - 2;

    let body_x = block_x + 1;

    let name_y = block_y + 1;
    let text_y = name_y + 2;

    let text_height = u16::min( num_lines as u16, block_height - 6 );

    let footer_y = text_y + 1 + text_height;

    let area_block = Rect::new( block_x, block_y, block_width, block_height );
    let area_name = Rect::new( body_x, name_y, body_width, 1 );
    let area_text = Rect::new( body_x, text_y, body_width, text_height );
    let area_footer = Rect::new( body_x, footer_y, body_width, 1 );


    ( area_block, area_name, area_text, area_footer )
}