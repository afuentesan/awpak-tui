use awpak_tui_ai::domain::{chat::chat::{Chat, ChatRequest}, data::data_utils::option_value_to_string};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::{app::model::app::{App, AppContent, AppFocus, Confirm}, chat::{functions::chat::{is_chat_content, is_chat_request_empty, is_chat_request_pending}}, content_generator::model::content_generator::ContentGenerator, error::Error, input::model::input::Input, result::{functions::result_utils::bool_err, result::AwpakResult}, selectable::{functions::selectable_utils::idx_current_selected_item, model::selectable::Selectable}};

pub fn confirm_agent_selection( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( idx_current_selected_item( a.ai_agents() ).is_none(), Error::Ignore ) )
    .write()
    .map( | a | show_chat_agent_selected( a ) )
    .finalize()
    .unzip( | a | a.change_focus( AppFocus::Search ) )
    .read()
}

pub fn open_chat( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( _ ) => AwpakResult::new_err( app, Error::Ignore ),
        _ => show_chat( app )
    }
}

fn show_chat( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.ai_agents().len() == 0, Error::Ignore ) )
    .write()
    .map_if(
        | a | a.ai_agents().len() == 1, 
        | a |
        {
            let ( a, agents ) = a.own_ai_agents();

            let a = a.change_ai_agents( agents.into_iter().map( | g | g.to_current_selected() ).collect() );

            show_chat_agent_selected( a )
        }
    )
    .map_if( 
        | a | a.ai_agents().len() > 1, 
        | a |
        {
            a.change_focus( AppFocus::Confirm( Confirm::AgentSelection ) )
        }
    )
    .read()
}

fn show_chat_agent_selected( app : App ) -> App
{
    AwpakResult::new( app )
    .validate()
    .zip_result( | a | idx_current_selected_item( a.ai_agents() ).ok_or( Error::Ignore ) )
    .write()
    .map( 
        | ( a, i ) |
        {
            let new_chat = Chat::new( Uuid::new_v4().to_string().as_str(), a.ai_agents()[ *i.as_ref().unwrap() ].inner().clone() );

            ( a.change_content( AppContent::Chat( new_chat ) ), i )
        }
    )
    .map( | ( a, i ) | ( chat_content_generator( a ), i ) )
    .map( | ( a, i ) | ( a.change_focus( AppFocus::Search ), i ) )
    .finalize()
    .unzip( | ( a, _ ) | a )
    .own()
}

fn chat_content_generator( app : App ) -> App
{
    let ( app, generator ) = app.own_content_generator();

    let id = app.chat_content().map( | c | format!( "chat_{}", c.id() ) ).unwrap_or( "chat".to_string() );

    let generator = match generator
    {
        ContentGenerator::Detail( g, _ ) => ContentGenerator::Chat( g, id ),
        _ => ContentGenerator::Chat( Box::new( generator ), id )
    };

    app.change_content_generator( generator )
}

pub fn append_text_to_content( text : String ) -> impl Fn( App ) -> AwpakResult<App>
{
    move | app |
    {
        let ( app, content ) = app.own_content();

        match content
        {
            AppContent::Chat( c ) => append_text_to_chat( app, c, text.as_str() ),
            _ => AwpakResult::new_err( app.change_content( content ), Error::Ignore )
        }
    }
}

fn append_text_to_chat( app : App, chat : Chat, text : &str ) -> AwpakResult<App>
{
    AwpakResult::new( app.change_content( AppContent::Chat( chat.append_response( text ) ) ) )
}

pub fn send_chat_request( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.content_search().text.trim() == "", Error::Ignore ) )
    .map_result( | a | bool_err( ! is_chat_content( a.content() ), Error::Ignore ) )
    .map_result( | a | bool_err( ! is_chat_request_empty( a.chat_content().unwrap().request() ), Error::Ignore ) )
    .write()
    .map(
        | a |
        {
            let ( a, chat ) = a.own_chat_content();

            let chat = chat.unwrap().change_request( ChatRequest::Pending( Value::String( a.content_search().text.trim().to_string() ) ) );

            let a = a.change_content_search( Input::default() );

            a.change_content( AppContent::Chat( chat ) )
        }
    )
    .read()
}

pub fn pending_chat( app : &App ) -> Option<Chat>
{
    match app.content()
    {
        AppContent::Chat( c ) => if is_chat_request_pending( c.request() ) { Some( c.clone() ) } else { None },
        _ => None    
    }
}

pub fn chat_to_waiting( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( _ ) =>
        {
            let ( app, chat ) = app.own_chat_content();

            match append_request_to_response( chat.unwrap() ).collect()
            {
                ( c, None ) => AwpakResult::new( 
                    app.change_content( 
                        AppContent::Chat( 
                            c.change_request( ChatRequest::Waiting ) 
                        )
                    )
                ),
                ( c, Some( e ) ) => AwpakResult::new_err( app.change_content( AppContent::Chat( c ) ), e )
            }
        },
        _ => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn finalize_chat_response( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Chat( _ ) =>
        {
            let ( app, chat ) = app.own_chat_content();
            let chat = chat.unwrap().change_request( ChatRequest::Empty );

            let chat = chat.append_response( "\n" );

            AwpakResult::new( app.change_content( AppContent::Chat( chat ) ) )
        },
        _ => AwpakResult::new_err( app, Error::Ignore )    
    }
}

fn append_request_to_response( chat : Chat ) -> AwpakResult<Chat>
{
    AwpakResult::new( chat )
    .validate()
    .map_result( | c | bool_err( ! is_chat_request_pending( c.request() ), Error::Ignore ) )
    .write()
    .map( 
        | c |
        {
            let ( chat, text ) = c.own_request_value();

            chat.append_response( format!( "\nPrompt:\n{}\n", option_value_to_string( text.as_ref() ) ).as_str() )
        }
    )
    .read()
}