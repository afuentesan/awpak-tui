use ratatui::style::Color;

use super::palette::Palette;


pub struct TableColors
{
    pub header_bg : Color,
    pub header_fg : Color,

    pub row_bg_selected : Color,
    pub row_bg_idle : Color,

    pub row_fg_selected : Color,
    pub row_fg_idle : Color,

    pub cell_bg_selected : Color,
    pub cell_fg_selected : Color,

    pub bg : Color
}

impl TableColors
{
    pub fn default( palette : &Palette ) -> Self
    {
        Self 
        { 
            header_bg : palette.fg, 
            header_fg : palette.bg,

            row_bg_selected : palette.bg_sel,
            row_bg_idle : palette.bg,

            row_fg_selected : palette.fg_sel, 
            row_fg_idle : palette.fg, 

            cell_bg_selected : palette.fg_sel,
            cell_fg_selected : palette.bg_sel,

            bg : palette.bg
        }
    }

    pub fn default_selected( palette : &Palette ) -> Self
    {
        Self 
        { 
            header_bg : palette.fg_focus, 
            header_fg : palette.bg_focus,

            row_bg_selected : palette.bg_focus_sel,
            row_bg_idle : palette.bg_focus,

            row_fg_selected : palette.fg_focus_sel, 
            row_fg_idle : palette.fg_focus, 

            cell_bg_selected : palette.fg_focus_sel,
            cell_fg_selected : palette.bg_focus_sel,

            bg : palette.bg_focus
        }
    }

    // pub fn default() -> Self
    // {
    //     Self 
    //     { 
    //         header_bg : TableColors::HEADER_BG, 
    //         header_fg : TableColors::HEADER_FG,

    //         row_bg_selected : TableColors::ROW_BG_SELECTED,
    //         row_bg_idle : TableColors::ROW_BG_IDLE,

    //         row_fg_selected : TableColors::ROW_FG_SELECTED, 
    //         row_fg_idle : TableColors::ROW_FG_IDDLE, 

    //         cell_bg_selected : TableColors::CELL_BG_SELECTED,
    //         cell_fg_selected : TableColors::CELL_FG_SELECTED,

    //         bg : TableColors::BG
    //     }
    // }

    // pub fn default_selected() -> Self
    // {
    //     Self 
    //     { 
    //         header_bg : TableColors::HEADER_BG_CS, 
    //         header_fg : TableColors::HEADER_FG_CS,

    //         row_bg_selected : TableColors::ROW_BG_SELECTED_CS,
    //         row_bg_idle : TableColors::ROW_BG_IDLE_CS,

    //         row_fg_selected : TableColors::ROW_FG_SELECTED_CS, 
    //         row_fg_idle : TableColors::ROW_FG_IDDLE_CS, 

    //         cell_bg_selected : TableColors::CELL_BG_SELECTED_CS,
    //         cell_fg_selected : TableColors::CELL_FG_SELECTED_CS,

    //         bg : TableColors::BG_CS
    //     }
    // }
}