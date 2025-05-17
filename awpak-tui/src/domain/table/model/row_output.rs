use crate::domain::{app::model::app::AppContent, content_generator::model::content_generator::ContentGenerator, field::model::field::Field, message::model::message::Message};


pub enum RowOutput
{
    NewContent { generator : ContentGenerator, content : AppContent },
    ShowMessage( Message ),
    EditField( Field ),
    Ignore
}