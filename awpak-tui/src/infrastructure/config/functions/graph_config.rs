use std::{fs::File, io::BufReader};

use awpak_ai::domain::graph::build_graph::build_graph_from_path;

use crate::{domain::{graph::graph::{AwpakTUIGraph, GraphRequest}, path::path_utils::path_for_file}, infrastructure::config::model::graph_config::AwpakTUIGraphConfig};

const CONFIG_GRAPH_VAR : &'static str = "AWPAK_TUI_GRAPH";

pub fn init_graphs_from_config() -> Vec<AwpakTUIGraph>
{
    match graph_config()
    {
        Some( g ) =>
        {
            g.into_iter().flat_map( | c | graph_config_to_graph( c ) ).collect()
        },
        _ => vec![]
    }
}

fn graph_config_to_graph( config : AwpakTUIGraphConfig ) -> Option<AwpakTUIGraph>
{
    let graph = build_graph_from_path( &config.path ).ok()?;

    Some(
        AwpakTUIGraph 
        { 
            id : uuid::Uuid::new_v4().into(), 
            name : config.name, 
            graph : graph,
            request : GraphRequest::Empty,
            response : vec![]
        }
    )
}

fn graph_config() -> Option<Vec<AwpakTUIGraphConfig>>
{
    match std::env::var( CONFIG_GRAPH_VAR )
    {
        Ok( v ) => graph_config_from_path( v ),
        _ => None
    }
}

fn graph_config_from_path( path : String ) -> Option<Vec<AwpakTUIGraphConfig>>
{
    let path = path_for_file( path.as_str() ).ok()?;

    let file = File::open( path ).map_err( | _ | () ).ok()?;
    let reader = BufReader::new( file );

    serde_json::from_reader(reader ).ok()
}
