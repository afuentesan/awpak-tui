use crate::domain::{app::model::app::AppContent, content_generator::model::content_generator::ContentGenerator, directory::functions::expand_dir::{dir_expand, parent_directory}, error::Error, executable_expandable::functions::executable_expandable::expand_excutable_expandable, expandable::functions::expandable::expand_expandable};

pub fn parent_of_generator( generator : &ContentGenerator ) -> Result<ContentGenerator, Error>
{
    match generator
    {
        ContentGenerator::Directory( d ) => Ok( ContentGenerator::Directory( parent_directory( d )? ) ),
        ContentGenerator::Expandable( _ ) => Err( Error::Ignore ),
        ContentGenerator::ExecutableExpandable( _ ) => Err( Error::Ignore ),
        ContentGenerator::Detail( _, _ ) => Err( Error::Ignore ),
        ContentGenerator::Graph( _, _ ) => Err( Error::Ignore ),
        ContentGenerator::Empty => Err( Error::EmptyContentGenerator )   
    }
}

pub fn generate_content(
    generator : &ContentGenerator
) -> Result<AppContent, Error>
{
    match generator
    {
        ContentGenerator::Directory( d ) =>
        {
            Ok( dir_expand( d )? )
        },
        ContentGenerator::Expandable( e ) =>
        {
            Ok( expand_expandable( e )? )
        },
        ContentGenerator::ExecutableExpandable( e ) =>
        {
            Ok( expand_excutable_expandable( e )? )
        },
        ContentGenerator::Detail( _, _ ) => Err( Error::Ignore ),
        ContentGenerator::Graph( _, _ ) => Err( Error::Ignore ),
        ContentGenerator::Empty => Err( Error::EmptyContentGenerator )
    }
}