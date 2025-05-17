use std::{sync::mpsc::{self, Sender}, thread, time::Duration};

use crate::{application::message::message::show_message, domain::{app::model::app::App, error::Error, message::model::message::Message, result::result::AwpakResult}, infrastructure::action::window::window_action::WindowAction};

pub fn app_exec_actions_while_err(
    mut app : App,
    tx : Sender<WindowAction>,
    actions : Vec<impl FnOnce( App ) -> AwpakResult<App>>
) -> App
{
    let tx_cancel_loading = send_show_loading( tx.clone() );

    for a in actions
    {
        app = match a( app ).collect()
        {
            ( a, None ) => return
            {
                send_cancel_loading( tx_cancel_loading );

                let _ = tx.send( WindowAction::Render( a.clone() ) );

                a
            },
            ( a, Some( _ ) ) => a
        };
    }

    let _ = tx.send( WindowAction::HideLoading );

    app
}

pub fn app_exec_action( 
    app : App, 
    tx : Sender<WindowAction>,
    action : impl FnOnce( App ) -> AwpakResult<App>
) -> App
{
    let tx_cancel_loading = send_show_loading( tx.clone() );

    match action( app ).collect()
    {
        ( a, None ) =>
        {
            send_cancel_loading( tx_cancel_loading );

            let _ = tx.send( WindowAction::Render( a.clone() ) );

            a
        },
        ( a, Some( e ) ) => 
        {
            send_cancel_loading( tx_cancel_loading );

            parse_error( a, Some( tx ), e )
        }
    }
}

pub fn app_exec_action_allways_refresh( 
    app : App, 
    tx : Sender<WindowAction>,
    action : impl FnOnce( App ) -> AwpakResult<App>
) -> App
{
    let tx_cancel_loading = send_show_loading( tx.clone() );

    match action( app ).collect()
    {
        ( a, None ) =>
        {
            send_cancel_loading( tx_cancel_loading );

            let _ = tx.send( WindowAction::Render( a.clone() ) );

            a
        },
        ( a, Some( e ) ) => 
        {
            send_cancel_loading( tx_cancel_loading );

            let a = parse_error( a, None, e );

            let _ = tx.send( WindowAction::Render( a.clone() ) );

            a
        }
    }
}

fn send_cancel_loading( tx : Sender<bool> )
{
    let _ = tx.send( true );
}

fn send_show_loading( tx : Sender<WindowAction> ) -> Sender<bool>
{
    let ( tx_cancel_loading, rx_cancel_loading ) = mpsc::channel();

    let _ = tx.send( WindowAction::ShowLoading );

    let _ = thread::spawn( move || 
        {
            thread::sleep( Duration::from_millis( 500 ) );

            let cancel = rx_cancel_loading.try_recv().unwrap_or( false );

            if ! cancel { let _ = tx.send( WindowAction::ConfirmShowLoading ); }
        } 
    );

    tx_cancel_loading
}

fn parse_error( app : App, tx : Option<Sender<WindowAction>>, err : Error ) -> App
{
    match message_from_err( err )
    {
        Some( m ) =>
        {
            let app = show_message( m )( app ).own();

            if let Some( tx ) = tx
            {
                let _ = tx.send( WindowAction::Render( app.clone() ) );
            }
            
            app
        },
        None => 
        {
            if let Some( tx ) = tx
            {
                let _ = tx.send( WindowAction::HideLoading );
            }

            app
        }
    }
}

fn message_from_err( err : Error ) -> Option<Message>
{
    match err
    {
        Error::DestinationPathExists( s ) |
        Error::CopyFile( s ) |
        Error::CopyDirectory( s ) |
        Error::DeleteFile( s ) |
        Error::DeleteDirectory( s ) => Some( Message::Error( s ) ),
        _ => None
    }
}