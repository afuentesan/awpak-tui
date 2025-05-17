use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Stylize as _}, text::Line, DefaultTerminal, Frame};

use crate::infrastructure::ui::color::palette::Palette;


pub fn render_loading( terminal : &mut DefaultTerminal, last_buffer : Option<&Buffer>, palette : &Palette )
{
    let _ = terminal.draw( | frame |
        {
            if let Some( b ) = last_buffer
            {
                frame.buffer_mut().content = b.content.clone().into_iter()
                .map( 
                    | mut c |
                    {
                        if let Color::Rgb( r, g, b ) = c.bg
                        {
                            c.bg = Color::Rgb( r / 2, g / 2, b / 2 );
                        }

                        if let Color::Rgb( r, g, b ) = c.fg
                        {
                            c.fg = Color::Rgb( r / 2, g / 2, b / 2 );
                        }

                        c
                    }
                ).collect::<Vec<_>>();

                frame.buffer_mut().area = b.area;
            }
            
            draw_loading( frame, palette );
        } 
    );
}

pub fn hide_loading( terminal : &mut DefaultTerminal, last_buffer : Option<&Buffer> )
{
    let _ = terminal.draw( | frame |
        {
            if let Some( b ) = last_buffer
            {
                frame.buffer_mut().content = b.content.clone();
                frame.buffer_mut().area = b.area;
            }
        } 
    );
}

fn draw_loading( frame : &mut Frame, palette : &Palette )
{
    let area = frame.area();

    let area = Rect::new( 
        area.width - 15, 
        area.height - 2, 
        11, 
        1
    );

    frame.render_widget( Line::from( "Loading..." ).bg( palette.bg ).fg( palette.fg ), area );
}