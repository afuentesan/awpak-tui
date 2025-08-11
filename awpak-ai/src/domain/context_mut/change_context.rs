
use awpak_utils::result::result::AwpakResult;

use crate::domain::{context_mut::context_mut::ContextMut, data::{data_compare::compare_data, data_insert::value_to_context, data_selection::data_selection}, error::Error, graph::graph::Graph};


pub async fn change_context( 
    context_mut : &Vec<ContextMut>, 
    mut graph : Graph 
) -> AwpakResult<Graph, Error>
{
    for c in context_mut
    {
        match compare_data( 
            &graph,
            &c.condition 
        ).await
        {
            Ok( r ) if r =>
            {
                graph = match change_item_context( c, graph ).await.collect()
                {
                    ( g, None ) => g,
                    ( g, Some( e ) ) => return AwpakResult::new_err( g, e )    
                };
            }
            Ok( _ ) => continue,
            Err( e ) => return AwpakResult::new_err( graph, e )
        }
    }

    AwpakResult::new( graph )
}

pub async fn change_item_context( context_mut : &ContextMut, mut graph : Graph ) -> AwpakResult<Graph, Error>
{
    let data = match data_selection( &graph, &context_mut.from ).await
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