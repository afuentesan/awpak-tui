use ratatui::{layout::{Alignment, Rect}, style::{Color, Style, Stylize as _}, text::{Line, Span}, widgets::{Paragraph, StatefulWidget, Widget}, Frame};

use crate::{domain::util::string_utils::{divide_str, str_len}, infrastructure::ui::util::ui_utils::str_lines_width_limited};


#[derive(Default)]
pub struct ScrollState
{
    pub scroll : ( u16, u16 ),
    pub cursor_position : u16
}

pub struct TextAreaWidget<'a>
{
    lines : Vec<Line<'a>>,
    line_col : ( u16, u16 ),
    bg : Color,
    fg : Color
}

impl TextAreaWidget<'_>
{
    fn scroll( &self, state : &ScrollState, area : Rect ) -> ( u16, u16 )
    {
        let visible_lines = ( state.scroll.0, state.scroll.0 + area.height - 1 );

        let y = if self.line_col.0 >= visible_lines.0 && self.line_col.0 < visible_lines.1
        {
            state.scroll.0
        }
        else if self.line_col.0 < visible_lines.0
        {
            state.scroll.0 - ( visible_lines.0 - self.line_col.0 )
        }
        else
        {
            state.scroll.0 + ( self.line_col.0 - visible_lines.1 )
        };

        let visible_cols = ( state.scroll.1, state.scroll.1 + area.width - 3 );

        let x = if self.line_col.1 >= visible_cols.0 && self.line_col.1 < visible_cols.1
        {
            state.scroll.1
        }
        else if self.line_col.1 < visible_cols.0
        {
            state.scroll.1 - ( visible_cols.0 - self.line_col.1 )
        }
        else
        {
            state.scroll.1 + ( self.line_col.1 - visible_cols.1 )
        };

        ( y, x )
    }
}

pub struct TextLinesInfo<'a>
{
    lines : Vec<Line<'a>>,
    max_width : u16,
    cursor_position : ( u16, u16 )
}

impl StatefulWidget for TextAreaWidget<'_>
{
    type State = ScrollState;

    fn render( self, area : Rect, buf : &mut ratatui::prelude::Buffer, state : &mut Self::State )
    {
        let scroll = self.scroll( state, area );

        state.scroll = scroll;

        let paragraph = Paragraph::new( self.lines )
        // .block( Block::bordered().borders( Borders::TOP | Borders::BOTTOM ) )
        .style( Style::new().bg( self.bg ).fg( self.fg ) )
        .alignment(Alignment::Left)
        // .wrap( Wrap { trim : false })
        .scroll( scroll );

        paragraph.render( area, buf );
    }
}

fn text_lines<'a>( 
    text : Vec<&'a str>, 
    idx_cursor : u16,
    bg : Color,
    fg : Color,
    prepend : &'a str
) -> TextLinesInfo<'a>
{
    let info = text.iter()
    .fold( 
        ( 0, vec![], 0, 0, None ), 
        | mut a, s | 
        {
            let len = str_len( s );

            if len > a.0 { a.0 = len; }

            let ( text, cursor_position ) = text_line( s, len, a.2, idx_cursor as usize, a.3, bg, fg, prepend );

            if let Some( p ) = cursor_position
            {
                a.4 = Some( p );
            }

            a.1.push( Line::from( text ).bg( bg ).fg( fg ) );

            a.2 += len + 1;

            a.3 += 1 as u16;

            a
        }
    );

    TextLinesInfo
    {
        max_width : info.0 as u16,
        lines : info.1,
        cursor_position : info.4.unwrap_or( ( 0, 0 ) )
    }
}

fn text_line<'a>( 
    text : &'a str, 
    len_str : usize, 
    len_from : usize, 
    cursor_position : usize, 
    line_number : u16,
    bg : Color,
    fg : Color,
    prepend : &'a str
) -> ( Vec<Span<'a>>, Option<(u16, u16)> )
{
    // let first = Span::from( "│" );

    let first = Span::from( prepend );

    if cursor_position >= len_from && cursor_position < ( len_from + len_str )
    {
        let splited = divide_str( text, cursor_position - len_from );

        let ( cursor, rest ) = divide_str( splited.1, 1 );

        let position = ( line_number, ( cursor_position - len_from ) as u16 );

        ( vec![ first, Span::from( splited.0 ), Span::from( cursor ).bg( fg ).fg( bg ), Span::from( rest ) ], Some( position ) )
    }
    else if cursor_position == ( len_from + len_str )
    {
        let position = ( line_number, ( cursor_position - len_from ) as u16 );

        ( vec![ first, Span::from( text ), Span::from( " " ).bg( fg ).fg( bg ) ], Some( position ) )
    }
    else
    {
        ( vec![ first, Span::from( text ) ], None )
    }
}

pub fn text_area<'a>( 
    text : &'a str, 
    state : &mut ScrollState, 
    limite_width : Option<u16>,
    bg : Color,
    fg : Color,
    prepend : &'a str
) -> ( TextAreaWidget<'a>, ( u16, u16 ) )
{
    let TextLinesInfo {
        lines,
        cursor_position,
        max_width
    } = text_lines( 
        if let Some( limite_width ) = limite_width
        {
            str_lines_width_limited( text, limite_width as usize - 3 )
        }
        else
        {
            text.split( "\n" ).collect()
        }, 
        state.cursor_position,
        bg,
        fg,
        prepend
    );

    let size = ( max_width, lines.len() as u16 );

    let text_area = TextAreaWidget 
    { 
        lines, 
        line_col : cursor_position,
        bg,
        fg
    };

    ( text_area, size )
}

pub fn render_text_area<'a>( 
    text : &'a str, 
    state : &mut ScrollState, 
    area : Rect, 
    frame : &mut Frame,
    limite_width : bool,
    bg : Color,
    fg : Color,
    prepend : &'a str
)
{
    let ( text_area, _ ) = text_area( 
        text, 
        state, 
        if limite_width { Some( area.width ) } else { None }, 
        bg, 
        fg, 
        prepend
    );

    // let TextLinesInfo {
    //     lines,
    //     cursor_position,
    //     ..
    // } = text_lines( 
    //     if limite_width
    //     {
    //         str_lines_width_limited( text, area.width as usize - 3 )
    //     }
    //     else
    //     {
    //         text.split( "\n" ).collect()
    //     }, 
    //     state.cursor_position,
    //     bg,
    //     fg,
    //     prepend
    // );

    // let text_area = TextAreaWidget 
    // { 
    //     lines, 
    //     line_col : cursor_position,
    //     bg,
    //     fg
    // };

    frame.render_stateful_widget( text_area, area, state );
}