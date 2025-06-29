use awpak_ai::{domain::graph::graph::Graph, infrastructure::graph::{build_graph::graph_from_path, run_graph::run_graph}};
use text_io::read;

#[tokio::main]
async fn main() -> Result<(), ()>
{
    if std::env::args().len() < 2
    {
        eprintln!( "Graph path not found" );

        return Err( () );
    }

    let path = std::env::args().nth( 1 ).ok_or( () )?;

    let mut graph = graph_from_path( &path ).map_err( | _ | () )?;

    loop
    {
        let input = user_input();

        if input.trim() == "" { continue; }

        if input.trim() == "exit" { break; }

        graph = execute_graph( graph, input ).await;
    }
    
    Ok( () )
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
