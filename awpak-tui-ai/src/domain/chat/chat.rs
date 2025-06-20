use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::domain::agent::agent::AIAgent;


#[derive(Clone)]
pub struct Chat
{
    id : String,
    request : ChatRequest,
    response : String,
    agent : AIAgent
}

#[derive(Clone)]
pub enum ChatRequest
{
    Pending( Value ),
    Waiting,
    Empty
}

impl Chat
{
    pub fn new( id : &str, agent : AIAgent ) -> Self
    {
        Self 
        { 
            id : id.to_string(),
            request : ChatRequest::Empty, 
            response : "".to_string(), 
            agent
        }
    }

    pub fn id( &self ) -> &str
    {
        &self.id
    }

    pub fn response( &self ) -> &str
    {
        &self.response
    }
    
    pub fn append_response( mut self, text : &str ) -> Self
    {
        self.response.push_str( text );

        self
    }
    
    pub fn request( &self ) -> &ChatRequest
    {
        &self.request
    }

    // pub fn own_request( mut self ) -> ( Self, ChatRequest )
    // {
    //     let request = std::mem::replace( &mut self.request, ChatRequest::Empty );

    //     ( self, request )
    // }

    pub fn change_request( mut self, new : ChatRequest ) -> Self
    {
        self.request = new;

        self
    }

    // pub fn own_llm( mut self ) -> ( Self, LLM )
    // {
    //     let llm = std::mem::replace( &mut self.llm, LLM::Empty );

    //     ( self, llm )
    // }

    pub fn agent( &self ) -> &AIAgent
    {
        &self.agent
    }

    pub fn own_agent( mut self ) -> ( Self, AIAgent )
    {
        let agent = std::mem::replace( &mut self.agent, AIAgent::default() );

        ( self, agent )
    }

    pub fn request_value( &self ) -> Option<&Value>
    {
        match self.request()
        {
            ChatRequest::Pending( t ) => Some( t ),
            _ => None
        }
    }

    pub fn own_request_value( mut self ) -> ( Self, Option<Value> )
    {
        let request = std::mem::replace( &mut self.request, ChatRequest::Empty );

        match request
        {
            ChatRequest::Pending( s ) => ( self, Some( s ) ),
            _ => ( self, None )    
        }
    }
}

impl Default for Chat
{
    fn default() -> Self 
    {
        Self 
        { 
            id : Uuid::new_v4().to_string(),
            request : ChatRequest::Empty, 
            response: Default::default(), 
            agent : AIAgent::default()
        }
    }
}

impl ToString for Chat
{
    fn to_string( &self ) -> String
    {
        let response = self.response.trim().replacen( "Prompt:", "", 1 ).replace( "\n", " " );

        if response != ""
        {
            let len = response.graphemes( true ).count();

            let last = usize::min( len, 100 );

            UnicodeSegmentation::graphemes( response.as_str(), true )
            .take( last )
            .collect::<String>()
        }
        else
        {
            "Empty chat".into()
        }
    }
}

pub trait EndChat
{
    fn end_chat() -> Self;
}

pub trait StrToChat
{
    fn str_to_chat( s : &str ) -> Self;
}

pub trait ChatChannel : Clone + Send + 'static
{
    fn send_str( &self, s : &str );
    fn end_chat( &self );
    fn abort( &self ) -> bool;
}
