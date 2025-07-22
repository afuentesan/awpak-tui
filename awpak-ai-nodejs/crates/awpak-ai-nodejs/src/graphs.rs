
use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use awpak_ai::{domain::{error::Error, graph::graph::Graph}, infrastructure::graph::build_graph::graph_from_json_file_path};

pub fn graphs() -> &'static Arc<Mutex<HashMap<String, Graph>>>
{
    static U : OnceLock<Arc<Mutex<HashMap<String, Graph>>>> = OnceLock::new();
    U.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn save_graph( id : &str, graph : Graph )
{
    graphs().lock().unwrap().insert( id.to_string(), graph );
}

pub fn graph( id : &str, path : &str ) -> Result<Graph, Error>
{
    let lock = graphs().lock().unwrap();

    let g = lock.get( id );

    match g
    {
        Some( g ) => 
        {
            let g = g.clone();

            drop( lock );

            Ok( g )
        },
        None => 
        {
            drop( lock );
            
            new_graph( id, path )
        }
    }
}

fn new_graph( id : &str, path : &str ) -> Result<Graph, Error>
{
    match graph_from_json_file_path( path )
    {
        Ok( g ) =>
        {
            let mut lock = graphs().lock().unwrap();

            lock.insert( id.to_string(), g.clone() );

            drop( lock );

            Ok( g )
        },
        e => e 
    }
}