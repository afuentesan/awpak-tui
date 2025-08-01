use std::{pin::Pin, sync::{Arc, Mutex}};

use rig::{agent::Agent, completion::{CompletionModel, Prompt}, message::{AssistantContent, Message, Text, ToolResultContent, UserContent}, streaming::{StreamedAssistantContent, StreamingCompletion}, OneOrMany};
use tokio_stream::{Stream, StreamExt};
use tracing::info;
type StreamingResult = Pin<Box<dyn Stream<Item = Result<Text, Error>> + Send>>;
use crate::domain::{agent::agent::AIAgent, error::Error, signals::cancel_graph::is_graph_cancelled, tracing::filter_layer::{AGENT_STREAM, AGENT_SYNC, AGENT_TOOL_CALL, AGENT_TOOL_RESULT}, utils::string_utils::option_string_to_str};

pub async fn run_agent<T: CompletionModel + 'static>( 
    id : Option<&String>,
    prompt : String, 
    provider : Agent<T>, 
    agent : &AIAgent 
) -> Result<( String, Vec<Message> ), Error>
{
    match agent.is_stream
    {
        true => run_stream_agent( id, prompt, provider, agent ).await,
        false => run_sync_agent( id, prompt, provider, agent ).await
    }
}

pub async fn run_stream_agent<T: CompletionModel + 'static>( 
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

async fn run_sync_agent<T: CompletionModel + 'static>( 
    id : Option<&String>,
    prompt : String, 
    provider : Agent<T>, 
    agent : &AIAgent 
) -> Result<( String, Vec<Message> ), Error>
{
    let mut history = agent.history.clone();

    let response = provider
    .prompt( prompt )
    .multi_turn( 
        match agent.servers.len()
        {
            0 => 0,
            _ => agent.turns.unwrap_or( 25 )
        }
    )
    .with_history( &mut history )
    .await.map_err( | e | Error::Agent( e.to_string() ) )?;

    info!( target:AGENT_SYNC, id=option_string_to_str( id ), text=response );
    
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
                info!( target:AGENT_STREAM, id=option_string_to_str( id ), text=text );

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
    chat_history: Arc<Mutex<Vec<Message>>>
) -> StreamingResult
where
    M: CompletionModel + 'static,
    <M as CompletionModel>::StreamingResponse: std::marker::Send,
{
    let prompt: Message = prompt.into();

    (Box::pin(async_stream::stream! {
        let mut current_prompt = prompt;
        let mut did_call_tool = false;

        'outer: loop {

            let h = chat_history.lock().unwrap().clone();

            let mut stream = agent
                .stream_completion(current_prompt.clone(), h )
                .await.map_err( | e | Error::Agent( e.to_string() ) )?
                .stream()
                .await.map_err( | e | Error::Agent( e.to_string() ) )?;

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
                    Ok(StreamedAssistantContent::Text(text)) => {
                        yield Ok(Text { text: text.text });
                        did_call_tool = false;
                    },
                    Ok(StreamedAssistantContent::ToolCall(tool_call)) => {

                        info!( 
                            target:AGENT_TOOL_CALL,  
                            id=option_string_to_str( id.as_ref() ),
                            text=format!( 
                                "\nToolCall:\n- Function name: {}\n- Args: {:?}\n", 
                                tool_call.function.name,
                                tool_call.function.arguments
                            )
                        );

                        let tool_result =
                            agent.tools.call(&tool_call.function.name, tool_call.function.arguments.to_string())
                            .await.map_err( | e | Error::MCPTool( e.to_string() ) )?;

                        info!( 
                            target:AGENT_TOOL_RESULT, 
                            id=option_string_to_str( id.as_ref() ),
                            text=format!(
                                "\nTool result:\n{}\n",
                                tool_result
                            ) 
                        );

                        let tool_call_msg = AssistantContent::ToolCall(tool_call.clone());

                        tool_calls.push(tool_call_msg);
                        tool_results.push((tool_call.id, tool_call.call_id, tool_result));

                        did_call_tool = true;
                        // break;
                    },
                    Ok(StreamedAssistantContent::Reasoning(rig::message::Reasoning { reasoning })) => {
                        yield Ok(Text { text: reasoning });
                        did_call_tool = false;
                    },
                    Ok(_) => {
                        // do nothing here as we don't need to accumulate token usage
                    }
                    Err(e) => {
                        yield Err( Error::Agent( e.to_string() ) );
                        break 'outer;
                    }
                }
            }

            // Add (parallel) tool calls to chat history
            if !tool_calls.is_empty() {
                chat_history.lock().unwrap().push(Message::Assistant {
                    id: None,
                    content: OneOrMany::many(tool_calls).expect("Impossible EmptyListError"),
                });
            }

            // Add tool results to chat history
            for (id, call_id, tool_result) in tool_results {
                if let Some(call_id) = call_id {
                    chat_history.lock().unwrap().push(Message::User {
                        content: OneOrMany::one(UserContent::tool_result_with_call_id(
                            id,
                            call_id,
                            OneOrMany::one(ToolResultContent::text(tool_result)),
                        )),
                    });
                } else {
                    chat_history.lock().unwrap().push(Message::User {
                        content: OneOrMany::one(UserContent::tool_result(
                            id,
                            OneOrMany::one(ToolResultContent::text(tool_result)),
                        )),
                    });

                }

            }

            if ! did_call_tool { break; }

            let last = chat_history.lock().unwrap().pop();

            // Set the current prompt to the last message in the chat history
            current_prompt = match last
            {
                Some(prompt) => prompt,
                None => unreachable!("Chat history should never be empty at this point"),
            };

            // // Set the current prompt to the last message in the chat history
            // current_prompt = match chat_history.pop() {
            //     Some(prompt) => prompt,
            //     None => unreachable!("Chat history should never be empty at this point"),
            // };

            // if !did_call_tool {
            //     break;
            // }
        }

    })) as _
}