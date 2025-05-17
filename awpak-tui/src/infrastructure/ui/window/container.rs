use ratatui::{style::Stylize as _, widgets::{Block, BorderType}, Frame};

use crate::infrastructure::ui::{areas::areas::Areas, color::palette::Palette};

pub fn render_container( areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    let block = Block::bordered()
        .bg( palette.bg )
        .fg( palette.fg )
        .border_type( BorderType::Plain );

    frame.render_widget( block, areas.full );
}