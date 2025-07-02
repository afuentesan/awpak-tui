use std::{collections::HashMap, fs::File, io::BufReader, sync::{Arc, Mutex, OnceLock}};

use awpak_ai::domain::graph::{build_graph::build_graph_from_path, graph::Graph};

use crate::{domain::{error::Error, graph::graph::{AwpakTUIGraph, GraphRequest}, path::path_utils::path_for_file}, infrastructure::config::model::graph_config::AwpakTUIGraphConfig};

const CONFIG_GRAPH_VAR : &'static str = "AWPAK_TUI_GRAPH";

fn initial_graphs() -> &'static Arc<Mutex<HashMap<String, Graph>>>
{
    static I : OnceLock<Arc<Mutex<HashMap<String, Graph>>>> = OnceLock::new();
    I.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

fn current_graphs() -> &'static Arc<Mutex<HashMap<String, Graph>>>
{
    static U : OnceLock<Arc<Mutex<HashMap<String, Graph>>>> = OnceLock::new();
    U.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn current_graph( initial_id : &str, current_id : &str ) -> Result<Graph, Error>
{
    if ! current_graphs().lock().unwrap().contains_key( current_id )
    {
        let initial = initial_graphs().lock().unwrap();
        let initial = initial.get( initial_id )
        .ok_or( Error::Graph( format!( "Graph {} not found.", initial_id ) ) )?;

        Ok( initial.clone() )
    }
    else
    {
        let mut lock = current_graphs().lock().unwrap();

        lock.remove( current_id ).ok_or( Error::Graph( format!( "Graph {} not found.", initial_id ) ) )    
    }
}

pub fn save_graph_in_current( id : &str, graph : Graph )
{
    let mut lock = current_graphs().lock().unwrap();

    lock.insert( id.to_string(), graph );
}

pub fn init_graphs_from_config() -> Vec<AwpakTUIGraph>
{
    match graph_config()
    {
        Some( g ) =>
        {
            g.into_iter()
            .flat_map( 
                | c | 
                {
                    let graph = build_graph_from_path( &c.path ).ok()?;

                    let g = graph_config_to_graph( c );

                    save_graph( g.initial_id.clone(), graph );

                    Some( g ) 
                }
            ).collect()
                    
        },
        _ => vec![]
    }
}

fn save_graph( id : String, graph : Graph )
{
    initial_graphs().lock().unwrap().insert( id, graph );
}

fn graph_config_to_graph( config : AwpakTUIGraphConfig ) -> AwpakTUIGraph
{
    AwpakTUIGraph 
    { 
        initial_id : uuid::Uuid::new_v4().into(),
        id : "".into(), 
        name : config.name, 
        request : GraphRequest::Empty,
        response : vec![]
    }
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
