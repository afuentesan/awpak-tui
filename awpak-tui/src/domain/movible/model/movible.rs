use crate::domain::{directory::model::directory::Directory, file::model::file::File};

#[derive(Clone)]
pub enum Movible
{
    File( File ),
    Directory( Directory )
}

#[derive(Clone)]
pub enum MovibleAction
{
    Copy( Vec<Movible> ),
    Cut( Vec<Movible> ),
    Delete( Vec<Movible> ),
    None
}

impl Movible
{
    pub fn path( &self ) -> &std::path::Path
    {
        match self
        {
            Movible::File( f ) => f.path(),
            Movible::Directory( d ) => d.path()    
        }
    }
}