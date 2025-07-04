use std::{pin::Pin, sync::{Arc, Mutex}};

use rig::{agent::Agent, completion::CompletionModel, message::{AssistantContent, Message, Text, ToolResultContent, UserContent}, streaming::StreamingCompletion, OneOrMany};
use tokio_stream::{Stream, StreamExt};
use tracing::info;
type StreamingResult = Pin<Box<dyn Stream<Item = Result<Text, Error>> + Send>>;
use crate::domain::{agent::agent::AIAgent, error::Error, signals::cancel_graph::is_graph_cancelled, tracing::filter_layer::{AGENT_STREAM, AGENT_TOOL_CALL, AGENT_TOOL_RESULT}};

pub async fn run_agent<T: CompletionModel + 'static>( 
    id : Option<&String>,
    prompt : String, 
    provider : Agent<T>, 
    agent : &AIAgent 
) -> Result<( String, Vec<Message> ), Error>
{
    let chat_history = Arc::new( Mutex::new( agent.history.clone() ) );
    let chat_history_result = chat_history.clone();

    let mut result = stream_chat( 
        match id
        {
            Some( id ) => Some( id.clone() ),
            _ => None
        },
        provider, 
        prompt, 
        chat_history
    ).await;
    
    let response = string_from_stream( id, &mut result ).await?;

    let mut history = chat_history_result.lock().unwrap().split_off( 0 );

    history.push( Message::assistant( response.clone() ) );

    Ok( ( response, history ) )
}

async fn string_from_stream(
    id : Option<&String>,
    stream: &mut StreamingResult
) -> Result<String, Error>
{
    let mut ret = String::new();

    while let Some( content ) = stream.next().await
    {
        if let Some( id ) = id
        {
            if is_graph_cancelled( id ) { return Err( Error::Agent( "Graph cancelled".into() ) ) }
        }

        match content 
        {
            Ok( Text { text } ) => 
            {
                info!( target:AGENT_STREAM, text );

                ret.push_str( text.as_str() );
            },
            Err( e ) => 
            {
                return Err( e )
            }
        }
    }

    Ok( ret )
}

async fn stream_chat<M>(
    id : Option<String>,
    agent: Agent<M>,
    prompt: impl Into<Message> + Send,
    chat_history: Arc<Mutex<Vec<Message>>>,
) -> StreamingResult
where
    M: CompletionModel + 'static,
    <M as CompletionModel>::StreamingResponse: std::marker::Send,
{
    let prompt: Message = prompt.into();

    (Box::pin(async_stream::stream! {
        let mut current_prompt = prompt;
        let mut did_call_tool = false;

        'outer: loop 
        {
            let h = chat_history.lock().unwrap().clone();

            let mut stream = agent
                .stream_completion(current_prompt.clone(), h )
                .await
                .map_err( | e | Error::Agent( e.to_string() ) )?
                .stream()
                .await
                .map_err( | e | Error::Agent( e.to_string() ) )?;

            chat_history.lock().unwrap().push( current_prompt );
            
            let mut tool_calls = vec![];
            let mut tool_results = vec![];

            while let Some(content) = stream.next().await 
            {
                if let Some( ref id ) = id
                {
                    if is_graph_cancelled( &id ) { yield Err( Error::Agent( "Graph cancelled".into() ) ) }
                }

                match content {
                    Ok(AssistantContent::Text(text)) => {
                        yield Ok(Text { text: text.text });
                        did_call_tool = false;
                    },
                    Ok(AssistantContent::ToolCall(tool_call)) => {

                        let msg_tool_call = format!( 
                            "\nToolCall:\n- Function name: {}\n- Args: {:?}\n", 
                            tool_call.function.name,
                            tool_call.function.arguments
                        );

                        info!( 
                            target:AGENT_TOOL_CALL,  
                            text=msg_tool_call
                        );

                        let tool_result =
                            agent.tools.call(&tool_call.function.name, tool_call.function.arguments.to_string())
                            .await
                            .map_err( | e | Error::MCPTool( e.to_string() ) )?;

                        let msg_tool_result = format!(
                            "\nTool result:\n{}\n",
                            tool_result
                        );

                        info!( 
                            target:AGENT_TOOL_RESULT, 
                            text=msg_tool_result 
                        );

                        let tool_call_msg = AssistantContent::ToolCall(tool_call.clone());

                        tool_calls.push(tool_call_msg);
                        tool_results.push((tool_call.id, tool_result));

                        did_call_tool = true;
                    },
                    Err(e) => {
                        yield Err( Error::Agent( e.to_string() ) );
                        break 'outer;
                    }
                }
            }

            // Add (parallel) tool calls to chat history
            if ! tool_calls.is_empty()
            {
                let m = Message::Assistant 
                {
                    content : OneOrMany::many( tool_calls ).expect( "Impossible EmptyListError" ),
                };

                chat_history.lock().unwrap().push( m );
            }

            // Add tool results to chat history
            for ( id, tool_result ) in tool_results 
            {
                let m = Message::User 
                {
                    content: OneOrMany::one(
                        UserContent::tool_result(
                            id,
                            OneOrMany::one(ToolResultContent::text(tool_result)),
                        )
                    )
                };

                chat_history.lock().unwrap().push( m );
            }

            if ! did_call_tool { break; }

            let last = chat_history.lock().unwrap().pop();            

            // Set the current prompt to the last message in the chat history
            current_prompt = match last
            {
                Some(prompt) => prompt,
                None => unreachable!("Chat history should never be empty at this point"),
            };
        }

    })) as _
}