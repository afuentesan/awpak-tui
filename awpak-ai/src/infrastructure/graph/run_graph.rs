use awpak_utils::result::result::AwpakResult;

use crate::{application, domain::{error::Error, graph::graph::Graph}};


pub async fn run_graph( 
    input : String,
    graph : Graph 
) -> AwpakResult<Graph, Error>
{
    application::graph::run_graph::run_graph( input, graph ).await
}

#[cfg(test)]
mod tests
{
    use crate::infrastructure::graph::build_graph::graph_from_json_file_path;

    use super::*;

    #[tokio::test]
    async fn test_run_loop_from_parsed_input_context_mut_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/loop_from_parsed_input_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "[1.2,2,3,0.3]".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Result: 6.5" );
    }

    #[tokio::test]
    async fn test_run_loop_context_mut_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/loop_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Result: 6" );
    }

    #[tokio::test]
    async fn test_run_context_mut_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/context_mut_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Exit ok: 2" );
    }

    #[tokio::test]
    async fn test_run_graph_node_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/graph_node_executor_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "Echo input".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Exit ok\nEcho input" );

        // assert_eq!( graph.err().unwrap().to_string(), "Ok" );
    }

    #[tokio::test]
    async fn test_run_add_two_numbers_graph_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/add_two_numbers_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( r#"{"a":3, "b":9}"#.into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Sum result: 12" );
    }

    #[tokio::test]
    async fn test_run_add_two_numbers_graph_a_is_string()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/add_two_numbers_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( r#"{"a":"asdf", "b":9}"#.into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_err() );

        let final_output = final_output.err().unwrap();

        assert_eq!( final_output, "Result is not a number: " );
    }

    #[tokio::test]
    async fn test_run_add_3_graph_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/add_3_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "2".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Sum result: 5" );
    }

    #[tokio::test]
    async fn test_run_add_3_graph_input_is_a_string()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/add_3_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "asdf".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_err() );

        let final_output = final_output.err().unwrap();

        assert_eq!( final_output, "asdf is not a number" );
    }

    #[tokio::test]
    async fn test_run_echo_graph_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/echo_graph.json" );

        assert!( graph.is_ok() );

        let graph = graph.unwrap();

        let graph = run_graph( "hola adios".into(), graph ).await;

        assert!( graph.is_ok() );

        let graph = graph.own();

        assert!( graph.final_output.is_some() );

        let final_output = graph.final_output.unwrap();

        assert!( final_output.is_ok() );

        let final_output = final_output.unwrap();

        assert_eq!( final_output, "Exit ok\nhola adios" );

        // assert_eq!( graph.err().unwrap().to_string(), "Ok" );
    }

    #[tokio::test]
    async fn test_run_echo_graph_empty_input()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/echo_graph.json" );

        assert!( graph.is_ok() );

        let mut graph = graph.unwrap();

        graph.final_output = Some( Ok( "Last output".into() ) );

        let graph = run_graph( "".into(), graph ).await;

        assert!( graph.is_err() );

        let graph = graph.own();

        assert!( graph.final_output.is_none() );
    }
}