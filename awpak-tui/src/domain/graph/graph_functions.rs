
use crate::domain::{app::model::app::AppContent, graph::graph::{AwpakTUIGraph, GraphRequest}};


pub fn is_graph_content( content : &AppContent ) -> bool
{
    match content
    {
        AppContent::Graph( _ ) => true,
        _ => false    
    }
}

pub fn is_graph_request_empty( request : &GraphRequest ) -> bool
{
    match request
    {
        GraphRequest::Empty => true,
        _ => false    
    }
}

pub fn is_graph_request_pending( request : &GraphRequest ) -> bool
{
    match request
    {
        GraphRequest::Pending( _ ) => true,
        _ => false    
    }
}

pub fn append_string_to_graph_response( str : String, mut graph : AwpakTUIGraph ) -> AwpakTUIGraph
{
    if graph.response.len() == 0 
    { 
        graph.response = str.split( "\n" ).map( | s | s.to_string() ).collect();

        return graph;
    }

    let mut iter = str.split( "\n" );

    match iter.next()
    {
        Some( s ) =>
        {
            graph.response.last_mut().unwrap().push_str( s );
        },
        _ => return graph
    };

    iter.for_each( | s | graph.response.push( s.to_string() ) );

    graph
}