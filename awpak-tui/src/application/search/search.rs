use crate::{application::app::change_focus::{next_focus, previous_focus}, domain::{app::model::app::{App, AppContent}, detail::model::detail::{Detail, DetailContent}, error::Error, input::{functions::input::{append_text_to_input, apply_input_modifiers}, model::input::{Input, InputModifier}}, result::{functions::result_utils::bool_err, result::AwpakResult}, table::{functions::table_search::show_hide_rows_by_regex, model::table::Table}}};

pub fn clear_search( app : App ) -> AwpakResult<App>
{
    regex_search_in_content( app.change_content_search( Input::default() ) )
}

pub fn paste_on_search( text : String ) -> Box<dyn Fn( App ) -> AwpakResult<App>>
{
    Box::new( move | app |
        {
            let text = if let AppContent::Chat( _ ) = app.content()
            {
                &text
            }
            else
            {
                &text.replace( "\n", " " )
            };

            AwpakResult::new( app )
            .validate()
            .map_result( | _ | bool_err( text == "", Error::Ignore ) )
            .write()
            .map(
                | a |
                {
                    let ( a, input ) = a.own_content_search();

                    let input = append_text_to_input( input, text );

                    a.change_content_search( input )
                }
            )
            .read()
        }
    )
}

pub fn search_line_up( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( _ ) => search_navigation( app, InputModifier::LineUp, previous_focus ),
        _ => previous_focus( app )    
    }
}

pub fn search_line_down( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( _ ) => search_navigation( app, InputModifier::LineDown, next_focus ),
        _ => next_focus( app )
    }
}

fn search_navigation( 
    app : App, 
    navigation : InputModifier,
    fn_default : impl Fn( App ) -> AwpakResult<App> 
) -> AwpakResult<App>
{
    let ( app, input ) = app.own_content_search();

    match search_move_line( input, navigation ).collect()
    {
        ( input, None ) => AwpakResult::new( app.change_content_search( input ) ),
        ( input, Some( _ ) ) => fn_default( app.change_content_search( input ) )
    }
}

fn search_move_line( input : Input, modifier : InputModifier ) -> AwpakResult<Input>
{
    let previous_cursor_position = input.cursor_position;

    let input = apply_input_modifiers( input, &vec![ modifier ] );

    if previous_cursor_position == input.cursor_position
    {
        AwpakResult::new_err( input, Error::Ignore )
    }
    else
    {
        AwpakResult::new( input )   
    }
}

pub fn change_regex_content_search( 
    modifiers : Vec<InputModifier>
) -> impl Fn( App ) -> AwpakResult<App>
{
    move | app | 
    {
        let ( app, input ) = app.own_content_search();

        let old_position = input.cursor_position;

        let input = apply_input_modifiers( input, &modifiers );

        let app = app.change_content_search( input );

        if modifiers_change_text( &modifiers, old_position )
        {
            regex_search_in_content( app )
        }
        else
        {
            AwpakResult::new( app )
        }
    }
}

fn modifiers_change_text( modifiers : &Vec<InputModifier>, old_position : u16 ) -> bool
{
    let cursor_in_first = old_position == 0;

    modifiers.iter().find(
        | m | match m
        {
            InputModifier::Left | InputModifier::Right => false,
            InputModifier::Delete if cursor_in_first => false,
            _ => true    
        }

    ).is_some()
}

pub fn regex_search_in_content( app : App ) -> AwpakResult<App>
{
    let ( app, content ) = app.own_content();

    AwpakResult::new(
        match content
        {
            AppContent::Table( t ) => regex_search_in_table( app, t ),
            AppContent::Detail( d ) => regex_search_in_detail( app , d ),
            AppContent::Empty => app,
            AppContent::Chat( c ) =>
            {
                app.change_content( AppContent::Chat( c ) )
            },
            AppContent::Graph( g ) =>
            {
                app.change_content( AppContent::Graph( g ) )
            }
        }
    )
}

fn regex_search_in_detail( app : App, detail : Box<Detail> ) -> App
{
    let ( detail, content ) = detail.own_content();

    let search = app.clone_content_search_text();

    let detail = detail.change_content(
        match content
        {
            DetailContent::Table( t ) => DetailContent::Table( show_hide_rows_by_regex( t, search.as_str() ) ),
            DetailContent::Empty => content
                
        }
    );

    app.change_content( AppContent::Detail( Box::new( detail ) ) )
}

fn regex_search_in_table( app : App, table : Table ) -> App
{
    let search = app.clone_content_search_text();

    app.change_content(
        AppContent::Table( show_hide_rows_by_regex( table, search.as_str() ) )
    )
}