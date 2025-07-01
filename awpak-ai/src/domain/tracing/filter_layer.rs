use std::{collections::HashMap, sync::mpsc::Sender};

use tracing::{field::{Field, Visit}, Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};


pub struct AwpakAIFilterLayer
{
    pub allowed : Vec<( AwpakAITarget, Sender<String> )>,
}

pub const AGENT_STREAM : &'static str = "agent_stream";
pub const AGENT_TOOL_CALL : &'static str = "agent_tool_call";
pub const AGENT_TOOL_RESULT : &'static str = "agent_tool_result";

pub enum AwpakAITarget
{
    AgentStream,
    AgentToolCall,
    AgentToolResult
}

impl AwpakAITarget
{
    pub fn as_str( &self ) -> &'static str
    {
        match self
        {
            AwpakAITarget::AgentStream => AGENT_STREAM,
            AwpakAITarget::AgentToolCall => AGENT_TOOL_CALL,
            AwpakAITarget::AgentToolResult => AGENT_TOOL_RESULT    
        }
    }
}

impl<S> Layer<S> for AwpakAIFilterLayer
where
    S: Subscriber,
{
    fn on_event( &self, event : &Event<'_>, _ : Context<'_, S> )
    {
        let mut visitor = FieldVisitor::default();

        event.record( &mut visitor );

        // println!( "Event: {:?}", event.fields() );

        if let Some( text )  = visitor.fields.remove( "text" ) 
        {
            // println!( "Text: {}, {}", text, event.metadata().target() );

            let _ = self.allowed.iter()
            .find( | ( t, _ ) | t.as_str() == event.metadata().target() )
            .map( | ( _, c ) | c )
            .map( 
                | c |
                {
                    let _ = c.send( text );

                    // println!( "Despues de send" );
                }
            );
        }
    }

    fn enabled( &self, metadata : &tracing::Metadata<'_>, _ : Context<'_, S> ) -> bool 
    {
        self.allowed.iter().find( | a | a.0.as_str() == metadata.target() ).is_some()
    }
}

#[derive(Default)]
struct FieldVisitor 
{
    pub fields: HashMap<String, String>,
}

impl Visit for FieldVisitor 
{
    fn record_debug( &mut self, field : &Field, value : &dyn std::fmt::Debug )
    {
        self.fields.insert( field.name().to_string(), format!( "{:?}", value ) );
    }

    fn record_str( &mut self, field:  &Field, value : &str ) 
    {
        self.fields.insert( field.name().to_string(), value.to_string() );
    }
}