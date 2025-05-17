use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use rig::message::Message;

use crate::domain::{chain::{chain::Chain, chain_client::{ChainClient, ChainClientProvider}, chain_functions::create_chain_client}, error::Error};

// MANAGE CHAIN CLIENTS

pub async fn chain_client( id : &str, chain : &Chain ) -> Result<ChainClient, Error>
{
    match chain_existing_client( id )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_chain_client( id, chain ).await?;

            chain_clients().lock().unwrap().insert( id.to_string(), client.clone() );

            Ok( client )
        }
    }
}

fn chain_clients() -> &'static Arc<Mutex<HashMap<String, ChainClient>>>
{
    static CHAIN_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, ChainClient>>>> = OnceLock::new();
    CHAIN_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

fn chain_existing_client( id : &str ) -> Result<ChainClient, Error>
{
    match chain_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

// END MANAGE CHAIN CLIENTS

// SAVE NODE_CHAIN HISTORY

pub fn save_node_chain_history(
    client_id : &str,
    item_id : &str,
    node_id : &str,
    new_history : Vec<Message>
)
{
    match chain_existing_client( client_id )
    {
        Ok( c ) => save_node_chain_item_history( c, item_id, node_id, new_history ),
        _ => {}
    }
}

fn save_node_chain_item_history( 
    client : ChainClient,
    item_id : &str,
    node_id : &str,
    new_history : Vec<Message>
)
{
    match client.items.iter().enumerate()
    .find( | ( _, c ) | c.id == item_id )
    .map( | ( i, _ ) | i )
    {
        Some( i ) => save_node_chain_idx_history( client, i, node_id, new_history ),
        None => {}
    }
}

fn save_node_chain_idx_history(
    client : ChainClient,
    idx : usize,
    node_id : &str,
    new_history : Vec<Message>
)
{
    match &client.items[ idx ].provider
    {
        ChainClientProvider::Node( n ) if n.id == node_id =>
        {
            save_node_chain_node_history( client, idx, new_history )
        },
        _ => {}
    }
}

fn save_node_chain_node_history(
    mut client : ChainClient,
    idx : usize,
    new_history : Vec<Message>
)
{
    match &mut client.items[ idx ].provider
    {
        ChainClientProvider::Node( n ) =>
        {
            if n.save_history
            {
                n.history = new_history;
            }
        },
        _ => unreachable!()
    };

    chain_clients().lock().unwrap().insert( client.id.clone(), client );
}

// END SAVE NODE_CHAIN HISTORY