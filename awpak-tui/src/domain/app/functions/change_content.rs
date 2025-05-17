use crate::{application::sortable::sort::sort_content, domain::{app::model::app::{App, AppContent}, content_generator::functions::generator::generate_content}};


pub fn change_app_content( app : App, content : AppContent ) -> App
{
    let app = app.change_content( content );

    let sort_by = app.content_sort();

    sort_content( app, sort_by ).own()
}

pub fn reload_app_content( app : App ) -> App
{
    let content = generate_content( app.content_generator() );

    match content
    {
        Ok( c ) => app.change_content( c ),
        _ => app    
    }
}