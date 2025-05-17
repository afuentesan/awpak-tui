use crate::{application::{chat::chat::send_chat_request, confirm::confirm::confirm_action, field::field::save_field, navigation::navigate::{direct_navigation, history_back, history_next, parent_navigation}, search::search::regex_search_in_content}, domain::{app::model::app::{App, AppContent, AppFocus}, content_generator::model::content_generator::ContentGenerator, detail::model::detail::{Detail, DetailContent}, error::Error, result::result::AwpakResult, table::{functions::table_exec::exec_current_selected_row, model::row_output::RowOutput}}};

pub fn exec_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Back => history_back( app ),
        AppFocus::Up => parent_navigation( app ),
        AppFocus::Next => history_next( app ),
        AppFocus::Sources => exec_focus_on_sources( app ),
        AppFocus::Content => exec_focus_on_content( app ),
        AppFocus::Search => exec_focus_on_search( app ),
        AppFocus::Confirm( c ) => confirm_action( app, c ),
        AppFocus::Field => save_field( app )
    }
}

fn exec_focus_on_search( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( _ ) => send_chat_request( app ),
        _ => regex_search_in_content( app )
    }
}

fn exec_focus_on_content( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Table( t ) => 
        {
            let result = exec_current_selected_row( t );

            execution_output( app, result )
        }
        AppContent::Detail( d ) =>
        {
            let result = exec_on_content_detail( d );

            execution_output( app, result )
        }
        AppContent::Empty => AwpakResult::new_err( app, Error::Ignore ),
        AppContent::Chat { .. } =>
        {
            // TODO: Si estamos en el área de las preguntas pedir respuestas
            AwpakResult::new_err( app, Error::Ignore )
        }
    }
}

fn execution_output( app : App, output : Result<RowOutput, Error> ) -> AwpakResult<App>
{
    match output
    {
        Ok( o ) => match o
        {
            RowOutput::NewContent { generator, content } => exec_change_content( app, generator, content ),
            RowOutput::ShowMessage( m ) => AwpakResult::new( app.change_message( Some( m ) ) ),
            RowOutput::EditField( f ) =>
            {
                AwpakResult::new( app.change_field( Some( f ) ) )
                .write()
                .map( | a | a.change_focus( AppFocus::Field ) )
                .read()
            }
            RowOutput::Ignore => AwpakResult::new( app )
        },
        Err( e ) => AwpakResult::new_err( app, e )
    }
}

fn exec_on_content_detail( detail : &Detail ) -> Result<RowOutput, Error>
{
    match detail.content()
    {
        DetailContent::Table( t ) => exec_current_selected_row( t ),
        DetailContent::Empty => Err( Error::Ignore )
    }
}

fn exec_focus_on_sources( app : App ) -> AwpakResult<App>
{
    let output = exec_current_selected_row( app.sources() );

    execution_output( app, output )
}

fn exec_change_content( app : App, generator : ContentGenerator, content : AppContent ) -> AwpakResult<App>
{
    direct_navigation( app, generator, content ).write()
        .map( 
            | a | 
            {
                match a.focus()
                {
                    AppFocus::Sources => a.change_focus( AppFocus::Search ),
                    _ => a
                }
                
            }
        )
        .read()
}
