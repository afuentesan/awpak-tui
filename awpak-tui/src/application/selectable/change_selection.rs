use awpak_tui_ai::domain::agent::agent::AIAgent;

use crate::{application::app::change_focus::{next_focus, previous_focus}, domain::{app::model::app::{App, AppContent, AppFocus, Confirm}, detail::model::detail::{Detail, DetailContent}, error::Error, result::result::AwpakResult, selectable::{functions::change_selectable::{append_or_remove_next_selection, append_or_remove_previous_selection, select_next_or_first_or_none_if_all_hidden, select_previous_or_last_or_none_if_all_hidden}, model::selectable_item::SelectableItem}, table::model::table::Table}};

pub fn append_or_remove_next_in_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Content => append_or_remove_next_in_content( app ),
        AppFocus::Sources |
        AppFocus::Search |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Confirm( _ ) |
        AppFocus::Field => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn append_or_remove_previous_in_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Content => append_or_remove_previous_in_content( app ),
        AppFocus::Sources |
        AppFocus::Search |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Confirm( _ ) |
        AppFocus::Field => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn select_next_in_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Sources => select_next_in_sources( app ),
        AppFocus::Content => select_next_in_content( app ),
        AppFocus::Search => select_next_in_search( app ),
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next => select_next_in_navigation( app ),
        AppFocus::Confirm( c ) => select_next_in_confirm( app, c ),
        AppFocus::Field => AwpakResult::new_err( app, Error::Ignore )
    }
}

pub fn select_previous_in_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Sources => select_previous_in_sources( app ),
        AppFocus::Content => select_previous_in_content( app ),
        AppFocus::Search => select_previous_in_search( app ),
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next => AwpakResult::new_err( app, Error::Ignore ),
        AppFocus::Confirm( c ) => select_previous_in_confirm( app, c ),
        AppFocus::Field => AwpakResult::new_err( app, Error::Ignore )
    }
}

fn select_previous_in_confirm( app : App, confirm : Confirm ) -> AwpakResult<App>
{
    change_confirm_selection(
        app,
        confirm,
        select_previous_or_last_or_none_if_all_hidden
    )
}

fn select_next_in_confirm( app : App, confirm : Confirm ) -> AwpakResult<App>
{
    change_confirm_selection(
        app,
        confirm,
        select_next_or_first_or_none_if_all_hidden
    )
}

fn change_confirm_selection( 
    app : App,
    confirm : Confirm,
    fn_select : impl Fn( Vec<SelectableItem<AIAgent>> ) -> Vec<SelectableItem<AIAgent>>
) -> AwpakResult<App>
{
    match confirm
    {
        Confirm::MovibleAction => AwpakResult::new_err( app, Error::Ignore ),
        Confirm::AgentSelection =>
        {
            let ( app, agents ) = app.own_ai_agents();

            let agents = fn_select( agents );

            AwpakResult::new( app.change_ai_agents( agents ) )
        }
    }
}

fn select_next_in_navigation( app : App ) -> AwpakResult<App>
{
    AwpakResult::new( app.change_focus( AppFocus::Sources ) )
}

fn select_next_in_search( app : App ) -> AwpakResult<App>
{
    next_focus( app )
}

fn select_previous_in_search( app : App ) -> AwpakResult<App>
{
    previous_focus( app )
}

fn select_next_in_sources( app : App ) -> AwpakResult<App>
{
    let ( app, rows ) = app.own_rows_sources();

    let rows = select_next_or_first_or_none_if_all_hidden( rows );

    AwpakResult::new( app.change_rows_sources( rows ) )
}

fn select_previous_in_sources( app : App ) -> AwpakResult<App>
{
    let ( app, rows ) = app.own_rows_sources();

    let rows = select_previous_or_last_or_none_if_all_hidden( rows );

    AwpakResult::new( app.change_rows_sources( rows ) )
}

fn select_next_in_content( app : App ) -> AwpakResult<App>
{
    change_content_selection( app, select_next_in_table_content )
}

fn select_previous_in_content( app : App ) -> AwpakResult<App>
{
    change_content_selection( app, select_previous_in_table_content )
}

fn append_or_remove_next_in_content( app : App ) -> AwpakResult<App>
{
    change_content_selection( app, append_or_remove_next_in_table_content )
}

fn append_or_remove_previous_in_content( app : App ) -> AwpakResult<App>
{
    change_content_selection( app, append_or_remove_previous_in_table_content )
}

fn change_content_selection( 
    app : App,
    fn_append_or_remove : impl Fn( Table ) -> Table
) -> AwpakResult<App>
{
    let ( app, content ) = app.own_content();

    match content
    {
        AppContent::Table( t ) =>
        {
            let app = app.change_content( AppContent::Table( fn_append_or_remove( t ) ) );

            AwpakResult::new( app )
        },
        AppContent::Detail( d ) => change_detail_selection( app, *d, fn_append_or_remove ),
        AppContent::Empty => AwpakResult::new( app ),
        AppContent::Chat( c ) =>
        {
            AwpakResult::new_err( 
                app.change_content( AppContent::Chat( c ) ), 
                Error::Ignore
            )
        }
    }
}

fn change_detail_selection( 
    app : App, 
    detail : Detail,
    table_selection : impl Fn( Table ) -> Table
) -> AwpakResult<App>
{
    let ( detail, content ) = detail.own_content();

    let content = match content
    {
        DetailContent::Table( t ) => DetailContent::Table( table_selection( t ) ),
        DetailContent::Empty => content
    };

    let detail = AppContent::Detail( Box::new( detail.change_content( content ) ) );

    AwpakResult::new( app.change_content( detail ) )
}

fn select_next_in_table_content( table : Table ) -> Table
{
    let ( content, rows ) = table.own_rows();

    content.change_rows(
        select_next_or_first_or_none_if_all_hidden( rows )
    )
}

fn select_previous_in_table_content( table : Table ) -> Table
{
    let ( content, rows ) = table.own_rows();

    content.change_rows(
        select_previous_or_last_or_none_if_all_hidden( rows )
    )
}

fn append_or_remove_next_in_table_content( table : Table ) -> Table
{
    let ( content, rows ) = table.own_rows();

    content.change_rows(
        append_or_remove_next_selection( rows )
    )
}

fn append_or_remove_previous_in_table_content( table : Table ) -> Table
{
    let ( content, rows ) = table.own_rows();

    content.change_rows(
        append_or_remove_previous_selection( rows )
    )
}

