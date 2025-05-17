use crate::domain::{app::model::app::App, error::Error, message::model::message::Message, result::{functions::result_utils::bool_err, result::AwpakResult}};


pub fn show_message( message : Message ) -> impl FnOnce( App ) -> AwpakResult<App>
{
    move | a | AwpakResult::new( a.change_message( Some( message ) ) )
}

pub fn hide_message( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.message().is_none(), Error::Ignore ) )
    .write()
    .map( | a | a.change_message( None ) )
    .read()
}