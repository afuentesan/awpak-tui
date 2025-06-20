use crate::domain::{error::Error, graph::{build_graph::build_graph_from_path, graph_node::GraphNode, node::{GraphNodeConfig, Node, NodeExecutor}}};


pub fn graph_node_from_config( 
    config : GraphNodeConfig
) -> Result<Node, Error>
{
    let graph = build_graph_from_path( &config.path )?;

    Ok(
        Node 
        { 
            id : config.id, 
            executor : NodeExecutor::Graph(
                GraphNode
                {
                    graph,
                    input : config.input,
                    output : config.output
                }
            ), 
            output : config.node_output, 
            destination : config.node_destination
        }
    )
}