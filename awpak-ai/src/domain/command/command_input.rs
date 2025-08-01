use crate::domain::{data::{data::DataFrom, data_selection::data_selection, data_utils::value_to_string}, error::Error, graph::graph::Graph};


pub fn command_args( 
    graph : &Graph,
    args : &Vec<DataFrom> 
) -> Result<Vec<String>, Error>
{
    let mut ret = vec![];

    for arg in args
    {
        ret.push( command_arg( graph, arg )? );
    }

    Ok( ret )
}

fn command_arg( 
    graph : &Graph,
    arg : &DataFrom 
) -> Result<String, Error>
{
    Ok( value_to_string( &data_selection( graph, arg )? ) )
}

// pub fn command_args_with_wildcards( 
//     graph : &Graph,
//     args : &Vec<DataFrom> 
// ) -> Result<Vec<String>, Error>
// {
//     let mut ret = vec![];

//     for arg in args
//     {
//         ret.append( &mut command_arg_with_wildcards( graph, arg )? );
//     }

//     Ok( ret )
// }

// fn command_arg_with_wildcards( 
//     graph : &Graph,
//     arg : &DataFrom 
// ) -> Result<Vec<String>, Error>
// {
//     let str = value_to_string( &data_selection( graph, arg )? );

//     match glob( &str )
//     {
//         Ok( paths ) =>
//         {
//             let ret = paths.filter_map( Result::ok ).map( | p | p.display().to_string() ).collect::<Vec<_>>();

//             if ret.len() == 0
//             {
//                 Ok( vec![ str ] )
//             }
//             else
//             {
//                 Ok( ret )
//             }
//         },
//         _ => Ok( vec![ str ] )
//     }
// }