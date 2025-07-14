use std::{collections::HashMap, sync::mpsc::Sender};

use tracing::{field::{Field, Visit}, Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};


pub struct AwpakAIFilterLayer
{
    pub allowed : Vec<( AwpakAITarget, Sender<AwpakTracingMessage> )>,
}

pub const AGENT_PROMPT : &'static str = "agent_prompt";
pub const AGENT_STREAM : &'static str = "agent_stream";
pub const AGENT_SYNC : &'static str = "agent_sync";
pub const AGENT_TOOL_CALL : &'static str = "agent_tool_call";
pub const AGENT_TOOL_RESULT : &'static str = "agent_tool_result";

pub const COMMAND_AND_ARGS : &'static str = "command_and_args";
pub const COMMAND_RESULT : &'static str = "command_result";

pub const WEB_CLIENT_REQUEST : &'static str = "web_client_request";
pub const WEB_CLIENT_REQUEST_BODY : &'static str = "web_client_request_body";
pub const WEB_CLIENT_REQUEST_HEADERS : &'static str = "web_client_request_headers";
pub const WEB_CLIENT_REQUEST_QUERY_PARAMS : &'static str = "web_client_request_query_params";

pub const NODE_DESTINATION : &'static str = "node_destination";
pub const NODE_EXECUTION : &'static str = "node_execution";
pub const NODE_OUTPUT : &'static str = "node_output";

pub enum AwpakAITarget
{
    AgentPrompt,
    AgentStream,
    AgentSync,
    AgentToolCall,
    AgentToolResult,
    CommandAndArgs,
    CommandResult,
    WebClientRequest,
    WebClientRequestBody,
    WebClientRequestHeaders,
    WebClientRequestQueryParams,
    NodeDestination,
    NodeExecution,
    NodeOutput
}

impl AwpakAITarget
{
    pub fn as_str( &self ) -> &'static str
    {
        match self
        {
            AwpakAITarget::AgentPrompt => AGENT_PROMPT,
            AwpakAITarget::AgentStream => AGENT_STREAM,
            AwpakAITarget::AgentSync => AGENT_SYNC,
            AwpakAITarget::AgentToolCall => AGENT_TOOL_CALL,
            AwpakAITarget::AgentToolResult => AGENT_TOOL_RESULT,
            AwpakAITarget::CommandAndArgs => COMMAND_AND_ARGS,
            AwpakAITarget::CommandResult => COMMAND_RESULT,
            AwpakAITarget::WebClientRequest => WEB_CLIENT_REQUEST,
            AwpakAITarget::WebClientRequestBody => WEB_CLIENT_REQUEST_BODY,
            AwpakAITarget::WebClientRequestHeaders => WEB_CLIENT_REQUEST_HEADERS,
            AwpakAITarget::WebClientRequestQueryParams => WEB_CLIENT_REQUEST_QUERY_PARAMS,
            AwpakAITarget::NodeDestination => NODE_DESTINATION,
            AwpakAITarget::NodeExecution => NODE_EXECUTION,
            AwpakAITarget::NodeOutput => NODE_OUTPUT
        }
    }
}

pub struct AwpakTracingMessage
{
    pub id : Option<String>,
    pub text : String,
    pub target : String
}

impl<S> Layer<S> for AwpakAIFilterLayer
where
    S: Subscriber,
{
    fn on_event( &self, event : &Event<'_>, _ : Context<'_, S> )
    {
        let mut visitor = FieldVisitor::default();

        event.record( &mut visitor );

        if let Some( text ) = visitor.fields.remove( "text" )
        {
            let _ = self.allowed.iter()
            .find( | ( t, _ ) | t.as_str() == event.metadata().target() )
            .map( | ( _, c ) | c )
            .map( 
                | c |
                {
                    let id = match visitor.fields.remove( "id" )
                    {
                        Some( id ) if id.trim() != "" => Some( id ),
                        _ => None
                    };

                    let _ = c.send( 
                        AwpakTracingMessage 
                        { 
                            id,
                            text,
                            target : event.metadata().target().to_string()
                        }
                    );
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