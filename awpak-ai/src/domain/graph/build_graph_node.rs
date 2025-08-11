use crate::domain::{error::Error, graph::{build_graph::build_graph_from_path, graph_node::GraphNode, node::{GraphNodeConfig, NodeExecutor}}};


pub async fn graph_node_executor_from_config( 
    config : GraphNodeConfig
) -> Result<NodeExecutor, Error>
{
    let graph = build_graph_from_path( &config.path ).await?;

    Ok(
        NodeExecutor::Graph(
            GraphNode
            {
                graph,
                input : config.input,
                output : config.output
            }
        )
    )
}