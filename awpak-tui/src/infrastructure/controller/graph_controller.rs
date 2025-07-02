use std::sync::mpsc::{self, Sender};

use awpak_ai::{domain::{graph::graph::Graph, tracing::filter_layer::{AwpakAIFilterLayer, AwpakAITarget}}, infrastructure::graph::run_graph::run_graph};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{domain::graph::graph::AwpakTUIGraph, infrastructure::{action::app::action::Action, config::functions::graph_config::{current_graph, save_graph_in_current}}};


pub async fn send_prompt_to_graph( 
    graph : AwpakTUIGraph,
    channel : Sender<Action>
)
{
    if graph.prompt().is_none() { return; }

    proccess_send_prompt_to_graph( graph, channel.clone() ).await;

    let _ = channel.send( Action::EndChatResponse );
}

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
                        Ok( g ) => g,
                        Err( e ) =>
                        {
                            let _ = channel.send( Action::AppendTextToContent( e.to_string() ) );

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
                        ( g, Some( e ) ) =>
                        {
                            save_graph_in_current( "", g );

                            let _ = channel.send( Action::AppendTextToContent( e.to_string() ) );
                        }  
                    }
                }
            );
        }
    ).join();
}

pub fn capture_graph_output( channel : Sender<Action> )
{
    // TODO: Que se pueda personalizar donde enviar cada evento
    let ( tx, rx ) = mpsc::channel::<String>();

    let layer = AwpakAIFilterLayer
    {
        allowed : vec![ 
            ( AwpakAITarget::AgentStream, tx.clone() ),
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
                        let _ = channel.send( Action::AppendTextToContent( s ) );
                    },
                    _ => break    
                }
            }
            
        }
    );
}