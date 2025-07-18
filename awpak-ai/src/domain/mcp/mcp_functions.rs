

use mcp_core::{client::ClientBuilder, transport::ClientStdioTransport};
use rig::{agent::AgentBuilder, completion::CompletionModel};

use crate::domain::{command::command_input::command_args, error::Error, graph::graph::Graph};

use super::mcp::NodeMCPServer;

pub async fn add_mcp_clients_to_agent<M: CompletionModel>( 
    graph : &Graph,
    mut agent : AgentBuilder<M>,
    servers : &Vec<NodeMCPServer>
) -> Result<AgentBuilder<M>, Error>
{
    for server in servers
    {
        agent = add_server_to_agent( graph, agent, server ).await?;
    }

    Ok( agent )
}

async fn add_server_to_agent<T: CompletionModel>( 
    graph : &Graph,
    agent_builder : AgentBuilder<T>, 
    server : &NodeMCPServer 
) -> Result<AgentBuilder<T>, Error>
{
    let client = client_from_server( graph, server ).await?;

    let tools = client.list_tools( None, None ).await.map_err( | e | Error::MCPTool( e.to_string() ) )?.tools;

    let agent_builder = tools
    .into_iter().skip( 5 )
    .fold(
        agent_builder, 
        | builder, tool |
        {
            builder.mcp_tool(tool, client.clone().into())
        }
    );

    Ok( agent_builder )
}

async fn client_from_server( 
    graph : &Graph,
    server : &NodeMCPServer 
) -> Result<mcp_core::client::Client<ClientStdioTransport>, Error>
{
    let arguments = command_args( graph, &server.arguments )?;

    let mcp_client = ClientBuilder::new( 
        ClientStdioTransport::new( 
            &server.command, 
            arguments.iter().map( | s | s.as_str() ).collect::<Vec<_>>().as_slice()
        ).map_err( | e | Error::MCPTool( e.to_string() ) )? 
    ).build();

    mcp_client.open().await.map_err( | e | Error::MCPTool( e.to_string() ) )?;

    Ok( mcp_client )
}