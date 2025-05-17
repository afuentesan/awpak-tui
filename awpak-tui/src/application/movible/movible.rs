use crate::domain::{app::{functions::change_content::reload_app_content, model::app::{App, AppContent, AppFocus, Confirm}}, error::Error, file::functions::file::copy_file_to_clipboard, movible::{functions::movible::{delete_movibles, paste_and_delete_movibles, paste_movibles, selected_movibles_from_table}, model::movible::{Movible, MovibleAction}}, result::result::AwpakResult};


pub fn copy( app : App ) -> AwpakResult<App>
{
    save_movible_action( 
        app, 
        | m | 
        {
            copy_to_clipboard( &m );

            MovibleAction::Copy( m ) 
        }
    )
}

fn copy_to_clipboard( movibles : &Vec<Movible> )
{
    if movibles.len() != 1
    {
        return;
    }

    match &movibles[ 0 ]
    {
        Movible::File( f ) => {
            // TODO: Esto no funciona, por lo menos en mi os.
            let _ = copy_file_to_clipboard( f );
        },
        _ => {}    
    };
}

pub fn cut( app : App ) -> AwpakResult<App>
{
    save_movible_action( app, | m | MovibleAction::Cut( m ) )
}

pub fn delete( app : App ) -> AwpakResult<App>
{
    save_movible_action( app, | m | MovibleAction::Delete( m ) )
    .write()
    .map( | a | a.change_focus( AppFocus::Confirm( Confirm::MovibleAction ) ) )
    .read()
}

pub fn confirm_movible_action( app : App ) -> AwpakResult<App>
{
    match app.movible_action()
    {
        MovibleAction::Delete( m ) => match delete_movibles( m )
        {
            Ok( _ ) => AwpakResult::new( reload_app_content( app ) ),
            Err( e ) => AwpakResult::new_err( reload_app_content( app ), e )
        },
        _ => AwpakResult::new_err( app, Error::Ignore )    
    }
}

fn save_movible_action( app : App, fn_movible_action : impl Fn( Vec<Movible> ) -> MovibleAction ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .zip_result( | a | selected_movibles( a ) )
    .write()
    .map( 
        | ( a, m ) | 
        ( a.change_movible_action( fn_movible_action( m.unwrap() ) ), Ok( vec![] ) ) )
    .finalize()
    .unzip( | a | a.0 )
    .read()
}

pub fn paste( app : App ) -> AwpakResult<App>
{
    match app.movible_action()
    {
        MovibleAction::Copy( m ) => match paste_movibles( m, app.content_generator() )
        {
            Ok( _ ) => AwpakResult::new( reload_app_content( app ) ),
            Err( e ) => AwpakResult::new_err( reload_app_content( app ), e )    
        },
        MovibleAction::Cut( m ) => match paste_and_delete_movibles( m, app.content_generator() )
        {
            Ok( _ ) => AwpakResult::new( reload_app_content( app ) ),
            Err( e ) => AwpakResult::new_err( reload_app_content( app ), e )    
        },
        MovibleAction::Delete( m ) => match delete_movibles( m )
        {
            Ok( _ ) => AwpakResult::new( reload_app_content( app ) ),
            Err( e ) => AwpakResult::new_err( reload_app_content( app ), e )    
        },
        MovibleAction::None => AwpakResult::new_err( app, Error::Ignore )
    }
}

fn selected_movibles( app : &App ) -> Result<Vec<Movible>, Error>
{
    match app.focus()
    {
        AppFocus::Content => selected_movibles_from_content( app ),
        _ => Err( Error::Ignore )    
    }
}

fn selected_movibles_from_content( app : &App ) -> Result<Vec<Movible>, Error>
{
    match app.content()
    {
        AppContent::Table( t ) => selected_movibles_from_table( t ),
        _ => Err( Error::Ignore )
    }
}

