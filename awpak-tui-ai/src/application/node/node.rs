use std::sync::Arc;

use rig::{agent::Agent, streaming::StreamingCompletionModel};

use crate::domain::{chat::chat::ChatChannel, error::Error, node::{node_client::{NodeClient, NodeClientProvider}, node_functions::send_prompt_to_node}};

use super::node_client::save_node_history;

pub async fn send_prompt_to_node_client<T>( 
    client : NodeClient, 
    prompt : &str,
    chat_channel : T
) -> Result<String, Error>
where T: ChatChannel + Send + Sync
{
    match &client.provider
    {
        NodeClientProvider::Ollama( a ) =>
        {
            let a = a.clone();

            send_prompt( client, prompt, a, chat_channel ).await
        },
        NodeClientProvider::OpenAI( a ) =>
        {
            let a = a.clone();

            send_prompt( client, prompt, a, chat_channel ).await
        }   
    }
}

async fn send_prompt<T: StreamingCompletionModel, U: ChatChannel + Send + Sync>(
    client : NodeClient,
    prompt : &str,
    agent : Arc<Agent<T>>,
    chat_channel : U
) -> Result<String, Error>
{
    let chat_history = client.history;
    
    let ( out, history ) = send_prompt_to_node( prompt, chat_history, chat_channel, agent, &client.output ).await?;

    save_node_history( &client.id )( history );

    Ok( out )
}

