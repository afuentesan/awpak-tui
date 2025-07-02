use ratatui::{layout::Rect, style::{Color, Stylize}, text::{Line, Span}, widgets::Widget, Frame};

use crate::domain::util::string_utils::{divide_str, str_len};

#[derive(Debug, Clone, Copy)]
pub enum BorderPosition
{
    Top,
    Right,
    Bottom,
    Left
}

pub fn line_with_chars( 
    ( first, middle, last ) : ( char, char, char ) 
) -> impl Fn( ( ( Color, Color ), ( Color, Color ), ( Color, Color ) ) ) -> Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>
{
    move | ( first_color, middle_color, last_color ) |
    {
        let first = first.to_string().bg( first_color.0 ).fg( first_color.1 );
        let middle = middle.to_string().bg( middle_color.0 ).fg( middle_color.1 );
        let last = last.to_string().bg( last_color.0 ).fg( last_color.1 );

        let b = border( ( first, middle, last ) );

        Box::new(
            move | area, border_positions |
            {
                let ba = b( area );

                border_positions.iter()
                    .map( | p | ba( *p ) ).flatten().collect()
            }
        )
    }
}

fn border( 
    ( first, middle, last ) : ( Span<'static>, Span<'static>, Span<'static> )
) -> impl Fn( Rect ) -> Box<dyn Fn( BorderPosition ) -> Vec<( Rect, Line<'static> )>>
{
    move | area |
    {
        let first = first.clone();
        let middle = middle.clone();
        let last = last.clone();

        Box::new(
            move | border_position |
            {
                let mut points = positions( area, border_position );

                let first_point = points.remove( 0 );
                let last_point = points.remove( points.len() - 1 );

                let first_char = area_and_line( first_point, first.clone() );
                let last_char = area_and_line( last_point, last.clone() );

                let mut ret = points.iter().fold( 
                    vec![ first_char ], 
                    | mut a, p | 
                    {
                        a.push( area_and_line( *p, middle.clone() ) );
                        a
                    }
                );

                ret.push( last_char );

                ret
            }
        )
    }
}

fn area_and_line( position : ( u16, u16 ), span : Span ) -> ( Rect, Line )
{
    (
        Rect::new( position.0, position.1, 1, 1 ),
        Line::from( span )
    )
}

fn positions( area : Rect, border_position : BorderPosition ) -> Vec<( u16, u16 )>
{
    let start_end = match border_position
    {
        BorderPosition::Top => ( ( area.x - 1, area.y - 1 ), ( area.x + area.width, area.y - 1 ) ),
        BorderPosition::Right => ( ( area.x + area.width, area.y - 1 ), ( area.x + area.width, area.y + area.height ) ),
        BorderPosition::Bottom => ( ( area.x - 1, area.y + area.height ), ( area.x + area.width, area.y + area.height ) ),
        BorderPosition::Left => ( ( area.x - 1, area.y - 1 ), ( area.x - 1, area.y + area.height ) )
    };

    ( start_end.0.0..=start_end.1.0 ).into_iter()
    .map( | x | 
        {
            ( start_end.0.1..=start_end.1.1 ).into_iter()
            .map( | y | ( x, y ) ).collect::<Vec<_>>()
        }
    )
    .flatten()
    .collect::<Vec<_>>()
}

pub fn render_widgets( widgets : Vec<( Rect, impl Widget )>, frame : &mut Frame )
{
    widgets.into_iter().for_each( | ( r, w ) | frame.render_widget( w, r ) );
}

pub fn relative_area( area : Rect, x : i32, y : i32, w : i32, h : i32 ) -> Rect
{
    Rect::new( 
        sum_u_i(area.x, x ), 
        sum_u_i(area.y, y ), 
        sum_u_i(area.width, w ), 
        sum_u_i(area.height, h ), 
    )
}

fn sum_u_i( u : u16, i : i32 ) -> u16
{
    ( u as i32 + i ) as u16
}

pub fn merge_areas( a1 : Rect, a2 : Rect ) -> Rect
{
    let min_x = u16::min( a1.x, a2.x );
    let max_x = u16::max( a1.x + a1.width, a2.x + a2.width );

    let min_y = u16::min( a1.y, a2.y );
    let max_y = u16::max( a1.y + a1.height, a2.y + a2.height );

    Rect { x : min_x, y : min_y, width : max_x - min_x, height : max_y - min_y }
}

pub fn str_lines_width_limited( text : &str, max_width : usize ) -> Vec<&str>
{
    text.split( "\n" ).flat_map(
        | l | str_lines_from_line( l, max_width )
    )
    .collect()
}

pub fn str_lines_from_line( mut line : &str, max_width : usize ) -> Vec<&str>
{
    let mut lines : Vec<&str> = vec![];

    while str_len( line ) > max_width
    {
        let ( first, remain ) = divide_str( line, max_width );

        lines.push( first );

        line = remain;
    }

    lines.push( line );

    lines
}