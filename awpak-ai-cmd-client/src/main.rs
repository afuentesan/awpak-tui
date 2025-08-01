use std::{io::Write as _, sync::mpsc::{self, Sender}, time::Duration};

use awpak_ai::{domain::{error::Error, graph::graph::Graph, tracing::filter_layer::{AwpakAIFilterLayer, AwpakAITarget, AwpakTracingMessage}}, infrastructure::graph::{build_graph::graph_from_json_file_path, run_graph::run_graph}};
use clap::{Arg, ArgMatches, Command};
use text_io::read;
use tokio::time::sleep;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() -> Result<(), ()>
{
    let matches = init_arg_matches();
    
    let path = matches.get_one::<String>( "path" ).ok_or( Error::Ignore ).map_err( | _ | () )?;
    let input = matches.get_one::<String>( "input" );
    let trace = matches.get_one::<String>( "trace" );
    let chat = matches.get_flag( "chat" );

    subscribe_tracing( trace );

    // let path = std::env::args().nth( 1 ).ok_or( Error::Ignore ).map_err( | _ | () )?;

    let mut graph = match graph_from_json_file_path( path )
    {
        Ok( g ) => g,
        Err( e ) =>
        {
            eprintln!( "Build graph error: {:?}", e );

            return Err( () );
        }
    };

    if chat
    {
        loop
        {
            let input = user_input();

            if input.trim() == "" { continue; }

            if input.trim() == "/exit" { break; }

            graph = execute_graph( graph, input ).await;
        }
    }
    else
    {
        execute_graph( graph, input.unwrap_or( &String::new() ).to_string() ).await;
    }
    
    Ok( () )
}

fn init_arg_matches() -> ArgMatches
{
    Command::new("Awpak AI SHELL")
        .version("0.1.0")
        .about("CLI client for executing AI workflows defined as graphs using Awpak AI")
        .arg(
            Arg::new("path")
                .long("path")
                .value_name("FILE")
                .required(true)
                .help("Path to the JSON file containing the graph definition (required)"),
        )
        .arg(
            Arg::new("input")
                .long("input")
                .value_name("INPUT")
                .required(false)
                .help("Graph input value. Ignored when using --chat."),
        )
        .arg(
            Arg::new("trace")
                .long("trace")
                .value_name("TRACE_OPTIONS")
                .required(false)
                .help(
                    "Comma-separated trace options to control output visibility: \
                     If not provided, defaults to \"graph_output_ok,graph_output_err\".\n\
                     \n\
                     graph_input              -> Prints the graph input\n\
                     graph_output_ok          -> Prints the graph output if execution succeeds\n\
                     graph_output_err         -> Prints the graph output if execution fails\n\
                     \n\
                     agent_prompt             -> Prints the prompt for Agent nodes\n\
                     agent_stream             -> Shows streaming output from Agent nodes (if enabled)\n\
                     agent_sync               -> Shows synchronous output from Agent nodes\n\
                     agent_tool_call          -> Shows MCP tool calls made by Agent nodes\n\
                     agent_tool_result        -> Shows the result of MCP tool calls\n\
                     \n\
                     command_and_args         -> Shows the command and arguments for Command nodes\n\
                     command_result           -> Shows the result of Command nodes\n\
                     \n\
                     web_client_request       -> Shows the URL and method for WebClient nodes\n\
                     web_client_request_body  -> Shows the body of WebClient requests\n\
                     web_client_request_headers -> Shows headers of WebClient requests\n\
                     web_client_request_query_params -> Shows query parameters of WebClient requests\n\
                     web_client_response      -> Shows version and status code of WebClient responses\n\
                     web_client_response_headers -> Shows headers of WebClient responses\n\
                     web_client_response_body -> Shows body of WebClient responses\n\
                     \n\
                     node_destination         -> Shows each node executed and the chosen destination\n\
                     node_execution           -> Shows node IDs before execution\n\
                     node_output              -> Shows output of each node",
                ),
        )
        .arg(
            Arg::new("chat")
                .long("chat")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Start interactive chat mode. Input is read from a prompt instead of --input."),
        )
        .get_matches()
}

fn default_trace_options( tx : Sender<AwpakTracingMessage> ) -> Vec<( AwpakAITarget, Sender<AwpakTracingMessage> )>
{
    vec![
        ( AwpakAITarget::GraphOutputOk, tx.clone() ),
        ( AwpakAITarget::GraphOutputErr, tx.clone() )
    ]
}

fn trace_options_from_str( 
    trace : &str, 
    tx : Sender<AwpakTracingMessage>,
    tx_stream : Sender<AwpakTracingMessage>
) -> Vec<( AwpakAITarget, Sender<AwpakTracingMessage> )>
{
    let options = trace.split( "," )
    .flat_map(
        | o | AwpakAITarget::from_str( o )
    )
    .collect::<Vec<_>>();

    if options.len() == 0 { return default_trace_options( tx ) };

    options.into_iter().map( 
        | o | 
        {
            let tx = if o == AwpakAITarget::AgentStream { tx_stream.clone() } else { tx.clone() };

            ( o, tx )
        }
        
    ).collect()
}

fn trace_options( 
    trace : Option<&String>, 
    tx : Sender<AwpakTracingMessage>,
    tx_stream : Sender<AwpakTracingMessage>
) -> Vec<( AwpakAITarget, Sender<AwpakTracingMessage> )>
{
    match trace
    {
        Some( t ) => trace_options_from_str( t, tx, tx_stream ),
        None => default_trace_options( tx )   
    }
}

fn subscribe_tracing( trace : Option<&String> )
{
    // tracing_subscriber::fmt::init();

    let ( tx, rx ) = mpsc::channel::<AwpakTracingMessage>();
    let ( tx_stream, rx_stream ) = mpsc::channel::<AwpakTracingMessage>();

    let options = trace_options( trace, tx, tx_stream );

    // options.iter().for_each( | o | println!( "Option: {:?}", o.0 ) );

    let layer = AwpakAIFilterLayer 
    {
        allowed : options,
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
    let _ = std::io::stdout().flush();
    read!( "{}\n" )
}

async fn execute_graph( graph : Graph, input : String ) -> Graph
{
    let result = run_graph( input, graph ).await;

    let _ = sleep( Duration::from_millis( 100 ) ).await;

    match result.collect()
    {
        ( g, None ) => 
        {
            println!( "\n\n" );
            let _ = std::io::stdout().flush();

            g
        },
        ( g, Some( e ) ) => 
        {
            println!( "\n\nError: {}\n\n", e.to_string() );
            let _ = std::io::stdout().flush();
            
            g
        }
    }
}