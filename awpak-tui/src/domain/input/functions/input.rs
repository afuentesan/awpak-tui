use unicode_segmentation::UnicodeSegmentation;

use crate::domain::{input::model::input::{Input, InputModifier}, result::result::AwpakResult, util::string_utils::{str_len, StrLine}};

pub fn append_text_to_input( input : Input, text : &str ) -> Input
{
    let modifiers = text.chars().map(
        | c | InputModifier::Char( c )
    )
    .collect::<Vec<_>>();

    apply_input_modifiers( input, &modifiers )
}

pub fn apply_input_modifiers( input : Input, modifiers : &Vec<InputModifier> ) -> Input
{
    AwpakResult::new( input )
    .write()
    .map_iter(
        modifiers.iter(), 
        apply_input_modifier
    ).own()
}

fn apply_input_modifier( input : Input, modifier : &InputModifier ) -> Input
{
    match modifier
    {
        InputModifier::Char( c ) => apply_input_modifier_add_char( input, *c ),
        InputModifier::Delete => apply_input_modifier_delete_char( input ),
        InputModifier::Left => apply_input_modifier_left( input ),
        InputModifier::Right => apply_input_modifier_right( input ),
        InputModifier::LineUp => apply_input_modifier_line_up( input ),
        InputModifier::LineDown => apply_input_modifier_line_down( input )
    }
}

fn apply_input_modifier_line_up( mut input : Input ) -> Input
{
    let positions = StrLine::iter( &input.text, Some( input.cursor_position ) )
    .fold(
        ( None, None ), 
        | mut a, l |
        {
            if a.1.is_some()
            {
                return a;
            }

            if a.0.is_none()
            {
                a.0 = Some( l );

                return a;
            }

            if input.cursor_position >= l.from && input.cursor_position <= ( l.from + l.len )
            {
                a.1 = Some( l );

                return a;
            }

            a.0 = Some( l );

            a
        }
    );

    if positions.0.is_none() || positions.1.is_none()
    {
        return input;
    }
    
    let previous = positions.0.unwrap();
    let current = positions.1.unwrap();

    let col_pos = input.cursor_position - current.from;

    if previous.len >= col_pos
    {
        input.cursor_position = previous.from + col_pos;
    }
    else
    {
        input.cursor_position = previous.from + previous.len;    
    }

    input
}

fn apply_input_modifier_line_down( mut input : Input ) -> Input
{
    let positions = StrLine::iter( &input.text, None )
    .fold(
        ( None, None ), 
        | mut a, l |
        {
            if a.1.is_some()
            {
                return a;
            }

            if input.cursor_position >= l.from && input.cursor_position <= ( l.from + l.len )
            {
                a.0 = Some( l );

                return a;
            }

            if a.0.is_some() && a.1.is_none()
            {
                a.1 = Some( l );
            }

            a
        }
    );

    if positions.0.is_none() || positions.1.is_none()
    {
        return input;
    }
    
    let current = positions.0.unwrap();
    let next = positions.1.unwrap();

    let col_pos = input.cursor_position - current.from;

    if next.len >= col_pos
    {
        input.cursor_position = next.from + col_pos;
    }
    else
    {
        input.cursor_position = next.from + next.len;    
    }

    input
}

fn apply_input_modifier_add_char( mut input : Input, c : char ) -> Input
{
    let idx = usize::from( input.cursor_position );
    let len = str_len( &input.text );

    input.text = match ( idx, len )
    {
        ( idx, len ) if idx == len => format!( r#"{}{}"#, input.text, c ),
        _ => input.text.graphemes( true )
        .enumerate()
        .map( | ( i, c_inner ) | if i == idx { format!( r#"{}{}"#, c, c_inner ) } else { c_inner.to_string() } )
        .fold( "".to_string(), | a, s | format!( r#"{}{}"#, a, s ) )
    };

    input.cursor_position += 1;

    input
}

fn apply_input_modifier_delete_char( mut input : Input ) -> Input
{
    if input.cursor_position == 0
    {
        return input
    }
    
    input.cursor_position -= 1;
        
    input.text = input.text.graphemes( true )
        .enumerate()
        .filter( | ( i, _ ) | *i != input.cursor_position as usize )
        .fold( "".to_string(), | a, ( _, c ) | format!( "{}{}", a, c ) );

    input
}

fn apply_input_modifier_left( mut input : Input ) -> Input
{
    if input.cursor_position > 0
    {
        input.cursor_position -= 1;
    }

    input
}

fn apply_input_modifier_right( mut input : Input ) -> Input
{
    let max_cursor_position = str_len( &input.text ) as u16;

    input.cursor_position = u16::min( input.cursor_position + 1, max_cursor_position );

    input
}