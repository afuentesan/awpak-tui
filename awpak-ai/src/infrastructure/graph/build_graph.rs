use crate::domain::{error::Error, graph::{build_graph::{build_graph_from_path, build_graph_from_str}, graph::Graph}};


pub async fn graph_from_json_file_path( path : impl AsRef<str> ) -> Result<Graph, Error>
{
    build_graph_from_path( path.as_ref() ).await
}

pub async fn graph_from_json_str( json : impl AsRef<str> ) -> Result<Graph, Error>
{
    build_graph_from_str( json ).await
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[tokio::test]
    async fn test_graph_from_path_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/echo_graph.json" ).await;

        assert!( graph.is_ok() )
    }
}