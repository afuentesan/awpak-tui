use rig::{agent::AgentBuilder, completion::CompletionModel};
use rmcp::{transport::{ConfigureCommandExt, TokioChildProcess}, ServiceExt};
use tokio::process::Command;

use crate::domain::{command::command_input::command_args, error::Error, graph::graph::Graph};

use super::mcp::NodeMCPServer;

pub async fn add_mcp_clients_to_agent<M: CompletionModel>( 
    graph : &Graph,
    mut agent : AgentBuilder<M>,
    servers : &Vec<NodeMCPServer>
) -> Result<( AgentBuilder<M>, Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> ), Error>
{
    let mut clients : Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> = vec![];

    for server in servers
    {
        let client;

        ( agent, client ) = add_server_to_agent( graph, agent, server ).await?;

        clients.push( client );
    }

    Ok( ( agent, clients ) )
}

async fn add_server_to_agent<T: CompletionModel>(
    graph : &Graph,
    agent_builder : AgentBuilder<T>, 
    server : &NodeMCPServer 
) -> Result<( AgentBuilder<T>, rmcp::service::RunningService<rmcp::RoleClient, ()> ), Error>
{
    let client: rmcp::service::RunningService<rmcp::RoleClient, ()> = client_from_server( graph, server ).await?;

    let tools = client.list_all_tools().await.map_err( | e | Error::MCPTool( e.to_string() ) )?;
    
    let agent_builder = tools
    .into_iter()
    .fold(
        agent_builder, 
        | builder, tool |
        {
            builder.rmcp_tool( tool, client.clone() )
        }
    );

    Ok( ( agent_builder, client ) )
}

async fn client_from_server( 
    graph : &Graph,
    server : &NodeMCPServer 
) -> Result<rmcp::service::RunningService<rmcp::RoleClient, ()>, Error>
{    
    let arguments = command_args( graph, &server.arguments )?;

    let child_process = tokio_child_process( server, arguments )?;

    let client = ()
    .serve( child_process )
    .await.map_err( | e | Error::MCPTool( e.to_string() ) )?;

    Ok( client )
}

fn tokio_child_process( server : &NodeMCPServer, arguments : Vec<String> ) -> Result<TokioChildProcess, Error>
{
    TokioChildProcess::new( mcp_command( server, arguments ) ).map_err( | e | Error::MCPTool( e.to_string() ) )
}

fn mcp_command( server : &NodeMCPServer, arguments : Vec<String> ) -> Command
{
    Command::new(
        &server.command
    )
    .configure(
        |cmd| {
            arguments.iter().for_each( | a | { cmd.arg( a ); } );
        }
    )
}