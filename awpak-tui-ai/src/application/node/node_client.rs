use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use rig::message::Message;

use crate::domain::{error::Error, node::{node::Node, node_client::NodeClient, node_functions::create_node_client}};

// MANAGE NODE CLIENTS

pub async fn node_client( id : &str, node : &Node ) -> Result<NodeClient, Error>
{
    match node_existing_client( id )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_node_client( id, node ).await?;

            node_clients().lock().unwrap().insert( id.to_string(), client.clone() );

            Ok( client )
        }
    }
}

fn node_clients() -> &'static Arc<Mutex<HashMap<String, NodeClient>>>
{
    static NODE_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, NodeClient>>>> = OnceLock::new();
    NODE_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

fn node_existing_client( id : &str ) -> Result<NodeClient, Error>
{
    match node_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

// END MANAGE NODE CLIENTS

// NODE HISTORY

pub fn save_node_history( id : &str ) -> Box<dyn FnOnce( Vec<Message> )>
{
    let id = id.to_string();

    Box::new(
        move | h |
        {
            fn_save_node_history( id, h )
        }
    )
}

fn fn_save_node_history( id : String, history : Vec<Message> )
{
    match node_existing_client( id.as_str() )
    {
        Ok( c ) => save_history( id, c, history ),
        _ => {}
    }
}

fn save_history( id : String, mut client : NodeClient, new_history : Vec<Message> )
{
    client.history = if client.save_history { new_history } else { client.history };

    node_clients().lock().unwrap().insert( id, client );
}

// END NODE HISTORY