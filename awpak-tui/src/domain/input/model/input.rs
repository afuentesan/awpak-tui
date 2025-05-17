
#[derive(Debug, Default, Clone)]
pub struct Input
{
    pub text : String,
    pub cursor_position : u16
}

pub enum InputModifier
{
    Char( char ),
    Delete,
    Right,
    Left,
    LineUp,
    LineDown
}