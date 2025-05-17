use ratatui::style::Color;

pub const BG_DEF : Color = Color::from_u32( 0x1e1e1e );
pub const FG_DEF : Color = Color::from_u32( 0xfafafa );

pub const BG_SEL_DEF : Color = Color::from_u32( 0x1e1e1e );
pub const FG_SEL_DEF : Color = Color::from_u32( 0xfafafa );

pub const BG_FOCUS_DEF : Color = Color::from_u32( 0x2d2d30 );
pub const FG_FOCUS_DEF : Color = Color::from_u32( 0xfcfcfc );

pub const BG_FOCUS_SEL_DEF : Color = Color::from_u32( 0x3e3e42 );
pub const FG_FOCUS_SEL_DEF : Color = Color::from_u32( 0xfefefe );

pub struct Palette
{
    pub bg : Color,
    pub fg : Color,

    pub bg_sel : Color,
    pub fg_sel : Color,

    pub bg_focus : Color,
    pub fg_focus : Color,

    pub bg_focus_sel : Color,
    pub fg_focus_sel : Color
}

impl Default for Palette
{
    fn default() -> Self 
    {
        Self 
        { 
            bg : BG_DEF, 
            fg : FG_DEF, 
            
            bg_sel : BG_SEL_DEF, 
            fg_sel : FG_SEL_DEF, 
            
            bg_focus : BG_FOCUS_DEF, 
            fg_focus : FG_FOCUS_DEF, 
            
            bg_focus_sel : BG_FOCUS_SEL_DEF, 
            fg_focus_sel : FG_FOCUS_SEL_DEF 
        }
    }
}