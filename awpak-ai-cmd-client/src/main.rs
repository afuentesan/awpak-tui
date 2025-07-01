use std::{io::Write as _, sync::mpsc::{self}};

use awpak_ai::{domain::{error::Error, graph::graph::Graph, tracing::filter_layer::{AwpakAIFilterLayer, AwpakAITarget}}, infrastructure::graph::{build_graph::graph_from_path, run_graph::run_graph}};
use text_io::read;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() -> Result<(), ()>
{
    if std::env::args().len() < 2
    {
        eprintln!( "Graph path not found" );

        return Err( () );
    }

    subscribe_tracing();

    let path = std::env::args().nth( 1 ).ok_or( Error::Ignore ).map_err( | _ | () )?;

    let mut graph = match graph_from_path( &path )
    {
        Ok( g ) => g,
        Err( e ) =>
        {
            eprintln!( "Build graph error: {:?}", e );

            return Err( () );
        }
    };

    loop
    {
        let input = user_input();

        if input.trim() == "" { continue; }

        if input.trim() == "exit" { break; }

        graph = execute_graph( graph, input ).await;
    }
    
    Ok( () )
}

fn subscribe_tracing()
{
    let ( tx, rx ) = mpsc::channel::<String>();

    let layer = AwpakAIFilterLayer 
    {
        allowed : vec![ 
            // ( AwpakAITarget::AgentStream, tx.clone() ),
            ( AwpakAITarget::AgentToolCall, tx.clone() ),
            ( AwpakAITarget::AgentToolResult, tx )
        ],
    };

    tracing_subscriber::registry()
        .with(layer)
        .init();

    std::thread::spawn( move || 
        {
            loop
            {
                match rx.recv()
                {
                    Ok( s ) => 
                    {
                        print!( "{}", s );
                        let _ = std::io::stdout().flush();
                    },
                    _ => break    
                }
            }
            
        }
    );
}

fn user_input() -> String
{
    print!( "Prompt: " );
    read!( "{}\n" )
}

async fn execute_graph( graph : Graph, input : String ) -> Graph
{
    let result = run_graph( input, graph ).await;

    match result.collect()
    {
        ( g, None ) => 
        {
            show_graph_result( &g );

            g
        },
        ( g, Some( e ) ) => 
        {
            println!( "Error: {}", e.to_string() );

            g
        }
    }
}

fn show_graph_result( graph : &Graph )
{
    match &graph.final_output
    {
        Some( o ) => match o
        {
            Ok( m ) => println!( "ExitOk: {}", m ),
            Err( e ) => println!( "ExitErr: {}", e )
        },
        None => println!( "End graph execution without output" )
    }
}