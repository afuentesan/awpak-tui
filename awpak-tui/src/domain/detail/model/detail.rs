use crate::domain::{app::model::app::AppContent, table::model::table::Table};

#[derive(Clone)]
pub struct Detail
{
    detail : DetailContent,
    source : AppContent
}

#[derive(Clone)]
pub enum DetailContent
{
    Table( Table ),
    Empty
}

impl Detail
{
    pub fn new( source : AppContent, content : DetailContent ) -> Self
    {
        Self { detail : content, source }
    }

    pub fn source( &self ) -> &AppContent
    {
        &self.source
    }

    pub fn own_source( mut self ) -> ( Self, AppContent )
    {
        let old = std::mem::replace( &mut self.source, AppContent::Empty );

        ( self, old )
    }

    pub fn change_source( mut self, new : AppContent ) -> Self
    {
        self.source = new;

        self
    }

    pub fn content( &self ) -> &DetailContent
    {
        &self.detail
    }

    pub fn own_content( mut self ) -> ( Self, DetailContent )
    {
        let old = std::mem::replace( &mut self.detail, DetailContent::Empty );

        ( self, old )
    }

    pub fn change_content( mut self, new : DetailContent ) -> Self
    {
        self.detail = new;

        self
    }
}