use crate::domain::{error::Error, graph::{build_graph::build_graph_from_path, graph::Graph}};


pub fn graph_from_path( path : &str ) -> Result<Graph, Error>
{
    build_graph_from_path( path )
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_graph_from_path_ok()
    {
        let graph = graph_from_path( "test_data/graphs/echo_graph.json" );

        assert!( graph.is_ok() )
    }
}