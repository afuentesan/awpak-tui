use crate::domain::{app::model::app::AppContent, error::Error, expandable::model::expandable::Expandable, parser::functions::parser::path_to_content};


pub fn expand_expandable( expandable : &Expandable ) -> Result<AppContent, Error>
{
    path_to_content( expandable.path() )
}