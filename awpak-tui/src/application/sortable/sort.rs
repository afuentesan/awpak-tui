use crate::domain::{app::model::app::{App, AppContent, AppFocus}, detail::model::detail::{Detail, DetailContent}, error::Error, result::{functions::result_utils::bool_err, result::AwpakResult}, sortable::model::sortable::SortBy, table::functions::table_sort::{compare_sort_by, sort_table}};


pub fn sort_by_focus( new_sort : SortBy ) -> impl Fn( App ) -> AwpakResult<App>
{
    move | app | match app.focus()
    {
        AppFocus::Sources => sort_sources( app, new_sort ),
        AppFocus::Content |
        AppFocus::Search |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next => sort_content( app, new_sort ),
        AppFocus::Confirm( _ ) |
        AppFocus::Field => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn sort_sources( app : App, new_sort : SortBy ) -> AwpakResult<App>
{
    let ( app, mut rows ) = app.own_rows_sources();

    rows.sort_by( compare_sort_by( new_sort ) );

    let app = app.change_rows_sources( rows );

    AwpakResult::new( app.change_sources_sort( new_sort ) )
}

pub fn sort_content( app : App, new_sort : SortBy ) -> AwpakResult<App>
{
    AwpakResult::new( app )
    .validate()
    .map_result( | a | bool_err( a.content().is_empty(), Error::Ignore ) )
    .write()
    .map(
        | a |
        {
            let ( a, content ) = a.own_content();

            let a = a.change_content_sort( new_sort );

            a.change_content( sort_inner_content( content, new_sort ) )
        }
    )
    .read()
}

fn sort_inner_content( content : AppContent, sort_by : SortBy ) -> AppContent
{
    match content
    {
        AppContent::Empty => AppContent::Empty,
        AppContent::Table( t ) => AppContent::Table( sort_table( t, sort_by ).own() ),
        AppContent::Detail( d ) => sort_inner_detail( *d, sort_by ),
        AppContent::Chat( c ) =>
        {
            AppContent::Chat( c )
        },
        AppContent::Graph( g ) =>
        {
            AppContent::Graph( g )
        }
    }
}

fn sort_inner_detail( detail : Detail, sort_by : SortBy ) -> AppContent
{
    let ( detail, content ) = detail.own_content();

    let content = match content
    {
        DetailContent::Table( t ) => DetailContent::Table( sort_table( t, sort_by ).own() ),
        DetailContent::Empty => content
    };

    AppContent::Detail( Box::new( detail.change_content( content ) ) )
}