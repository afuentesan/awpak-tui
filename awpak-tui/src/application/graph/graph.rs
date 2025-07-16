use uuid::Uuid;

use crate::domain::{app::model::app::{App, AppContent, AppFocus, Confirm}, content_generator::model::content_generator::ContentGenerator, error::Error, graph::{graph::{AwpakTUIGraph, GraphRequest}, graph_functions::{append_string_to_graph_response, is_graph_content, is_graph_request_empty, is_graph_request_pending}}, input::model::input::Input, result::{functions::result_utils::bool_err, result::AwpakResult}, selectable::{functions::selectable_utils::idx_current_selected_item, model::selectable::Selectable as _}};

pub fn send_graph_request( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    // .map_result( | a | bool_err( a.content_search().text.trim() == "", Error::Ignore ) )
    .map_result( | a | bool_err( ! is_graph_content( a.content() ), Error::Ignore ) )
    .map_result( | a | bool_err( ! is_graph_request_empty( &a.graph_content().unwrap().request ), Error::Ignore ) )
    .write()
    .map(
        | a |
        {
            let ( a, graph ) = a.own_graph_content();

            let graph = graph.unwrap().change_request( GraphRequest::Pending( a.content_search().text.trim().to_string() ) );

            let a = a.change_content_search( Input::default() );

            a.change_content( AppContent::Graph( graph ) )
        }
    )
    .read()
}

pub fn open_new_graph( app : App ) -> AwpakResult<App>
{
    show_new_graph( app )
}

pub fn open_saved_graph( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Graph( _ ) => show_saved_graph( app ),
        _ => show_saved_or_new_graph( app )
    }
}

pub fn confirm_graph_selection( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( idx_current_selected_item( a.graphs() ).is_none(), Error::Ignore ) )
    .write()
    .map( | a | show_selected_graph( a ) )
    .finalize()
    .unzip( | a | a.change_focus( AppFocus::Search ) )
    .read()
}

pub fn confirm_saved_graph_selection( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .zip_result( | a | idx_current_selected_item( a.saved_graphs() ).ok_or( Error::Ignore ) )
    .write()
    .map(
        | ( a, i ) |
        {
            let i = i.unwrap();

            ( show_saved_graph_idx( a, i ), Ok( i ) )
        }
    )
    .finalize()
    .unzip( | ( a, _ ) | a )
    .read()
}

fn show_saved_or_new_graph( app : App ) -> AwpakResult<App>
{
    match show_saved_graph( app ).collect()
    {
        ( a, None ) => AwpakResult::new( a ),
        ( a, _ ) => show_new_graph( a )    
    }
}

fn show_saved_graph( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.saved_graphs().len() == 0, Error::Ignore ) )
    .write()
    .map_if(
        | a | a.saved_graphs().len() == 1, 
        | a |
        {
            show_saved_graph_idx( a, 0 )
        }
    )
    .map_if( 
        | a | a.saved_graphs().len() > 1, 
        | a |
        {
            a.change_focus( AppFocus::Confirm( Confirm::SavedGraphSelection ) )
        }
    )
    .read()
}

fn show_saved_graph_idx( app : App, idx : usize ) -> App
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.saved_graphs().len() <= idx, Error::Ignore ) )
    .write()
    .map(
        | a |
        {
            let ( a, graph ) = a.own_saved_graph( idx );

            let ( mut a, c ) = a.own_graph_content();

            if let Some( c ) = c
            {
                a = a.save_graph( c );
            }

            a.change_content( AppContent::Graph( graph.unwrap() ) )
        }
    )
    .map( | a | graph_content_generator( a ) )
    .map( | a | a.change_focus( AppFocus::Search ) )
    .own()
}

fn show_new_graph( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.graphs().len() == 0, Error::Ignore ) )
    .write()
    .map_if(
        | a | a.graphs().len() == 1, 
        | a |
        {
            let ( a, graphs ) = a.own_graphs();

            let a = a.change_graphs( graphs.into_iter().map( | g | g.to_current_selected() ).collect() );

            show_selected_graph( a )
        }
    )
    .map_if( 
        | a | a.graphs().len() > 1, 
        | a |
        {
            a.change_focus( AppFocus::Confirm( Confirm::GraphSelection ) )
        }
    )
    .read()
}

fn show_selected_graph( app : App ) -> App
{
    AwpakResult::new( app )
    .validate()
    .zip_result( | a | idx_current_selected_item( a.graphs() ).ok_or( Error::Ignore ) )
    .write()
    .map( 
        | ( a, i ) |
        {
            let mut new_graph = a.graphs()[ *i.as_ref().unwrap() ].inner().clone();

            new_graph.id = Uuid::new_v4().to_string();

            let ( mut a, g ) = a.own_graph_content();

            if let Some( g ) = g
            {
                a = a.save_graph( g );
            }

            ( a.change_content( AppContent::Graph( new_graph ) ), i )
        }
    )
    .map( | ( a, i ) | ( graph_content_generator( a ), i ) )
    .map( | ( a, i ) | ( a.change_focus( AppFocus::Search ), i ) )
    .finalize()
    .unzip( | ( a, _ ) | a )
    .own()
}

fn graph_content_generator( app : App ) -> App
{
    let ( app, generator ) = app.own_content_generator();

    let id = app.graph_content().map( | c | format!( "graph_{}", c.id ) ).unwrap_or( "graph".to_string() );

    let generator = match generator
    {
        ContentGenerator::Detail( g, _ ) => ContentGenerator::Graph( g, id ),
        ContentGenerator::Graph( g, _ ) => ContentGenerator::Graph( g, id ),
        _ => ContentGenerator::Graph( Box::new( generator ), id )
    };

    app.change_content_generator( generator )
}

pub fn pending_graph( app : &App ) -> Option<AwpakTUIGraph>
{
    match app.content()
    {
        AppContent::Graph( g ) => if is_graph_request_pending( &g.request ) { Some( g.clone() ) } else { None },
        _ => None    
    }
}

pub fn graph_to_waiting( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Graph( _ ) =>
        {
            let ( app, graph ) = app.own_graph_content();

            match check_graph_request_is_pending( graph.unwrap() ).collect()
            {
                ( g, None ) => AwpakResult::new( 
                    app.change_content( 
                        AppContent::Graph( 
                            g.change_request( GraphRequest::Waiting ) 
                        )
                    )
                ),
                ( g, Some( e ) ) => AwpakResult::new_err( app.change_content( AppContent::Graph( g ) ), e )
            }
        },
        _ => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn finalize_graph_response( id : Option<String> ) -> impl Fn( App ) -> AwpakResult<App>
{
    move | app |
    {
        match &id
        {
            Some( id ) => finalize_graph_response_with_id( id, app ),
            None => finalize_graph_response_without_id( app )
        }
    }
}

fn finalize_graph_response_with_id( id : &str, app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Graph( g ) if g.id == id =>
        {
            finalize_graph_response_without_id( app )
        },
        _ => finalize_saved_graph_response( id, app )
    }
}

fn finalize_saved_graph_response( id : &str, app : App ) -> AwpakResult<App>
{
    let ( app, saved ) = app.own_saved_graphs();

    let mut changed = false;

    let saved = saved.into_iter()
    .map(
        | g |
        {
            let ( s, g ) = g.own_inner();

            if g.id == id
            {
                changed = true;

                let g = g.change_request( GraphRequest::Empty );

                let g = append_string_to_graph_response( "\n".into(), g );

                s.change_inner( g )
            }
            else
            {
                s.change_inner( g )
            }
        }    
    ).collect::<Vec<_>>();

    match changed
    {
        true => AwpakResult::new( app.change_saved_graphs( saved ) ),
        false => AwpakResult::new_err( app.change_saved_graphs( saved ), Error::Graph( format!( "Graph with id {} not found", id ) ) ),
    }
}

pub fn finalize_graph_response_without_id( app : App ) -> AwpakResult<App>
{
    match app.content()
    {
        AppContent::Graph( _ ) =>
        {
            let ( app, graph ) = app.own_graph_content();
            let graph = graph.unwrap().change_request( GraphRequest::Empty );

            let graph = append_string_to_graph_response( "\n".into(), graph );

            AwpakResult::new( app.change_content( AppContent::Graph( graph ) ) )
        },
        _ => AwpakResult::new_err( app, Error::Ignore )    
    }
}

fn check_graph_request_is_pending( graph : AwpakTUIGraph ) -> AwpakResult<AwpakTUIGraph>
{
    AwpakResult::new( graph )
    .validate()
    .map_result( | c | bool_err( ! is_graph_request_pending( &c.request ), Error::Ignore ) )
    .write()
    .map( 
        | c |
        {
            let ( graph, _ ) = c.own_prompt();

            graph
        }
    )
    .read()
}

pub fn append_text_to_graph_content( id : Option<String>, text : String ) -> impl Fn( App ) -> AwpakResult<App>
{
    move | app |
    {
        match &id
        {
            Some( id ) => append_text_to_graph_with_id( id, app, &text ),
            None => append_text_to_graph_without_id( app, &text )
        }
        
    }
}

fn append_text_to_graph_with_id(
    id : &str,
    app : App,
    text : &str
) -> AwpakResult<App>
{
    let ( app, content ) = app.own_content();

    match content
    {
        AppContent::Graph( c ) if c.id == id => append_text_to_graph( app, c, text ),
        _ => 
        {
            let app = app.change_content( content );

            append_text_to_saved_graph( id, app, &text )
        }
    }
}

fn append_text_to_saved_graph( 
    id : &str,
    app : App,
    text : &str
) -> AwpakResult<App>
{
    let ( app, mut saved ) = app.own_saved_graphs();

    let mut changed = false;

    saved = saved.into_iter()
    .map(
        | s |
        {
            let ( s, inner ) = s.own_inner();

            if inner.id == id
            {
                changed = true;

                s.change_inner( append_string_to_graph_response( text.to_string(), inner ) )
            }
            else
            {
                s.change_inner( inner )
            }
        }
    ).collect();

    match changed
    {
        true => AwpakResult::new( app.change_saved_graphs( saved ) ),
        false => AwpakResult::new_err( app.change_saved_graphs( saved ), Error::Graph( format!( "Graph with id {} not found", id ) ) )
    }
}

fn append_text_to_graph_without_id(
    app : App,
    text : &str
) -> AwpakResult<App>
{
    let ( app, content ) = app.own_content();

    match content
    {
        AppContent::Graph( c ) => append_text_to_graph( app, c, text ),
        _ => AwpakResult::new_err( app.change_content( content ), Error::Ignore )
    }
}

fn append_text_to_graph( app : App, graph : AwpakTUIGraph, text : &str ) -> AwpakResult<App>
{
    AwpakResult::new( app.change_content( AppContent::Graph( append_string_to_graph_response( text.to_string(), graph ) ) ) )
}