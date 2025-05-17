use std::sync::Arc;

use rig::{agent::Agent, message::{AssistantContent, Message, ToolCall, ToolFunction, ToolResult, ToolResultContent, UserContent}, streaming::{StreamingChat, StreamingChoice, StreamingCompletionModel, StreamingResult}, OneOrMany};
use serde_json::Value;
use tokio_stream::{Stream, StreamExt as _};

use crate::domain::{chat::chat::ChatChannel, error::Error, util::file_utils::log_to_file};

use super::{node::{Node, NodeOutputDestination, NodeProvider}, node_client::{NodeClient, NodeClientProvider}, ollama_node_client::ollama_node_client, openai_node_client::openai_node_client};

// SEND PROMPT TO NODE

pub async fn send_prompt_to_node<T: StreamingCompletionModel, U: ChatChannel>( 
    prompt : &str,
    mut chat_history : Vec<Message>,
    chat_channel : U,
    agent : Arc<Agent<T>>,
    output_destination : &NodeOutputDestination
) -> Result<( String, Vec<Message> ), Error>
{
    response_header( &chat_channel, output_destination );

    let mut stream = stream_chat( agent.clone(), prompt, chat_history.clone() ).await?;

    let mut response = stream_response( &agent, &mut stream, &chat_channel, output_destination ).await?;

    chat_history.push( Message::user( prompt ) );

    while let StreamResponse::ToolCall( _p, mut h ) = response
    {
        chat_history.append( &mut h );

        let mut stream = stream_chat( agent.clone(), "", chat_history.clone() ).await?;

        response = stream_response( &agent, &mut stream, &chat_channel, output_destination ).await?;
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

async fn stream_response<M: StreamingCompletionModel, U: ChatChannel>(
    agent: &Agent<M>,
    stream: &mut StreamingResult,
    chat_channel : &U,
    output_destination : &NodeOutputDestination
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
            Ok( StreamingChoice::Message( text ) ) => 
            {
                complete_output = stream_response_append_text( text, chat_channel, complete_output, output_destination );
            },
            Ok( StreamingChoice::ToolCall( name, id, arguments ) ) =>
            {
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

async fn stream_response_tool_call<M: StreamingCompletionModel>(
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

    history.push( assistant_message_tool_call( id.clone(), name, arguments ) );

    tool_response.push_str( &response );

    history.push( user_tool_response( id, response ) );

    Ok( ( tool_response, history ) )
}

fn assistant_message_tool_call( id : String, name : String, arguments : Value ) -> Message
{
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

fn stream_response_append_text<T: ChatChannel>( 
    text : String, 
    chat_channel : &T, 
    mut append_to : String,
    output_destination : &NodeOutputDestination
) -> String
{
    append_to.push_str( text.as_str() );

    match output_destination
    {
        NodeOutputDestination::Channel =>
        {
            let _ = chat_channel.send_str( text.as_str() );
        },
        NodeOutputDestination::Log( p ) => 
        {
            let _ = log_to_file( text.as_str(), p );
        },
        NodeOutputDestination::Ignore => {}
    };

    append_to
}

async fn stream_chat<T: StreamingCompletionModel>( 
    agent : Arc<Agent<T>>,
    prompt : &str, 
    chat_history : Vec<Message> 
) -> Result<std::pin::Pin<Box<dyn Stream<Item = Result<rig::streaming::StreamingChoice, rig::completion::CompletionError>> + Send + 'static>>, Error>
{
    agent
    .stream_chat( 
        prompt, 
        chat_history.clone()
    )
    .await.map_err( | e | Error::AgentErr( e.to_string() ) )
}

fn response_header<U: ChatChannel>(
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

// CREATE NODE CLIENT

pub async fn create_node_client( 
    id : &str, 
    node : &Node
) -> Result<NodeClient, Error>
{
    Ok(
        NodeClient 
        { 
            id : id.to_string(), 
            history : vec![], 
            save_history: node.save_history, 
            output : node.output.clone(),
            provider : create_node_client_provider( node ).await?
        }
    )
}

async fn create_node_client_provider( 
    node : &Node
) -> Result<NodeClientProvider, Error>
{
    match &node.provider
    {
        NodeProvider::Ollama( c ) => ollama_node_client( node, c ).await,
        NodeProvider::OpenAI( c ) => openai_node_client( node,c ).await,
        NodeProvider::Empty => return Err( Error::AgentErr( "AgentErr: Empty NodeProvider".into() ) )    
    }
}

// END CREATE NODE CLIENT