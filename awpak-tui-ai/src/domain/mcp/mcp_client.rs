use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use crate::domain::{error::Error, mcp::{mcp::{MCPClient, NodeMCPServer}, mcp_functions::create_mcp_client}};

pub async fn mcp_client( server : &NodeMCPServer ) -> Result<MCPClient, Error>
{
    let id = id_from_server( server );

    match mcp_existing_client( id.as_str() )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_mcp_client( server ).await?;

            mcp_clients().lock().unwrap().insert( id, client.clone() );

            Ok( client )
        }
    }
}

fn mcp_existing_client( id : &str ) -> Result<MCPClient, Error>
{
    match mcp_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

fn mcp_clients() -> &'static Arc<Mutex<HashMap<String, MCPClient>>>
{
    static MCP_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, MCPClient>>>> = OnceLock::new();
    MCP_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

fn id_from_server( server : &NodeMCPServer ) -> String
{
    format!(
        "{}_{}_{}",
        server.command,
        server.arguments.iter().map( | s | s.clone() ).collect::<Vec<_>>().join( "_" ),
        server.env.iter().map( | ( k, v ) | format!( "{}_{}", k, v ) ).collect::<Vec<_>>().join( "_" )
    )
}