use awpak_utils::result::result::AwpakResult;
use serde_json::json;

use crate::{application::graph::run_graph::run_graph, domain::{data::data_selection::data_to_string, error::Error, graph::{graph::Graph, graph_node::{GraphNode, GraphNodeOutput}}, utils::string_utils::{prefix_str_suffix, str_from_option}}};


pub async fn execute_graph(
    parent_graph : &Graph,
    mut graph_node : GraphNode
) -> AwpakResult<( GraphNode, String ), Error>
{
    let graph = graph_node.graph;

    let input = data_to_string( parent_graph, graph_node.input.clone() );

    match run_graph( input, graph ).await.collect()
    {
        ( g, None ) =>
        {
            let output = graph_output( &g, &graph_node.output );

            graph_node.graph = g;

            AwpakResult::new( ( graph_node, output ) )
        },
        ( g, Some( e ) ) =>
        {
            // Cuando hay error no estamos recogiendo la salida así que no la calculamos. Esto podría cambiar en un futuro.
            //let output = graph_output( &g, &graph_node.output );

            graph_node.graph = g;

            AwpakResult::new_err( ( graph_node, "".into() ), e )
        }
    }
}

fn graph_output(
    graph : &Graph,
    output : &Vec<GraphNodeOutput>
) -> String
{
    if graph.final_output.is_none() { return "".into() }

    let final_output = graph.final_output.clone().unwrap();

    output.iter().fold(
        "".to_string(), 
        | mut a,  o |
        {
            a.push_str( 
                match o
                {
                    GraphNodeOutput::Success { prefix, suffix } =>
                    {
                        prefix_str_suffix( 
                            prefix.as_ref(), 
                            suffix.as_ref(), 
                            if final_output.is_ok() { "true" } else { "false" } 
                        )
                    },
                    GraphNodeOutput::Out { prefix, suffix } =>
                    {
                        prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), str_from_option( final_output.as_ref().ok() ).as_str() )
                    },
                    GraphNodeOutput::Err { prefix, suffix } =>
                    {
                        prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), str_from_option( final_output.as_ref().err() ).as_str() )
                    },
                    GraphNodeOutput::Object { prefix, suffix } =>
                    {
                        let json = json!(
                            {
                                "success" : if final_output.is_ok() { true } else { false },
                                "out" : str_from_option( final_output.as_ref().ok() ).as_str(),
                                "err" : str_from_option( final_output.as_ref().err() ).as_str()
                            }
                        ).to_string();

                        prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), &json ) 
                    }
                }.as_str()
            );

            a
        }
    )
}