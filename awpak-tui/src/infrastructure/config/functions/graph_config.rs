use std::{collections::HashMap, fs::File, io::BufReader, sync::{Arc, Mutex, OnceLock}};

use awpak_ai::domain::graph::{build_graph::build_graph_from_path, graph::Graph};

use crate::{domain::{error::Error, graph::graph::{AwpakTUIGraph, GraphRequest}, path::path_utils::path_for_file}, infrastructure::config::model::graph_config::{AwpakTUIGraphConfig, AwpakTUIGraphOutputConfig}};

const CONFIG_GRAPH_VAR : &'static str = "AWPAK_TUI_GRAPH";

fn graph_outputs() -> &'static Arc<Mutex<HashMap<String, AwpakTUIGraphOutputConfig>>>
{
    static O : OnceLock<Arc<Mutex<HashMap<String, AwpakTUIGraphOutputConfig>>>> = OnceLock::new();
    O.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

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

fn current_initial_id_relation() -> &'static Arc<Mutex<HashMap<String, String>>>
{
    static R : OnceLock<Arc<Mutex<HashMap<String, String>>>> = OnceLock::new();
    R.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn current_graph( initial_id : &str, current_id : &str ) -> Result<Graph, Error>
{
    if ! current_graphs().lock().unwrap().contains_key( current_id )
    {
        let initial = initial_graphs().lock().unwrap();
        let initial = initial.get( initial_id )
        .ok_or( Error::Graph( format!( "Graph {} not found in initial.", initial_id ) ) )?;

        current_graphs().lock().unwrap()
        .insert( current_id.to_string(), initial.clone() );
    }
    
    let lock = current_graphs().lock().unwrap();

    let graph = lock.get( current_id ).ok_or( Error::Graph( format!( "Graph {} not found in current.", current_id ) ) )?;

    current_initial_id_relation().lock().unwrap().insert( current_id.to_string(), initial_id.to_string() );

    Ok( graph.clone() )
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

                    let graph_output = c.output.clone();

                    let g = graph_config_to_graph( c );

                    save_graph_output( g.initial_id.clone(), graph_output );

                    save_graph( g.initial_id.clone(), graph );

                    Some( g ) 
                }
            ).collect()
                    
        },
        _ => vec![]
    }
}

fn save_graph_output( id : String, output : AwpakTUIGraphOutputConfig )
{
    graph_outputs().lock().unwrap().insert( id, output );
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


pub fn graph_output_config( id : &str ) -> Option<AwpakTUIGraphOutputConfig>
{
    match current_initial_id_relation().lock().unwrap().get( id )
    {
        Some( id ) => graph_output_from_initial_id( id ),
        _ => None
    }
}

fn graph_output_from_initial_id( initial_id : &str ) -> Option<AwpakTUIGraphOutputConfig>
{
    match graph_outputs().lock().unwrap().get( initial_id )
    {
        Some( c ) => Some( c.clone() ),
        _ => None
    }
}