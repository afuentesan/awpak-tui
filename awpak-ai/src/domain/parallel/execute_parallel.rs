use serde_json::Value;
use tokio::task::JoinSet;

use crate::domain::{command::execute_command::execute_command, data::{data::DataType, data_compare::compare_data, data_utils::str_to_value}, error::Error, graph::graph::Graph, parallel::parallel::{Parallel, ParallelExecutor}, web_client::execute_web_client::execute_web_client};

pub async fn execute_parallel(
    graph : &Graph,
    parallel : &Parallel
) -> Result<Vec<Value>, Error>
{
    let mut tasks = JoinSet::new();

    for ( idx, executor ) in parallel.executors.iter().enumerate()
    {
        tasks.spawn(
            execute_item( graph.clone(), executor.clone(), idx )
        );
    }

    let mut results = tasks.join_all().await;

    results.sort_by( | ( _, t1 ), ( _, t2 ) | t1.cmp( t2 ) );

    results.into_iter().try_fold(
        vec![], 
        | mut a, r |
        {
            a.push( r.0? );

            Ok( a )
        }
    )
}

async fn execute_item(
    graph : Graph,
    item : ParallelExecutor,
    idx : usize
) -> ( Result<Value, Error>, usize )
{
    match compare_data( 
            &graph,
            item.condition() 
    )
    {
        Ok( r ) if r => {},
        Ok( _ ) => return ( Ok( Value::Null ), idx ),
        Err( e ) => return ( Err( e ), idx )
    };

    let result = match item
    {
        ParallelExecutor::Command { ty, executor, condition : _ } =>
        {
            result_str_to_value(
                execute_command( &graph, &executor ).await,
                ty
            )
        },
        ParallelExecutor::WebClient { ty, executor, condition : _ } =>
        {
            result_str_to_value(
                execute_web_client( &graph, &executor ).await,
                ty
            )
        }
    };

    ( result, idx )
}

fn result_str_to_value( result : Result<String, Error>, ty : DataType ) -> Result<Value, Error>
{
    match result
    {
        Ok( r ) => str_to_value( &r, &ty, true ),
        Err( e ) => Err( e )    
    }
}