use std::{sync::Arc, time::Duration};

use rig::{agent::Agent, completion::CompletionModel, message::{AssistantContent, Message, ToolCall, ToolFunction, ToolResult, ToolResultContent, UserContent}, streaming::StreamingChat, OneOrMany};
use serde_json::Value;
use tokio::time::sleep;
use tokio_stream::StreamExt as _;

use crate::domain::{chat::chat::ChatChannel, error::Error, util::file_utils::log_to_file};

use super::node::NodeOutputDestination;

// SEND PROMPT TO NODE

pub async fn send_prompt_to_node<T: CompletionModel, U: ChatChannel + Send + Sync>( 
    prompt : &str,
    mut chat_history : Vec<Message>,
    chat_channel : U,
    agent : Arc<Agent<T>>,
    output_destination : &NodeOutputDestination,
    tools_output : Option<&NodeOutputDestination>,
    millis_between_tool_call : Option<u64>,
    millis_between_streams : Option<u64>
) -> Result<( String, Vec<Message> ), Error>
{
    response_header( &chat_channel, output_destination );

    let mut stream = stream_chat( agent.clone(), prompt, chat_history.clone() ).await?;

    let mut response = stream_response( &agent, &mut stream, &chat_channel, output_destination, tools_output, millis_between_tool_call ).await?;

    chat_history.push( Message::user( prompt ) );

    while let StreamResponse::ToolCall( p, mut h ) = response
    {
        if let Some( od ) = tools_output
        {
            trace_node_prompt( 
                od, 
                format!( "\n\n{}\n\n", p.as_str() ).as_str(), 
                &chat_channel 
            );
        }

        if let Some( m ) = millis_between_streams
        {
            let _ = sleep( Duration::from_millis( m ) ).await;
        }
        
        chat_history.append( &mut h );

        let mut stream = stream_chat( agent.clone(), p.as_str(), chat_history.clone() ).await?;

        response = stream_response( &agent, &mut stream, &chat_channel, output_destination, tools_output, millis_between_tool_call ).await?;

        chat_history.push( Message::user( p ) );
    }

    let output = match response
    {
        StreamResponse::End( output, mut history ) => 
        {
            chat_history.append( &mut history );

            output
        },
        StreamResponse::ToolCall( output, _ ) => output
    };

    Ok( ( output, chat_history ) )
}

enum StreamResponse
{
    ToolCall( String, Vec<Message> ),
    End( String, Vec<Message> )
}

impl From<( String, String, Vec<Message> )> for StreamResponse
{
    fn from( ( tool_response, output, history ) : ( String, String, Vec<Message> ) ) -> Self 
    {
        if tool_response.trim() != ""
        {
            StreamResponse::ToolCall( tool_response, history )
        }
        else
        {
            StreamResponse::End( output, history )    
        }
    }
}

async fn stream_response<M: CompletionModel, U: ChatChannel + Send + Sync>(
    agent: &Agent<M>,
    stream: &mut rig::streaming::StreamingCompletionResponse<<M>::StreamingResponse>,
    chat_channel : &U,
    output_destination : &NodeOutputDestination,
    tools_output : Option<&NodeOutputDestination>,
    millis_between_tool_call : Option<u64>
) -> Result<StreamResponse, Error> 
{
    let mut history : Vec<Message> = vec![];

    let mut tool_response = "".to_string();

    let mut complete_output : String = "".to_string();

    while let Some( chunk ) = stream.next().await 
    {
        if chat_channel.abort() { return Err( Error::AgentErr( "Abort".to_string() ) ) }

        match chunk 
        {
            Ok( AssistantContent::Text( text ) ) => 
            {
                complete_output = stream_response_append_text( text.text, chat_channel, complete_output, output_destination );
            },
            Ok( AssistantContent::ToolCall( t ) ) =>
            {
                let id = t.id;
                let name = t.function.name;
                let arguments = t.function.arguments;

                if let Some( od ) = tools_output
                {
                    trace_node_prompt( 
                        od, 
                        format!( "\n\nName: {}\nId: {}\nArguments: {}\n\n", name, id, arguments ).as_str(), 
                        chat_channel 
                    );
                }

                if let Some( m ) = millis_between_tool_call
                {
                    let _ = sleep( Duration::from_millis( m ) ).await;
                }

                ( tool_response, history ) = stream_response_tool_call( 
                    &agent,
                    tool_response,
                    history, 
                    id, 
                    name, 
                    arguments 
                ).await?;
            },
            Err( e ) => 
            {
                return Err( Error::AgentErr( e.to_string() ) )
            }
        }
    }

    if complete_output.trim() != ""
    {
        history.push( Message::assistant( complete_output.clone() ) );
    }

    Ok( StreamResponse::from( ( tool_response, complete_output, history ) ) )
}

async fn stream_response_tool_call<M: CompletionModel>(
    agent: &Agent<M>,
    mut tool_response : String,
    mut history : Vec<Message>,
    id : String,
    name : String,
    arguments : Value
) -> Result<( String, Vec<Message> ), Error>
{
    let response = agent
                    .tools
                    .call( &name, arguments.to_string() )
                    .await
                    .map_err( | e | Error::MCPToolErr( e.to_string() ) )?;

    history.push( assistant_message_tool_call( id.clone(), name.clone(), arguments ) );

    tool_response.push_str( &response );

    let _ = log_to_file( &response, "/tmp/response.log" );

    // history.push( user_tool_response( if id.trim() == "" { name } else { id }, response ) );

    history.push( user_tool_response( if id.trim() == "" { name } else { id }, "{}".to_string() ) );

    Ok( ( tool_response, history ) )
}

fn assistant_message_tool_call( id : String, name : String, arguments : Value ) -> Message
{
    let _ = log_to_file( format!( "Name: {name}" ).as_str(), "/tmp/response.log" );

    Message::Assistant 
    { 
        content : OneOrMany::one(
            AssistantContent::ToolCall(
                ToolCall
                {
                    id,
                    function : ToolFunction
                    {
                        name,
                        arguments
                    }
                }
            )
        )
    }
}

fn user_tool_response( id : String, text : String ) -> Message
{
    Message::User 
    { 
        content : OneOrMany::one(
            UserContent::ToolResult(
                ToolResult
                {
                    id,
                    content : OneOrMany::one(
                        ToolResultContent::Text( text.into() )
                    )
                }
            )
        ) 
    }
}

fn stream_response_append_text<T: ChatChannel + Send + Sync>( 
    text : String, 
    chat_channel : &T, 
    mut append_to : String,
    output_destination : &NodeOutputDestination
) -> String
{
    append_to.push_str( text.as_str() );

    trace_node_prompt( output_destination, text.as_str(), chat_channel );

    append_to
}

pub fn trace_node_prompt<T>(
    output : &NodeOutputDestination,
    prompt : &str,
    chat_channel : &T
)
where T: ChatChannel + Send + Sync
{
    match output
    {
        NodeOutputDestination::Channel =>
        {
            let _ = chat_channel.send_str( prompt );
        },
        NodeOutputDestination::Log( p ) =>
        {
            let _ = log_to_file( prompt, p );
        },
        NodeOutputDestination::Ignore => {}
    }
}

async fn stream_chat<T: CompletionModel>( 
    agent : Arc<Agent<T>>,
    prompt : &str, 
    chat_history : Vec<Message> 
) -> Result<rig::streaming::StreamingCompletionResponse<<T>::StreamingResponse>, Error>
// -> Result<std::pin::Pin<Box<dyn Stream<Item = Result<StreamingCompletionResponse<String>, rig::completion::CompletionError>> + Send + 'static>>, Error>
{
    agent
    .stream_chat( 
        prompt, 
        chat_history.clone()
    )
    .await.map_err( | e | Error::AgentErr( e.to_string() ) )
}

pub fn response_header<U: ChatChannel>(
    chat_channel : &U,
    output_destination : &NodeOutputDestination
)
{
    let _ = chat_channel.send_str( "\n\nResponse:\n" );

    match output_destination
    {
        NodeOutputDestination::Channel => {},
        NodeOutputDestination::Ignore => 
        {
            let _ = chat_channel.send_str( "Output ignored\n" );
        },
        NodeOutputDestination::Log( p ) =>
        {
            let _ = chat_channel.send_str( format!( "Output send to file {}\n", p ).as_str() );
        }
    };
}

// END SEND PROMPT TO NODE

