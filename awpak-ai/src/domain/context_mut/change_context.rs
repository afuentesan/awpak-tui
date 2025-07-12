
use awpak_utils::result::result::AwpakResult;

use crate::domain::{context_mut::context_mut::ContextMut, data::{data_insert::value_to_context, data_selection::data_selection}, error::Error, graph::graph::Graph};


pub async fn change_context( 
    context_mut : &Vec<ContextMut>, 
    mut graph : Graph 
) -> AwpakResult<Graph, Error>
{
    for c in context_mut
    {
        graph = match change_item_context( c, graph ).await.collect()
        {
            ( g, None ) => g,
            ( g, Some( e ) ) => return AwpakResult::new_err( g, e )    
        };
    }

    AwpakResult::new( graph )
}

pub async fn change_item_context( context_mut : &ContextMut, mut graph : Graph ) -> AwpakResult<Graph, Error>
{
    let data = match data_selection( graph.input.as_ref(), &graph.parsed_input, &graph.context, &context_mut.from )
    {
        Ok( d ) => d,
        Err( e ) => return AwpakResult::new_err( graph, e )
    };

    match value_to_context( graph.context, data, &context_mut.to ).collect()
    {
        ( ( c, _ ), None ) =>
        {
            graph.context = c;

            AwpakResult::new( graph )
        },
        ( ( c, _ ), Some( e ) ) =>
        {
            graph.context = c;

            AwpakResult::new_err( graph, e )
        }
    }
}