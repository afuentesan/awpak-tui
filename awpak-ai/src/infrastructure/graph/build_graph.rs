use crate::domain::{error::Error, graph::{build_graph::{build_graph_from_path, build_graph_from_str}, graph::Graph}};


pub fn graph_from_json_file_path( path : impl AsRef<str> ) -> Result<Graph, Error>
{
    build_graph_from_path( path.as_ref() )
}

pub fn graph_from_json_str( json : impl AsRef<str> ) -> Result<Graph, Error>
{
    build_graph_from_str( json )
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_graph_from_path_ok()
    {
        let graph = graph_from_json_file_path( "test_data/graphs/echo_graph.json" );

        assert!( graph.is_ok() )
    }
}