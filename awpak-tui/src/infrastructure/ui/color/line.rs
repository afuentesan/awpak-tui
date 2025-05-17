use ratatui::style::Color;

use super::palette::Palette;


pub struct LineColors;

impl LineColors
{
    pub fn colors( palette : &Palette ) -> ( ( Color, Color ), ( Color, Color ), ( Color, Color ) )
    {
        (
            ( palette.bg, palette.fg ),
            ( palette.bg, palette.fg ),
            ( palette.bg, palette.fg )
        )
    }

    pub fn colors_selected( palette : &Palette ) -> ( ( Color, Color ), ( Color, Color ), ( Color, Color ) )
    {
        ( 
            ( palette.bg_sel, palette.fg_sel ),
            ( palette.bg_sel, palette.fg_sel ),
            ( palette.bg_sel, palette.fg_sel )
        )
    }
}