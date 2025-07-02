use std::sync::mpsc::{self, Sender};

use awpak_ai::{domain::tracing::filter_layer::{AwpakAIFilterLayer, AwpakAITarget}, infrastructure::graph::run_graph::run_graph};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{domain::graph::graph::AwpakTUIGraph, infrastructure::action::app::action::Action};


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
                    let ( graph, obj_graph ) = graph.own_graph();
                    let ( graph, prompt ) = graph.own_prompt();

                    capture_output( graph, channel.clone() );

                    let result = run_graph( 
                        prompt.unwrap_or( "".into() ), 
                        obj_graph
                    ).await;

                    match result.collect()
                    {
                        ( _, None ) => {},
                        ( _, Some( e ) ) =>
                        {
                            let _ = channel.send( Action::AppendTextToContent( e.to_string() ) );
                        }  
                    }
                }
            );
        }
    ).join();
}

fn capture_output( _graph : AwpakTUIGraph, channel : Sender<Action> )
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