use crate::{application::{chain::{chain::send_prompt_to_chain_client, chain_client::chain_client}, node::{node::send_prompt_to_node_client, node_client::{node_client, save_node_history}}}, domain::{agent::agent::AIAgent, chat::chat::{Chat, ChatChannel}, data::data_utils::option_value_to_string, error::Error}};


pub async fn send_propmt_to_chat<T>( chat : Chat, chat_channel : T )
where T: ChatChannel + Send + Sync
{
    if chat.request_value().is_none()
    {
        return;
    }

    proccess_send_prompt_to_chat( chat, chat_channel.clone() ).await;

    chat_channel.end_chat();
}

async fn proccess_send_prompt_to_chat<T>( chat : Chat, chat_channel : T )
where T: ChatChannel + Send + Sync
{
    let handle = tokio::runtime::Handle::current();

    let _ = std::thread::spawn( move ||
        {
            handle.block_on( async move
                {
                    let result = match chat.agent()
                    {
                        AIAgent::Node( _ ) => 
                        {
                            send_propmt_to_node_chat( chat, chat_channel.clone() ).await
                        },
                        AIAgent::Chain( _ ) =>
                        {
                            send_prompt_to_chain_chat( chat, chat_channel.clone() ).await
                        },
                        AIAgent::Repeat( _ ) =>
                        {
                            todo!()
                        }
                    };

                    match result
                    {
                        Ok( _ ) => {},
                        Err( e ) =>
                        {
                            let _ = chat_channel.send_str( e.to_string().as_str() );
                        }
                    }
                }
            );
        }
    ).join();
}

async fn send_prompt_to_chain_chat<T>( chat : Chat, chat_channel : T ) -> Result<(), Error>
where T: ChatChannel + Send + Sync
{
    let ( chat, agent ) = chat.own_agent();

    let ( _, chain ) = agent.own_chain();

    let chain = chain.ok_or( Error::AgentErr( "AgentErr: Chain not found in agent".into() ) )?;

    let id = chat.id();

    let prompt = option_value_to_string( chat.request_value() );

    let chain_client = chain_client( id, &chain ).await?;
    
    let _ = send_prompt_to_chain_client( 
        chain_client, 
        prompt.as_str(), 
        serde_json::Value::Null,
        chat_channel
    ).await?;

    Ok( () )
}

async fn send_propmt_to_node_chat<T>( chat : Chat, chat_channel : T ) -> Result<(), Error>
where T: ChatChannel
{
    let ( chat, agent ) = chat.own_agent();

    let ( _, node ) = agent.own_node();

    let node = node.ok_or( Error::AgentErr( "AgentErr: Node not found in agent".into() ) )?;

    let id = chat.id();

    let prompt = option_value_to_string( chat.request_value() );

    let node_client = node_client( id, &node ).await?;

    let ( _, history ) = send_prompt_to_node_client( 
        node_client, 
        prompt.as_str(), 
        chat_channel
    ).await?;

    save_node_history( id )( history );

    Ok( () )
}