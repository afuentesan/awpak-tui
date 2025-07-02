use crate::domain::{app::{functions::change_content::{change_app_content, reload_app_content}, model::app::{App, AppContent}}, content_generator::{functions::generator::{generate_content, parent_of_generator}, model::content_generator::ContentGenerator}, error::Error, result::{functions::result_utils::bool_err, result::AwpakResult}};

pub fn back_from_graph( app : App ) -> AwpakResult<App>
{
    match app.graph_content_generator()
    {
        Some( ( parent, _ ) ) =>
        {
            let parent = parent.clone();

            close_graph( app, parent )
        },
        None => AwpakResult::new_err( app, Error::Ignore )
    }
}

fn close_graph( app : App, parent : ContentGenerator ) -> AwpakResult<App>
{
    let app = app.change_content_generator( parent );

    let ( app, graph ) = app.own_graph_content();

    let app = match graph
    {
        Some( g ) => app.save_graph( g ),
        _ => app    
    };

    AwpakResult::new( reload_app_content( app ) )
}

pub fn back_from_detail( app : App ) -> AwpakResult<App>
{
    match app.detail_content_generator()
    {
        Some( ( parent, _ ) ) =>
        {
            let parent = parent.clone();

            close_detail( app, parent )
        },
        None => AwpakResult::new_err( app, Error::Ignore )
    }
}

fn close_detail( app : App, parent : ContentGenerator ) -> AwpakResult<App>
{
    let ( app, detail ) = app.own_detail_content();

    match detail
    {
        Some( d ) => AwpakResult::new( app.change_content( d.own_source().1 ).change_content_generator( parent ) ),
        None => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn parent_navigation( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.focus().is_confirm(), Error::Ignore ) )
    .map_result( | a | bool_err( a.content_generator().is_empty(), Error::Ignore ) )
    .zip_result( | a | parent_of_generator( a.content_generator() ) )
    .zip_result( 
        | ( _, g ) | 
        generate_content( g.as_ref().unwrap() )
    )
    .write()
    .map(
        | ( ( a, g ), c ) | 
        ( 
            ( 
                direct_navigation( a, g.unwrap(), c.unwrap() ).own(), 
                Ok( ContentGenerator::Empty ) 
            ), 
            Ok( AppContent::Empty ) 
        )
    )
    .finalize()
    .unzip( | a | a.0.0 )
    .read()
}

pub fn direct_navigation(
    app : App,
    generator : ContentGenerator,
    content : AppContent
) -> AwpakResult<App>
{
    let fn_history_own = | a : App | a.own_history_back();
    let fn_history_change = | a : App, h | a.change_history_back( h );

    AwpakResult::new( app )
    .write()
    .map( | a | a.own_history_next().0 )
    .map_if(
        | a | ! generator.eq( a.content_generator() ), 
        | a | 
        current_generator_to_history( a, fn_history_own, fn_history_change ) 
    )
    .map( | a | a.change_content_generator( generator ) )
    .map( | a | change_app_content( a, content ) )
    .read()

}

pub fn history_next( app : App ) -> AwpakResult<App>
{
    history_navigation(
        app, 
        | a | a.history_next(), 
        | a | a.own_history_next(), 
        | a, h | a.change_history_next( h ), 
        | a | a.own_history_back(), 
        | a, h | a.change_history_back( h )
    )
}

pub fn history_back( app : App ) -> AwpakResult<App>
{
    history_navigation(
        app, 
        | a | a.history_back(), 
        | a | a.own_history_back(), 
        | a, h | a.change_history_back( h ), 
        | a | a.own_history_next(), 
        | a, h | a.change_history_next( h )
    )
}

fn new_history_result( app : App ) -> AwpakResult<App>
{
    match app.content_generator()
    {
        ContentGenerator::Directory( _ ) |
        ContentGenerator::ExecutableExpandable( _ ) |
        ContentGenerator::Expandable( _ ) |
        ContentGenerator::Empty => AwpakResult::new( app ),
        ContentGenerator::Detail( _, _ ) => AwpakResult::new_err( app, Error::Ignore ),
        ContentGenerator::Graph( _, _ ) => AwpakResult::new_err( app, Error::Ignore )
    }
}

fn history_navigation(
    app : App,
    history_read : impl Fn( &App ) -> Vec<&ContentGenerator>,
    history_own : impl Fn( App ) -> ( App, Vec<ContentGenerator> ),
    history_change : impl Fn( App, Vec<ContentGenerator> ) -> App,
    other_history_own : impl Fn( App ) -> ( App, Vec<ContentGenerator> ),
    other_history_change : impl Fn( App, Vec<ContentGenerator> ) -> App
) -> AwpakResult<App>
{
    new_history_result( app )
    .validate()
    .map_result( | a | bool_err( a.focus().is_confirm(), Error::Ignore ) )
    .map_result( | a | bool_err( history_read( a ).len() == 0, Error::Ignore ) )
    .zip_result( | a | generate_content( history_read( a ).last().unwrap() ) )
    .write()
    .map( 
        | ( a, c ) | 
        ( current_generator_to_history( a, &other_history_own, &other_history_change ), c ) 
    )
    .map( 
        | ( a, c ) | 
        ( history_pop_to_current( a, &history_own, &history_change ), c ) 
    )
    .map( | a | ( change_app_content( a.0, a.1.unwrap() ), Ok( AppContent::Empty ) ) )
    .finalize()
    .unzip( | a | a.0 )
    .read()
}

fn history_pop_to_current( 
    app : App,
    history_own : impl Fn( App ) -> ( App, Vec<ContentGenerator> ),
    history_change : impl Fn( App, Vec<ContentGenerator> ) -> App
) -> App
{
    let ( app, mut history ) = history_own( app );

    let app = app.change_content_generator( history.pop().unwrap() );

    history_change( app, history )
}

fn current_generator_to_history( 
    app : App,
    other_history_own : impl Fn( App ) -> ( App, Vec<ContentGenerator> ),
    other_history_change : impl Fn( App, Vec<ContentGenerator> ) -> App
) -> App
{
    let ( app, current ) = app.own_content_generator();

    let ( app, history ) = other_history_own( app );

    let history = push_if_not_equal_and_not_empty( history, current.own_without_detail() );

    other_history_change( app, history )
}

fn push_if_not_equal_and_not_empty( mut contents : Vec<ContentGenerator>, new : ContentGenerator ) -> Vec<ContentGenerator>
{
    if new.is_empty()
    {
        return contents
    }

    if ! new.try_eq( contents.last() )
    {
        contents.push( new );
    }

    contents
}