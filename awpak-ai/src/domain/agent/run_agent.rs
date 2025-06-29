use std::{pin::Pin, sync::{Arc, Mutex}};

use rig::{agent::Agent, completion::CompletionModel, message::{AssistantContent, Message, Text, ToolResultContent, UserContent}, streaming::StreamingCompletion, OneOrMany};
use tokio_stream::{Stream, StreamExt};
type StreamingResult = Pin<Box<dyn Stream<Item = Result<Text, Error>> + Send>>;
use crate::domain::{agent::agent::AIAgent, error::Error};

pub async fn run_agent<T: CompletionModel + 'static>( 
    prompt : String, 
    provider : Agent<T>, 
    agent : &AIAgent 
) -> Result<( String, Vec<Message> ), Error>
{
    let chat_history = Arc::new( Mutex::new( agent.history.clone() ) );
    let chat_history_result = chat_history.clone();

    let mut result = stream_chat( 
        provider, 
        prompt, 
        chat_history
    ).await;
    
    let response = string_from_stream( &mut result ).await?;

    let mut history = match chat_history_result.lock()
    {
        Ok( mut h ) => h.split_off( 0 ),
        Err( mut h ) => h.get_mut().split_off( 0 )
    };

    history.push( Message::assistant( response.clone() ) );

    Ok( ( response, history ) )
}

async fn string_from_stream(
    stream: &mut StreamingResult
) -> Result<String, Error>
{
    let mut ret = String::new();

    while let Some( content ) = stream.next().await
    {
        match content 
        {
            Ok( Text { text } ) => 
            {
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
            let h = match chat_history.lock()
            {
                Ok( h ) => h.clone(),
                Err( h ) => h.into_inner().clone()
            };

            let mut stream = agent
                .stream_completion(current_prompt.clone(), h )
                .await
                .map_err( | e | Error::Agent( e.to_string() ) )?
                .stream()
                .await
                .map_err( | e | Error::Agent( e.to_string() ) )?;

            match chat_history.lock()
            {
                Ok( mut h ) => h.push( current_prompt.clone() ),
                Err( mut h ) => h.get_mut().push( current_prompt.clone() )
            };
            // chat_history.push( current_prompt.clone() );

            let mut tool_calls = vec![];
            let mut tool_results = vec![];

            while let Some(content) = stream.next().await 
            {
                match content {
                    Ok(AssistantContent::Text(text)) => {
                        yield Ok(Text { text: text.text });
                        did_call_tool = false;
                    },
                    Ok(AssistantContent::ToolCall(tool_call)) => {
                        let tool_result =
                            agent.tools.call(&tool_call.function.name, tool_call.function.arguments.to_string())
                            .await
                            .map_err( | e | Error::MCPTool( e.to_string() ) )?;

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

                match chat_history.lock()
                {
                    Ok( mut h ) => h.push( m ),
                    Err( mut h ) => h.get_mut().push( m )
                };

                // chat_history.lock().unwrap().push(Message::Assistant {
                //     content: OneOrMany::many(tool_calls).expect("Impossible EmptyListError"),
                // });
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

                match chat_history.lock()
                {
                    Ok( mut h ) => h.push( m ),
                    Err( mut h ) => h.get_mut().push( m )
                };

                // chat_history.push(Message::User {
                //     content: OneOrMany::one(UserContent::tool_result(
                //         id,
                //         OneOrMany::one(ToolResultContent::text(tool_result)),
                //     )),
                // });
            }

            if ! did_call_tool { break; }

            let last = match chat_history.lock()
            {
                Ok( mut h ) => h.pop(),
                Err( mut h ) => h.get_mut().pop()
            };

            // Set the current prompt to the last message in the chat history
            current_prompt = match last
            {
                Some(prompt) => prompt,
                None => unreachable!("Chat history should never be empty at this point"),
            };
        }

    })) as _
}