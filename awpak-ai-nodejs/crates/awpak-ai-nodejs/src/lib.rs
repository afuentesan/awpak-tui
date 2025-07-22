
use awpak_ai::{domain::graph::graph::Graph, infrastructure::graph::{build_graph::graph_from_json_file_path, run_graph::run_graph as rg} };
use neon::prelude::*;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use crate::graphs::{graph, save_graph};

mod graphs;

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> 
{
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

fn run_graph( mut cx : FunctionContext ) -> JsResult<JsPromise>
{
    let rt = runtime( &mut cx )?;
    let channel = cx.channel();

    let ( deferred, promise ) = cx.promise();

    let id = cx.argument::<JsString>( 0 )?;
    let path = cx.argument::<JsString>( 1 )?;
    let input = cx.argument::<JsString>( 2 ).unwrap_or( JsString::new( &mut cx, "" ) );

    let id = id.value( &mut cx );
    let path = path.value( &mut cx );
    let input = input.value( &mut cx );

    let graph = match graph( id.as_str(), path.as_str() )
    {
        Ok( g ) => g,
        Err( e ) => return JsResult::Err( cx.throw_error( e.to_string() )? )
    };

    rt.spawn(
        async move
        {
            let result = rg( input, graph ).await;

            deferred.settle_with( &channel, move | mut cx | 
                {
                    match result.collect()
                    {
                        ( g, None ) => 
                        {
                            let result = graph_result( &g );

                            save_graph( id.as_str(), g );

                            Ok( cx.string( result ) )
                        },
                        ( _, Some( e ) ) => 
                        {
                            cx.throw_error(e.to_string() )
                        },
                    }
                }
            );
        } 
    );

    Ok( promise )
}

fn run_graph_once( mut cx : FunctionContext ) -> JsResult<JsPromise>
{
    let rt = runtime( &mut cx )?;
    let channel = cx.channel();

    let ( deferred, promise ) = cx.promise();

    let path = cx.argument::<JsString>( 0 )?;
    let input = cx.argument::<JsString>( 1 ).unwrap_or( JsString::new( &mut cx, "" ) );

    let path = path.value( &mut cx );
    let input = input.value( &mut cx );

    let graph = match graph_from_json_file_path( path.as_str() )
    {
        Ok( g ) => g,
        Err( e ) => return JsResult::Err( cx.throw_error( e.to_string() )? )
    };

    rt.spawn(
        async move
        {
            let result = rg( input, graph ).await;

            deferred.settle_with( &channel, move | mut cx | 
                {
                    match result.collect()
                    {
                        ( g, None ) => Ok( cx.string( graph_result( &g ) ) ),
                        ( _, Some( e ) ) => cx.throw_error(e.to_string() ),
                    }
                }
            );
        } 
    );

    Ok( promise )
}

fn graph_result( graph : &Graph ) -> String
{
    match &graph.final_output
    {
        Some( o ) => match o
        {
            Ok( s ) => format!( "ExitOk: \n{}" ,s ),
            Err( s ) => format!( "ExitErr: \n{}" ,s ),
        },
        _ => "".into()
    }
}

#[neon::main]
fn main( mut cx: ModuleContext ) -> NeonResult<()>
{
    cx.export_function( "run_graph_once", run_graph_once )?;
    cx.export_function( "run_graph", run_graph )?;

    Ok( () )
}
