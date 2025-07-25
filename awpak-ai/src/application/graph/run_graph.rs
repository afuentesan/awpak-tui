
use awpak_utils::result::result::AwpakResult;
use serde_json::Value;
use async_recursion::async_recursion;
use tracing::info;

use crate::{application::graph::execute_graph::execute_graph, domain::{agent::execute_agent::execute_agent, agent_history_mut::change_agent_history::change_agent_history, command::execute_command::execute_command, context_mut::change_context::change_context, data::{data::{DataComparator, DataType}, data_compare::compare_data, data_insert::{str_to_context, value_to_context}, data_selection::data_to_string, data_utils::str_to_value}, error::{ChangeError, Error}, graph::{graph::Graph, node::{NodeDestination, NodeExecutor, NodeNext}}, parallel::execute_parallel::execute_parallel, tracing::filter_layer::{GRAPH_INPUT, GRAPH_OUTPUT_ERR, GRAPH_OUTPUT_OK, NODE_DESTINATION, NODE_EXECUTION, NODE_OUTPUT}, utils::string_utils::option_string_to_str, web_client::execute_web_client::execute_web_client}};


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
    trace_graph_input( graph.id.as_ref(), &input );

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
        | r | 
        {
            trace_graph_output( r.graph.id.as_ref(), r.graph.final_output.as_ref() );

            r.graph
        }
    )
    .read()
}

fn trace_graph_input( graph_id : Option<&String>, input : &str )
{
    info!(
        target:GRAPH_INPUT, 
        id=option_string_to_str( graph_id ), 
        text=input
    );
}

fn trace_graph_output( graph_id : Option<&String>, final_output : Option<&Result<String, String>> )
{
    let ( ok, text ) = match final_output
    {
        Some( r ) => match r
        {
            Ok( s ) => ( true, s ),
            Err( s ) => ( false, s )
        },
        None => return
    };

    if ok
    {
        info!(
            target:GRAPH_OUTPUT_OK, 
            id=option_string_to_str( graph_id ), 
            text=text
        );
    }
    else
    {
        info!(
            target:GRAPH_OUTPUT_ERR, 
            id=option_string_to_str( graph_id ), 
            text=text
        );
    }
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
        Err( e ) => 
        {
            let msg = format!( "Init graph {:?}.\n", graph.id );

            return AwpakResult::new_err( graph, e.prepend_str( msg ) )
        }
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

    let ( 
        node, 
        result
    ) = match &node.executor
    {
        NodeExecutor::Parallel( p ) =>
        {
            let result = execute_parallel(
                &runner.graph, 
                p
            ).await
            .prepend_err( format!( "NodeExecutor::Parallel {}\n", node.id ) );

            runner.graph.nodes.insert( runner.next.clone(), node );

            return match result
            {
                Ok( r ) => proccess_parallel_result( r, runner ).await,
                Err( e ) => ( AwpakResult::new_err( runner, e ), false )
            }
        },
        NodeExecutor::WebClient( c ) =>
        {
            let result = execute_web_client( 
                &runner.graph, 
                c 
            ).await
            .prepend_err( format!( "NodeExecutor::WebClient {}\n", node.id ) );

            (
                node,
                result
            )
        },
        NodeExecutor::Command( c ) =>
        {
            let result = execute_command( 
                &runner.graph,
                c 
            ).await
            .prepend_err( format!( "NodeExecutor::Command {}\n", node.id ) );

            (
                node,
                result
            )
        },
        NodeExecutor::Agent( a ) =>
        {
            let result = execute_agent( 
                &runner.graph, 
                a 
            ).await
            .prepend_err( format!( "NodeExecutor::Agent {}\n", node.id ) );

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

            match execute_graph( &runner.graph, g ).await.collect()
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

                    let msg = format!( "NodeExecutor::Graph {}\n", node.id );

                    (
                        node,
                        Err( e.prepend_str( msg ) )
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

                    let msg = format!( "NodeExecutor::ContextMut {}\n", node.id );

                    (
                        node,
                        Err( e.prepend_str( msg ) )
                    )
                }
            }
        },
        NodeExecutor::AgentHistoryMut( ahm ) =>
        {
            let graph = runner.graph;

            match change_agent_history( 
                ahm, 
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

                    let msg = format!( "NodeExecutor::AgentHistoryMut {}\n", node.id );

                    (
                        node,
                        Err( e.prepend_str( msg ) )
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

async fn proccess_parallel_result( result : Vec<Value>, mut runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    info!(
        target:NODE_OUTPUT, 
        id=option_string_to_str( runner.graph.id.as_ref() ), 
        text=serde_json::to_string( &result ).unwrap_or( "".into() )
    );

    let node = runner.graph.nodes.get( runner.next.as_str() ).unwrap();

    match &node.output
    {
        Some( o ) =>
        {
            let context = runner.graph.context;

            match value_to_context( context, Value::Array( result ), o ).collect()
            {
                ( ( c, _ ), None ) =>
                {
                    runner.graph.context = c;

                    redirect_or_exit( runner ).await
                },
                ( ( c, _ ), Some( e ) ) =>
                {
                    runner.graph.context = c;

                    ( AwpakResult::new_err( runner, e ), false )
                }
            }
        },
        None => redirect_or_exit( runner ).await
    }
}

async fn proccess_result( str_result : String, runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    info!(
        target:NODE_OUTPUT, 
        id=option_string_to_str( runner.graph.id.as_ref() ), 
        text=str_result
    );

    match output_to_context( str_result, runner ).await.collect()
    {
        ( g, None ) => redirect_or_exit( g ).await,
        ( g, Some( e ) ) => ( AwpakResult::new_err( g, e ), false )
    }
}

async fn redirect_or_exit( runner : GraphRunner ) -> ( AwpakResult<GraphRunner, Error>, bool )
{
    let node = runner.graph.nodes.get( runner.next.as_str() ).unwrap();

    let mut count : usize = 0;

    for d in &node.destination
    {
        match check_node_destination_condition( &runner.graph, &d.condition )
        {
            Ok( r ) if r =>
            {
                let d = d.clone();

                return update_next( node.id.clone(), runner, d ).await
            }
            Ok( _ ) => {},
            Err( e ) => 
            {
                let msg = format!( "NodeDestination condition. NodeId: {}. Destination {}\n", node.id, count );

                return ( 
                    AwpakResult::new_err( 
                        runner, 
                        e.prepend_str( msg )
                    ), 
                    false 
                )
            }
        }

        count += 1;
    }

    let msg = format!( "Destination not found in node: {}", node.id );

    ( AwpakResult::new_err( runner, Error::NodeNotFound( msg ) ), false )
}

fn trace_node_destination( from : String, to : &str, graph_id : Option<&String> )
{
    info!( target:NODE_DESTINATION, id=option_string_to_str( graph_id ), text=format!( "From: {}, to: {}", from, to ) );
}

fn check_node_destination_condition( graph : &Graph, comparator : &DataComparator ) -> Result<bool, Error>
{
    compare_data(
        &graph, 
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
                &runner.graph,  
                o
            );

            runner.graph.final_output = Some( Ok( o ) );

            ( AwpakResult::new( runner ), false )
        },
        NodeNext::ExitErr( o ) =>
        {
            trace_node_destination( from, "ExitErr", runner.graph.id.as_ref() );

            let o = data_to_string( 
                &runner.graph, 
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
                _ => ( 
                    AwpakResult::new_err( 
                        runner, 
                        Error::NodeNotFound( format!( "Node {} not found. Destination from: {}", n, from ) ) 
                    ), 
                    false 
                )
            }
        }
    }
}

async fn output_to_context( output : String, mut runner : GraphRunner ) -> AwpakResult<GraphRunner, Error>
{
    let node = runner.graph.nodes.get( runner.next.as_str() ).unwrap();

    if let NodeExecutor::ContextMut( _ ) | NodeExecutor::AgentHistoryMut( _ ) = &node.executor
    {
        return AwpakResult::new( runner );
    }

    match &node.output
    {
        Some( c ) =>
        {
            let msg_err = format!( "Data to context. NodeId: {}\nOutput: {}\n{:?}\n", node.id, output, c );

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

                    AwpakResult::new_err( runner, e.prepend_str( msg_err ) )
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
        Some( t ) => str_to_value( input, t, false )
                                .map_err( 
                                    | e | e.prepend_str( 
                                        format!( "ParsedInput. Type: {:?}. Input: {}\n", input_type, input ) 
                                    )
                                ),
        None => Ok( Value::Null )
    }
}

fn graph_input(
    input : &str
) -> Option<String>
{
    if input.trim() == "" { None } else { Some( input.trim().to_string() ) }
}