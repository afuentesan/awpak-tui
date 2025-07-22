use std::{collections::HashMap, fs::File, io::BufReader};

use awpak_utils::file_utils::path_for_file;

use crate::domain::{error::Error, graph::{graph::{Graph, GraphConfig}, build_graph_node::graph_node_from_config, node::{Node, NodeConfig}}};

pub fn build_graph_from_str( str : impl AsRef<str> ) -> Result<Graph, Error>
{
    let config : GraphConfig = serde_json::from_str( str.as_ref() ).map_err( | e | Error::ParseData( e.to_string() ) )?;

    build_graph( config )
}

pub fn build_graph_from_path( path : &str ) -> Result<Graph, Error>
{
    let path = path_for_file( path ).map_err( | e | Error::File( e.to_string() ) )?;

    let file = File::open( path ).map_err( | e | Error::File( e.to_string() ) )?;
    let reader = BufReader::new( file );

    let config : GraphConfig = serde_json::from_reader(reader ).map_err( | e | Error::File( e.to_string() ) )?;

    build_graph( config )
}

pub fn build_graph(
    config : GraphConfig
) -> Result<Graph, Error>
{
    let ( first, nodes ) = build_nodes( config.first, config.nodes )?;

    Ok(
        Graph::new(
        
            config.input_type,
            config.context,
            first,
            nodes,
            config.preserve_context
        )
    )
}

fn node_from_config( config : NodeConfig ) -> Result<Node, Error>
{
    match config
    {
        NodeConfig::Node( n ) => Ok( n ),
        NodeConfig::Graph( c ) => graph_node_from_config( c )
    }
}

fn build_nodes( first : NodeConfig, config_nodes : Vec<NodeConfig> ) -> Result<( String, HashMap<String, Node> ), Error>
{
    let first = node_from_config( first )?;

    let first_str = first.id.clone();

    let mut nodes = vec![];

    for n in config_nodes
    {
        nodes.push( node_from_config( n )? );
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

