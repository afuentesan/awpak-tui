use std::{process::Stdio, sync::Arc};

use rig::{agent::AgentBuilder, streaming::StreamingCompletionModel};
use rmcp::{model::Tool, service::RunningService, transport::TokioChildProcess, RoleClient, ServiceExt};
use tokio::process::Command;

use crate::domain::error::Error;

use super::{mcp::{MCPClient, NodeMCPServer}, mcp_client::mcp_client, tools::add_tool};

pub async fn add_clients_from_servers<M: StreamingCompletionModel>( 
    mut agent : AgentBuilder<M>,
    servers : &Vec<NodeMCPServer>
) -> Result<AgentBuilder<M>, Error>
{
    let mut idx = 0;

    for server in servers
    {
        ( agent, idx ) = add_client_from_server( agent, server, idx ).await?;
    }

    Ok( agent )
}

async fn add_client_from_server<M: StreamingCompletionModel>( 
    agent : AgentBuilder<M>, 
    server : &NodeMCPServer, 
    idx : usize
) -> Result<( AgentBuilder<M>, usize ), Error>
{
    match mcp_client( server ).await
    {
        Ok( c ) => Ok( add_client_tools( agent, c, idx )? ),
        Err( e ) => Err( Error::MCPToolErr( format!( "MCPToolErr: {}", e.to_string() ) ) )
    }
}

fn add_client_tools<M: StreamingCompletionModel>( 
    mut agent : AgentBuilder<M>, 
    client : MCPClient, 
    mut idx : usize 
) -> Result<( AgentBuilder<M>, usize ), Error>
{
    for tool in client.tools.into_iter()
    {
        idx = idx + 1;

        agent = add_tool( agent, client.service.clone(), tool, idx )?;
    }

    Ok( ( agent, idx ) )
}

pub async fn create_mcp_client( server : &NodeMCPServer ) -> Result<MCPClient, Error>
{
    let service = service( server ).await?;
    let tools: Vec<Tool> = service.list_all_tools().await.map_err( | e | Error::MCPToolErr( e.to_string() ) )?;

    let client = MCPClient
    {
        service : Arc::new( service ),
        tools 
    };

    Ok(
      client  
    )
    
}

async fn service( server : &NodeMCPServer ) -> Result<RunningService<RoleClient, ()>, Error>
{
    let service: RunningService<RoleClient, ()> = ()
    .serve(TokioChildProcess::new(
        &mut server_command( server ),
    ).map_err( | e | Error::MCPToolErr( e.to_string() ) )?)
    .await.map_err( | e | Error::MCPToolErr( e.to_string() ) )?;

    Ok( service )
}

fn server_command( server : &NodeMCPServer ) -> Command
{
    let mut command = Command::new( &server.command );

    server.arguments.iter().for_each( | a | { command.arg( a ); } );

    command.stdout( Stdio::null() ).stderr( Stdio::null() );

    server.env.iter()
    .for_each( | ( k, v ) | { command.env( k, v ); } );

    command
}