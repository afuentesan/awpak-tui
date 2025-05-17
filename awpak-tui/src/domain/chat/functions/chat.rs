use awpak_tui_ai::domain::chat::chat::ChatRequest;

use crate::domain::{app::model::app::AppContent};


pub fn is_chat_content( content : &AppContent ) -> bool
{
    match content
    {
        AppContent::Chat( _ ) => true,
        _ => false    
    }
}

pub fn is_chat_request_empty( request : &ChatRequest ) -> bool
{
    match request
    {
        ChatRequest::Empty => true,
        _ => false    
    }
}

pub fn is_chat_request_pending( request : &ChatRequest ) -> bool
{
    match request
    {
        ChatRequest::Pending( _ ) => true,
        _ => false    
    }
}