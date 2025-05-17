use crate::domain::{app::model::app::{App, AppContent}, content_generator::model::content_generator::ContentGenerator, detail::functions::detail::{detail_from_content, detail_id_from_content}, result::result::AwpakResult};


pub fn show_detail( app : App ) -> AwpakResult<App>
{
    let ( app, content ) = app.own_content();

    match detail_from_content( content ).collect()
    {
        ( c, None ) => show_some_detail( app, c ),
        ( c, Some( e ) ) => AwpakResult::new_err( app.change_content( c ), e )
    }
}

fn show_some_detail( app : App, content : AppContent ) -> AwpakResult<App>
{
    let ( app, generator ) = app.own_content_generator();

    let id = detail_id( &content );

    let generator = ContentGenerator::Detail( Box::new( generator ), id );

    let app = app.change_content( content );
    
    AwpakResult::new( app.change_content_generator( generator ) )
}

fn detail_id( content : &AppContent ) -> String
{
    detail_id_from_content( content ).unwrap_or( "detail".to_string() )
}
