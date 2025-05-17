use ratatui::{layout::{Alignment, Rect}, style::Style, text::Line, widgets::{Block, Clear, Paragraph, Wrap}, Frame};

use crate::{domain::util::string_utils::split_str_by_len, infrastructure::ui::{areas::areas::Areas, color::palette::Palette}};


pub fn render_modal( title : &str, message : &str, areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    let width = calc_max_width( areas );

    let str_lines = split_str_by_len( message, ( width - 2 ).into() );

    let lines = str_lines
    .iter()
    .flat_map( | s | s.as_str().split( "\n" ).map( | l | Line::from( l ) ) )
    .collect::<Vec<_>>();

    let total_height = lines.len() as u16 + 2;

    let paragraph = Paragraph::new(lines)
    .block(Block::bordered().title( title ).title_alignment( Alignment::Center ) )
    .style(Style::new().bg( palette.bg_focus ).fg( palette.fg_focus ) )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    let area = area_message( areas, width, total_height );

    frame.render_widget( Clear, area );
    frame.render_widget( paragraph, area );
}

fn area_message( areas : &Areas, width : u16, height : u16 ) -> Rect
{
    let height = u16::min( areas.full.height - 4 , height );

    let x = ( areas.full.width / 2 ) - ( width / 2 );
    let y = u16::min( areas.content.y - 2, ( areas.full.height / 2 ) - ( height / 2 ) );

    Rect::new( x, y, width, height )
}

fn calc_max_width( areas : &Areas ) -> u16
{
    u16::min( areas.full.width - 2, 50 )
}