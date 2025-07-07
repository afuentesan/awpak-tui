use std::{io::Write as _, sync::mpsc::{self}};

use awpak_ai::{domain::{error::Error, graph::graph::Graph, tracing::filter_layer::{AwpakAIFilterLayer, AwpakAITarget, AwpakTracingMessage}}, infrastructure::graph::{build_graph::graph_from_path, run_graph::run_graph}};
use text_io::read;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() -> Result<(), ()>
{
    if std::env::args().len() < 2
    {
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
    let ( tx, rx ) = mpsc::channel::<AwpakTracingMessage>();
    let ( tx_stream, rx_stream ) = mpsc::channel::<AwpakTracingMessage>();

    let layer = AwpakAIFilterLayer 
    {
        allowed : vec![ 
            ( AwpakAITarget::AgentStream, tx_stream.clone() ),
            ( AwpakAITarget::AgentToolCall, tx.clone() ),
            ( AwpakAITarget::AgentToolResult, tx.clone() ),
            ( AwpakAITarget::CommandAndArgs, tx.clone() ),
            ( AwpakAITarget::CommandResult, tx.clone() ),
            ( AwpakAITarget::NodeExecution, tx.clone() ),
            ( AwpakAITarget::NodeDestination, tx.clone() )
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
                        println!( "\n{}", s.text );
                        let _ = std::io::stdout().flush();
                    },
                    _ => break    
                }
            }
            
        }
    );

    std::thread::spawn( move || 
        {
            loop
            {
                match rx_stream.recv()
                {
                    Ok( s ) => 
                    {
                        print!( "{}", s.text );
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