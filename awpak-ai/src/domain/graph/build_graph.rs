use std::{collections::HashMap, fs::File, io::BufReader};

use async_recursion::async_recursion;
use awpak_utils::file_utils::path_for_file;

use crate::domain::{error::Error, graph::{build_graph_node::graph_node_executor_from_config, graph::{Graph, GraphConfig}, node::{Node, NodeConfig, NodeExecutor, NodeExecutorConfig}}, store::{store::{Store, StoreConfig}, store_from_config::store_from_config}};

pub async fn build_graph_from_str( str : impl AsRef<str> ) -> Result<Graph, Error>
{
    let config : GraphConfig = serde_json::from_str( str.as_ref() ).map_err( | e | Error::ParseData( e.to_string() ) )?;

    build_graph( config ).await
}

#[async_recursion]
pub async fn build_graph_from_path( path : &str ) -> Result<Graph, Error>
{
    let path = path_for_file( path ).map_err( | e | Error::File( e.to_string() ) )?;

    let file = File::open( path ).map_err( | e | Error::File( e.to_string() ) )?;
    let reader = BufReader::new( file );

    let config : GraphConfig = serde_json::from_reader(reader ).map_err( | e | Error::File( e.to_string() ) )?;

    build_graph( config ).await
}

pub async fn build_graph(
    config : GraphConfig
) -> Result<Graph, Error>
{
    let ( first, nodes ) = build_nodes( config.first, config.nodes ).await?;

    let stores = init_stores( config.stores ).await?;
    Ok(
        Graph::new(
            stores,
            config.input_type,
            config.context,
            first,
            nodes,
            config.preserve_context
        )
    )
}

async fn init_stores( config : Vec<StoreConfig> ) -> Result<HashMap<String, Store>, Error>
{
    let mut ret = HashMap::new();

    for c in config
    {
        ret.insert( c.id.clone(), store_from_config( c ).await? );
    }

    Ok( ret )
}

async fn node_from_config( config : NodeConfig ) -> Result<Node, Error>
{
    let executor = node_executor_from_config( config.executor ).await?;

    Ok(
        Node
        {
            id : config.id,
            executor,
            destination : config.destination,
            output : config.output
        }
    )
}

async fn node_executor_from_config( config : NodeExecutorConfig ) -> Result<NodeExecutor, Error>
{
    match config
    {
        NodeExecutorConfig::Agent( a ) => Ok( NodeExecutor::Agent( a ) ),
        NodeExecutorConfig::AgentHistoryMut( a ) => Ok( NodeExecutor::AgentHistoryMut( a ) ),
        NodeExecutorConfig::Command( c ) => Ok( NodeExecutor::Command( c ) ),
        NodeExecutorConfig::ContextMut( c ) => Ok( NodeExecutor::ContextMut( c ) ),
        NodeExecutorConfig::Graph( g ) => graph_node_executor_from_config( g ).await,
        NodeExecutorConfig::Parallel( p ) => Ok( NodeExecutor::Parallel( p ) ),
        NodeExecutorConfig::WebClient( w ) => Ok( NodeExecutor::WebClient( w ) )
    }
}

async fn build_nodes( first : NodeConfig, config_nodes : Vec<NodeConfig> ) -> Result<( String, HashMap<String, Node> ), Error>
{
    let first = node_from_config( first ).await?;

    let first_str = first.id.clone();

    let mut nodes = vec![];

    for n in config_nodes
    {
        nodes.push( node_from_config( n ).await? );
    }

    let mut ret = HashMap::new();

    ret.insert( first.id.clone(), first );

    for n in nodes 
    { 
        let id = n.id.clone();

        match ret.insert( id.clone(), n )
        {
            Some( _ ) => return Err( Error::NodeExists( id ) ),
            None => continue
        }
    }

    Ok( ( first_str, ret ) )
}

