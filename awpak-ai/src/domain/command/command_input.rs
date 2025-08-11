use crate::domain::{data::{data::DataFrom, data_selection::data_selection, data_utils::value_to_string}, error::Error, graph::graph::Graph};


pub async fn command_args( 
    graph : &Graph,
    args : &Vec<DataFrom> 
) -> Result<Vec<String>, Error>
{
    let mut ret = vec![];

    for arg in args
    {
        ret.push( command_arg( graph, arg ).await? );
    }

    Ok( ret )
}

async fn command_arg( 
    graph : &Graph,
    arg : &DataFrom 
) -> Result<String, Error>
{
    Ok( value_to_string( &data_selection( graph, arg ).await? ) )
}