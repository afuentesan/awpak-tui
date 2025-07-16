use std::{sync::mpsc::{self, Sender}, time::Duration};

use awpak_ai::{domain::{graph::graph::Graph, signals::cancel_graph::{cancel_graph, init_cancel_state}, tracing::filter_layer::{AwpakAIFilterLayer, AwpakAITarget, AwpakTracingMessage, AGENT_STREAM, AGENT_SYNC, AGENT_TOOL_CALL, AGENT_TOOL_RESULT, COMMAND_AND_ARGS, COMMAND_RESULT, GRAPH_INPUT, GRAPH_OUTPUT_ERR, GRAPH_OUTPUT_OK, NODE_DESTINATION, NODE_EXECUTION}}, infrastructure::graph::run_graph::run_graph};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{domain::{graph::graph::AwpakTUIGraph, util::file_utils::append_text_to_file}, infrastructure::{action::app::action::Action, channel::channel::{clean_recv_abort_chat, try_recv_abort_chat}, config::{functions::graph_config::{current_graph, graph_output_config, save_graph_in_current}, model::graph_config::{AwpakTUIGraphOutputConfig, AwpakTUIGraphOutputDestinationConfig}}}};


pub async fn send_prompt_to_graph( 
    graph : AwpakTUIGraph,
    channel : Sender<Action>
)
{
    if graph.prompt().is_none() { return; }

    let response_id = if graph.id.trim() != "" { Some( graph.id.clone() ) } else { None };

    // let _ = channel.send(
    //     match &response_id
    //     {
    //         Some( i ) => Action::AppendTextToContentId 
    //         { 
    //             text : format!( "\nInit graph {}\n", i ),
    //             id : i.clone()
    //         },
    //         _ => Action::AppendTextToContent( format!( "\nInit graph\n" ) )
    //     }
    // );

    let end = control_graph_execution( graph.id.clone() );

    // alternate_proccess_send_prompt_to_graph( graph, channel.clone() ).await;
    proccess_send_prompt_to_graph( graph, channel.clone() ).await;

    let _ = end.send( () );

    let _ = channel.send( 
        Action::EndChatResponse( response_id ) 
    );
}

fn control_graph_execution( id : String ) -> Sender<()>
{
    clean_recv_abort_chat();

    init_cancel_state( id.clone() );

    let ( tx, rx ) = mpsc::channel::<()>();

    let _ = std::thread::spawn( move ||
        {
            loop
            {
                if rx.try_recv().is_ok()
                {
                    return;
                }

                if try_recv_abort_chat().is_some()
                {
                    cancel_graph( &id );

                    return;
                }

                std::thread::sleep( Duration::from_millis( 1000 ) );
            }
        }
    );

    tx
}

// async fn alternate_proccess_send_prompt_to_graph( graph : AwpakTUIGraph, channel : Sender<Action> )
// {
//     let ( graph, prompt ) = graph.own_prompt();

//     let obj_graph : Graph = match current_graph( &graph.initial_id, &graph.id )
//     {
//         Ok( mut g ) => 
//         {
//             g.id = Some( graph.id.clone() );

//             g
//         },
//         Err( e ) =>
//         {
//             let _ = channel.send( Action::AppendTextToContent( e.to_string() ) );

//             return;
//         }    
//     };

//     let result = run_graph( 
//         prompt.unwrap_or( "".into() ), 
//         obj_graph
//     ).await;

//     match result.collect()
//     {
//         ( g, None ) => save_graph_in_current( &graph.id, g ),
//         ( _, Some( e ) ) =>
//         {
//             let _ = channel.send( Action::AppendTextToContent( e.to_string() ) );
//         }  
//     }
// }

async fn proccess_send_prompt_to_graph( graph : AwpakTUIGraph, channel : Sender<Action> )
{
    let handle = tokio::runtime::Handle::current();

    let _ = std::thread::spawn( move ||
        {
            handle.block_on( async move
                {
                    let ( graph, prompt ) = graph.own_prompt();

                    let obj_graph : Graph = match current_graph( &graph.initial_id, &graph.id )
                    {
                        Ok( mut g ) => 
                        {
                            g.id = Some( graph.id.clone() );

                            g
                        },
                        Err( e ) =>
                        {
                            let _ = channel.send( Action::AppendTextToContent( format!( "\n\n{:?}\n", e ) ) );

                            return;
                        }    
                    };

                    let result = run_graph( 
                        prompt.unwrap_or( "".into() ), 
                        obj_graph
                    ).await;

                    match result.collect()
                    {
                        ( g, None ) => save_graph_in_current( &graph.id, g ),
                        ( _, Some( e ) ) =>
                        {
                            let _ = channel.send( Action::AppendTextToContent( format!( "\n\n{:?}\n", e ) ) );
                        }  
                    }
                }
            );
        }
    ).join();
}

pub fn capture_graph_output( channel : Sender<Action> )
{
    let ( tx, rx ) = mpsc::channel::<AwpakTracingMessage>();

    let layer = AwpakAIFilterLayer
    {
        allowed : vec![ 
            ( AwpakAITarget::AgentStream, tx.clone() ),
            ( AwpakAITarget::AgentSync, tx.clone() ),
            ( AwpakAITarget::AgentToolCall, tx.clone() ),
            ( AwpakAITarget::AgentToolResult, tx.clone() ),
            ( AwpakAITarget::CommandAndArgs, tx.clone() ),
            ( AwpakAITarget::CommandResult, tx.clone() ),
            ( AwpakAITarget::NodeDestination, tx.clone() ),
            ( AwpakAITarget::NodeExecution, tx.clone() ),
            ( AwpakAITarget::GraphInput, tx.clone() ),
            ( AwpakAITarget::GraphOutputOk, tx.clone() ),
            ( AwpakAITarget::GraphOutputErr, tx.clone() )
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
                        match s.id
                        {
                            Some( id ) =>
                            {
                                proccess_message( id, s.target, s.text, channel.clone() );
                            },
                            _ =>
                            {
                                let _ = channel.send( Action::AppendTextToContent( s.text ) );
                            }
                        }
                    },
                    _ => break    
                }
            }
            
        }
    );
}

fn proccess_message( id : String, target : String, text : String, channel : Sender<Action> )
{
    if let Some( c ) = graph_output_config( &id )
    {
        proccess_message_from_config( id, c, target, text, channel );
    }
}

fn proccess_message_from_config( 
    id : String, 
    config : AwpakTUIGraphOutputConfig, 
    target : String, 
    text : String, 
    channel : Sender<Action> 
)
{
    match target.as_str()
    {
        AGENT_STREAM => proccess_message_from_destinations( id, config.agent_stream, text, channel ),
        AGENT_SYNC => proccess_message_from_destinations( 
            id, 
            config.agent_sync, 
            format!( "\n{}\n", text ), 
            channel 
        ),
        AGENT_TOOL_CALL => proccess_message_from_destinations(
            id, 
            config.agent_tool_call, 
            text, 
            channel 
        ),
        AGENT_TOOL_RESULT => proccess_message_from_destinations(
            id, 
            config.agent_tool_result, 
            text, 
            channel 
        ),
        COMMAND_AND_ARGS => proccess_message_from_destinations( 
            id, 
            config.command_and_args, 
            format!( "\nExec command: {}\n", text ), 
            channel 
        ),
        COMMAND_RESULT => proccess_message_from_destinations( id, config.command_result, text, channel ),
        NODE_DESTINATION => proccess_message_from_destinations( 
            id, 
            config.node_destination, 
            format!( "\n\nNodeDestination. {}\n", text ), 
            channel 
        ),
        NODE_EXECUTION => proccess_message_from_destinations( 
            id, 
            config.node_execution, 
            format!( "\nExec node {}\n\n", text ), 
            channel 
        ),
        GRAPH_INPUT => proccess_message_from_destinations(
            id.clone(),
            config.graph_input,
            format!( "\nInit graph {}. Graph input:\n{}", id, text ),
            channel
        ),
        GRAPH_OUTPUT_OK => proccess_message_from_destinations(
            id.clone(),
            config.graph_output_ok,
            format!( "\nEnd graph {}. ExitOk:\n{}", id, text ),
            channel
        ),
        GRAPH_OUTPUT_ERR => proccess_message_from_destinations(
            id.clone(),
            config.graph_output_err,
            format!( "\nEnd graph {}. ExitErr:\n{}", id, text ),
            channel
        ),
        _ => {}
    }
}

fn proccess_message_from_destinations(
    id : String, 
    destination : Vec<AwpakTUIGraphOutputDestinationConfig>,
    text : String,
    channel : Sender<Action>
)
{
    destination.into_iter()
    .for_each( 
        | d |
        {
            proccess_message_from_destination( id.clone(), d, text.clone(), channel.clone() );
        }
    );
}

fn proccess_message_from_destination(
    id : String, 
    destination : AwpakTUIGraphOutputDestinationConfig,
    text : String,
    channel : Sender<Action>
)
{
    match destination
    {
        AwpakTUIGraphOutputDestinationConfig::Console => 
        {
            let _ = channel.send(
                Action::AppendTextToContentId 
                { 
                    id, 
                    text
                }
            );
        },
        AwpakTUIGraphOutputDestinationConfig::File( f ) =>
        {
            append_text_to_file( &text, &f );
        }
    }
}