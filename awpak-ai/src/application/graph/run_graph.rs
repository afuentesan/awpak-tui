use awpak_utils::result::result::AwpakResult;
use serde_json::Value;
use async_recursion::async_recursion;
use tracing::info;

use crate::{application::graph::execute_graph::execute_graph, domain::{agent::execute_agent::execute_agent, command::execute_command::execute_command, context_mut::change_context::change_context, data::{data::{DataComparator, DataType}, data_compare::compare_data, data_insert::str_to_context, data_selection::data_to_string, data_utils::str_to_value}, error::Error, graph::{graph::Graph, node::{NodeDestination, NodeExecutor, NodeNext}}, tracing::filter_layer::{NODE_DESTINATION, NODE_EXECUTION}, utils::string_utils::option_string_to_str}};


struct GraphRunner
{
    graph : Graph,
    next : String
}

#[async_recursion]
pub async fn run_graph( 
    input : String,
    graph : Graph 
) -> AwpakResult<Graph, Error>
{
    let graph = match init_graph( input, graph ).collect()
    {
        ( g, None ) => g,
        ( g, Some( e ) ) => return AwpakResult::new_err( g, e )    
    };

    let runner = GraphRunner { next : graph.first.clone() , graph };

    AwpakResult::new( runner )
    .write()
    .map_while(
        async | r | next_step( r ).await
    )
    .await
    .finalize()
    .unzip( 
        | r | r.graph
    )
    .read()
}

fn init_graph(
    input : String,
    mut graph : Graph
) -> AwpakResult<Graph, Error>
{
    let opt_input = graph_input( input.as_str() );
    let parsed_input = match graph_parsed_input( graph.input_type.as_ref(), input.as_str() )
    {
        Ok( p ) => p,
        Err( e ) => return AwpakResult::new_err( graph, e )
    };

    graph.input = opt_input;
    graph.parsed_input = parsed_input;
    graph.final_output = None;

    graph = graph.init_context();

    AwpakResult::new( graph )
}

async fn next_step( runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    let next = runner.next.clone();

    match runner.graph.nodes.get( next.as_str() )
    {
        Some( _ ) => execute_node( runner ).await,
        None => ( AwpakResult::new_err( runner, Error::NodeNotFound( next ) ), false )
    }
}

async fn execute_node( mut runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    let mut node = runner.graph.nodes.remove( runner.next.as_str() ).unwrap();

    info!( target:NODE_EXECUTION, id=option_string_to_str( runner.graph.id.as_ref() ), text=format!( "{}", node.id ) );

    let ( node, result ) = match &node.executor
    {
        NodeExecutor::Command( c ) =>
        {
            let result = execute_command( 
                runner.graph.id.as_ref(), 
                runner.graph.input.as_ref(), 
                &runner.graph.parsed_input, 
                &runner.graph.context, 
                c 
            ).await;

            (
                node,
                result
            )
        },
        NodeExecutor::Agent( a ) =>
        {
            let result = execute_agent( 
                runner.graph.id.as_ref(),
                runner.graph.input.as_ref(), 
                &runner.graph.parsed_input, 
                &runner.graph.context, 
                a 
            ).await;

            match result
            {
                Ok( ( s, h ) ) =>
                {
                    if a.save_history
                    {
                        let ( _, agent ) = node.executor.own_agent();

                        let mut agent = agent.unwrap();

                        agent.history = h;

                        node.executor = NodeExecutor::Agent( agent );

                        ( node, Ok( s ) )
                    }
                    else
                    {
                        ( node, Ok( s ) )
                    }
                },
                Err( e ) => ( node, Err( e ) )
            }
        },
        NodeExecutor::Graph( _ ) =>
        {
            let ( _, g ) = node.executor.own_graph();

            let g = g.unwrap();

            match execute_graph( runner.graph.input.as_ref(), &runner.graph.parsed_input, &runner.graph.context, g ).await.collect()
            {
                ( ( g, o ), None ) =>
                {
                    node.executor = NodeExecutor::Graph( g );

                    (
                        node,
                        Ok( o )
                    )
                },
                ( ( g, _ ), Some( e ) ) =>
                {
                    node.executor = NodeExecutor::Graph( g );

                    (
                        node,
                        Err( e )
                    )
                }
            }
        },
        NodeExecutor::ContextMut( cm ) =>
        {
            let graph = runner.graph;

            match change_context( 
                cm, 
                graph 
            )
            .await
            .collect()
            {
                ( g, None ) =>
                {
                    runner.graph = g;

                    (
                        node,
                        Ok( "".into() )
                    )
                },
                ( g, Some( e ) ) =>
                {
                    runner.graph = g;

                    (
                        node,
                        Err( e )
                    )
                }
            }
        }
    };

    runner.graph.nodes.insert( runner.next.clone(), node );

    match result
    {
        Ok( s ) => proccess_result( s, runner ).await,
        Err( e ) => ( AwpakResult::new_err( runner, e ), false )
    }
}

async fn proccess_result( str_result : String, runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    match output_to_context( str_result, runner ).await.collect()
    {
        ( g, None ) => redirect_or_exit( g ).await,
        ( g, Some( e ) ) => ( AwpakResult::new_err( g, e ), false )
    }
}

async fn redirect_or_exit( runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    let node = runner.graph.nodes.get( runner.next.as_str() ).unwrap();

    for d in &node.destination
    {
        match check_node_destination_condition( &runner.graph, &d.condition )
        {
            Ok( r ) if r =>
            {
                let d = d.clone();

                return update_next( node.id.clone(), runner, d ).await
            }
            Ok( _ ) => continue,
            Err( e ) => return ( AwpakResult::new_err( runner, e ), false )
        }
    }

    ( AwpakResult::new_err( runner, Error::NodeNotFound( "Destination not found".into() ) ), false )
}

fn trace_node_destination( from : String, to : &str, graph_id : Option<&String> )
{
    info!( target:NODE_DESTINATION, id=option_string_to_str( graph_id ), text=format!( "From: {}, to: {}", from, to ) );
}

fn check_node_destination_condition( graph : &Graph, comparator : &DataComparator ) -> Result<bool, Error>
{
    compare_data(
        graph.input.as_ref(), 
        &graph.parsed_input, 
        &graph.context, 
        comparator
    )
}

async fn update_next( from : String, mut runner : GraphRunner, destination : NodeDestination ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    match destination.next
    {
        NodeNext::ExitOk( o ) =>
        {
            trace_node_destination( from, "ExitOK", runner.graph.id.as_ref() );

            let o = data_to_string( 
                runner.graph.input.as_ref(), 
                &runner.graph.parsed_input, 
                &runner.graph.context, 
                o
            );

            runner.graph.final_output = Some( Ok( o ) );

            ( AwpakResult::new( runner ), false )
        },
        NodeNext::ExitErr( o ) =>
        {
            trace_node_destination( from, "ExitErr", runner.graph.id.as_ref() );

            let o = data_to_string( 
                runner.graph.input.as_ref(), 
                &runner.graph.parsed_input, 
                &runner.graph.context, 
                o
            );

            runner.graph.final_output = Some( Err( o ) );

            ( AwpakResult::new( runner ), false )
        },
        NodeNext::Node( n ) =>
        {
            match runner.graph.nodes.get( n.as_str() )
            {
                Some( _ ) =>
                {
                    trace_node_destination( from, n.as_str(), runner.graph.id.as_ref() );

                    runner.next = n;

                    ( AwpakResult::new( runner ), true )
                },
                _ => ( AwpakResult::new_err( runner, Error::NodeNotFound( format!( "Node {} not found.", n ) ) ), false )
            }
        }
    }
}

async fn output_to_context( output : String, mut runner : GraphRunner ) -> AwpakResult<GraphRunner, Error>
{
    let node = runner.graph.nodes.get( runner.next.as_str() ).unwrap();

    match &node.output
    {
        Some( c ) =>
        {
            let context = str_to_context( output, runner.graph.context, c );

            match context.collect()
            {
                ( c, None ) =>
                {
                    runner.graph.context = c;

                    AwpakResult::new( runner )
                },
                ( c, Some( e ) ) =>
                {
                    runner.graph.context = c;

                    AwpakResult::new_err( runner, e )
                }
            }
        },
        None => AwpakResult::new( runner )
    }
}

fn graph_parsed_input(
    input_type : Option<&DataType>,
    input : &str
) -> Result<Value, Error>
{
    match input_type
    {
        Some( t ) => str_to_value( input, t ),
        None => Ok( Value::Null )
    }
}

fn graph_input(
    input : &str
) -> Option<String>
{
    if input.trim() == "" { None } else { Some( input.trim().to_string() ) }
}