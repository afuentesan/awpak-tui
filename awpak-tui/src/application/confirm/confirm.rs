use crate::{application::{graph::graph::{confirm_graph_selection, confirm_saved_graph_selection}, movible::movible::confirm_movible_action}, domain::{app::model::app::{App, AppFocus, Confirm}, error::Error, movible::model::movible::MovibleAction, result::result::AwpakResult}};


pub fn confirm_action( app : App, confirm : Confirm ) -> AwpakResult<App>
{
    match confirm
    {
        Confirm::MovibleAction => confirm_movible_action( app )
                                  .finalize()
                                  .unzip( | a | a.change_movible_action( MovibleAction::None ) )
                                  .read(),
        Confirm::GraphSelection => confirm_graph_selection( app ),
        Confirm::SavedGraphSelection => confirm_saved_graph_selection( app )
    }
    .finalize()
    .unzip( | a | a.change_focus( AppFocus::Search ) )
    .read()
}

pub fn discard_action( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Confirm( c ) => discard_confirm_action( app, c ),
        AppFocus::Field => discard_field_action( app ),
        _ => AwpakResult::new_err( app, Error::Ignore ) 
    }
}

fn discard_field_action( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app.change_field( None ) )
    .write()
    .map( | a | a.change_focus( AppFocus::Content ) )
    .read()
}

fn discard_confirm_action( app : App, confirm : Confirm ) -> AwpakResult<App>
{
    match confirm
    {
        Confirm::MovibleAction => AwpakResult::new( app.change_movible_action( MovibleAction::None ) ),
        Confirm::GraphSelection => AwpakResult::new( app ),
        Confirm::SavedGraphSelection => AwpakResult::new( app )
    }
    .finalize()
    .unzip( | a | a.change_focus( AppFocus::Content ) )
    .read()
}