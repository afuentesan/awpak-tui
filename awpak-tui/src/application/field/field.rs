use crate::domain::{app::model::app::{App, AppContent, AppFocus}, detail::functions::detail::{persist_detail_field, update_detail_field}, error::Error, field::model::{edit_field_output::EditFieldOutput, field::Field}, input::{functions::input::{append_text_to_input, apply_input_modifiers}, model::input::{Input, InputModifier}}, result::{functions::result_utils::bool_err, result::AwpakResult}, table::functions::table_save::{save_table_field, update_content_table_selected_field}};

pub fn clear_field( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.field().is_none(), Error::Ignore ) )
    .write()
    .map(
        | a |
        {
            let ( a, f ) = a.own_field();

            let field = f.unwrap();

            a.change_field( Some( field.change_input( Input::default() ) ) )
        }
    )
    .read()
}

pub fn paste_on_field( text : String ) -> Box<dyn Fn( App ) -> AwpakResult<App>>
{
    Box::new( move | app |
        {
            let text = text.as_str();

            AwpakResult::new( app )
            .validate()
            .map_result( | _ | bool_err( text == "", Error::Ignore ) )
            .map_result( | a | bool_err( a.field().is_none(), Error::Ignore ) )
            .write()
            .map(
            | a |
                {
                    let ( a, f ) = a.own_field();

                    let f = f.unwrap();

                    let ( f, input ) = f.own_input();

                    let input = append_text_to_input( input, text );
                    
                    a.change_field( Some( f.change_input( input ) ) )
                }
            )
            .read()
        }
    )
}

pub fn save_field( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.field().is_none(), Error::Ignore ) )
    .zip_result( | a | persist_field( a ) )
    .write()
    .map( | a | update_field( a ) )
    .map( | ( a, v ) | ( a.change_focus( AppFocus::Content ), v ) )
    .map(
        | ( a, v ) |
        {
            let ( a, _ ) = a.own_field();

            ( a, v )
        }
    )
    .finalize()
    .unzip( | ( a, _ ) | a )
    .read()
}

fn persist_field( app : &App ) -> Result<EditFieldOutput, Error>
{
    match app.content()
    {
        AppContent::Table( t ) => save_table_field( t, app.field().unwrap() ),
        AppContent::Detail( d ) => persist_detail_field( d, app.field().unwrap() ),
        AppContent::Chat( _ ) => Err( Error::Ignore ),
        AppContent::Graph( _ ) => Err( Error::Ignore ),
        AppContent::Empty => Err( Error::Ignore )
    }
}

fn update_field( ( app, value ) : ( App, Result<EditFieldOutput, Error> ) ) -> ( App, Result<EditFieldOutput, Error> )
{
    let ( app, content ) = app.own_content();

    match content
    {
        AppContent::Table( t ) => ( update_content_table_selected_field( app, t, value.as_ref().unwrap() ), value ),
        AppContent::Detail( d ) => ( update_detail_field( app, *d, value.as_ref().unwrap() ), value ),
        AppContent::Chat( _ ) => unreachable!(),
        AppContent::Graph( _ ) => unreachable!(),
        AppContent::Empty => unreachable!()
    }
}

pub fn change_edit_field( 
    modifiers : Vec<InputModifier>
) -> impl Fn( App ) -> AwpakResult<App>
{
    move | app | 
    {
        AwpakResult::new( app )
        .validate()
        .map_result( | a | bool_err( a.field().is_none(), Error::Ignore ) )
        .write()
        .map(
            | a |
            {
                let ( a, f ) = a.own_field();

                let f = f.unwrap();

                let f = apply_modifiers_to_field( f, &modifiers );

                a.change_field( Some( f ) )
            }
        )
        .read()
    }
}

fn apply_modifiers_to_field( field : Field, modifiers : &Vec<InputModifier> ) -> Field
{
    let ( field, input ) = field.own_input();

    let input = apply_input_modifiers( input, modifiers );

    field.change_input( input )
}