use crate::domain::{app::model::app::{App, AppContent, AppFocus}, detail::model::detail::{Detail, DetailContent}, error::Error, result::result::AwpakResult, table::{functions::table_selection::{select_next_cell_in_selected_row, select_previous_cell_in_selected_row}, model::table::Table}};

const NUM_ELEMS : usize = 6;
const LAST_IND : usize = NUM_ELEMS - 1;
const ORDER : [ AppFocus; NUM_ELEMS ] = [ AppFocus::Sources, AppFocus::Search, AppFocus::Content, AppFocus::Back, AppFocus::Next, AppFocus::Up ];

pub fn next_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Content => change_focus_in_content( app, select_next_cell_in_selected_row, default_next_focus ),
        _ => default_next_focus( app )
    }
}

fn default_next_focus( app : App ) -> AwpakResult<App>
{
    focus(
        app, 
        | i | ORDER[ if i == LAST_IND { 0 } else { i + 1 } ]
    )
}

pub fn previous_focus( app : App ) -> AwpakResult<App>
{
    match app.focus()
    {
        AppFocus::Content => change_focus_in_content( app, select_previous_cell_in_selected_row, default_previous_focus ),
        _ => default_previous_focus( app )
    }
}

fn default_previous_focus( app : App ) -> AwpakResult<App>
{
    focus(
        app, 
        | i | ORDER[ if i == 0 { LAST_IND } else { i - 1 } ]
    )
}

fn change_focus_in_content( 
    app : App,
    fn_change_table_sel : impl Fn( Table ) -> AwpakResult<Table>,
    fn_default : impl Fn( App ) -> AwpakResult<App>
) -> AwpakResult<App>
{
    let ( app, content ) = app.own_content();

    match content
    {
        AppContent::Table( t ) => change_focus_in_content_table_or_default( app, t, fn_change_table_sel, fn_default ),
        AppContent::Detail( d ) => change_focus_in_detail( app, *d, fn_change_table_sel, fn_default ),
        AppContent::Empty => fn_default( app ),
        AppContent::Graph( g ) => fn_default( app.change_content( AppContent::Graph( g ) ) )
    }
}

fn change_focus_in_detail( 
    app : App, 
    detail : Detail,
    fn_change_table_sel : impl Fn( Table ) -> AwpakResult<Table>,
    fn_default : impl Fn( App ) -> AwpakResult<App> 
) -> AwpakResult<App>
{
    let ( detail, content ) = detail.own_content();

    match content
    {
        DetailContent::Table( t ) =>
        {
            change_focus_in_detail_or_default( app, change_focus_in_detail_table( detail, t, fn_change_table_sel ), fn_default )
        },
        DetailContent::Empty => fn_default( app.change_content( AppContent::Detail( Box::new( detail ) ) ) )
    }
}

fn change_focus_in_detail_or_default( 
    app : App, 
    detail : AwpakResult<Detail>,
    fn_default : impl Fn( App ) -> AwpakResult<App> 
) -> AwpakResult<App>
{
    match detail.collect()
    {
        ( detail, None ) => AwpakResult::new( app.change_content( AppContent::Detail( Box::new( detail ) ) ) ),
        ( detail, Some( _ ) ) => fn_default( app.change_content( AppContent::Detail( Box::new( detail ) ) ) )
    }
}

fn change_focus_in_detail_table( 
    detail : Detail, 
    table : Table,
    fn_change_table_sel : impl Fn( Table ) -> AwpakResult<Table> 
) -> AwpakResult<Detail>
{
    match fn_change_table_sel( table ).collect()
    {
        ( table, None ) => AwpakResult::new( detail.change_content( DetailContent::Table( table ) ) ),
        ( table, Some( e ) ) => AwpakResult::new_err( detail.change_content( DetailContent::Table( table ) ), e )
    }
}

fn change_focus_in_content_table_or_default( 
    app : App, 
    table : Table,
    fn_change_table_sel : impl Fn( Table ) -> AwpakResult<Table>,
    fn_default : impl Fn( App ) -> AwpakResult<App>
) -> AwpakResult<App>
{
    match change_focus_in_content_table( app, table, fn_change_table_sel ).collect()
    {
        ( app, None ) => AwpakResult::new( app ),
        ( app, Some( _ ) ) => fn_default( app )
    }
}

fn change_focus_in_content_table( 
    app : App, 
    table : Table,
    fn_change_table_sel : impl Fn( Table ) -> AwpakResult<Table>
) -> AwpakResult<App>
{
    match fn_change_table_sel( table ).collect()
    {
        ( table, None ) =>
        {
            AwpakResult::new( app.change_content( AppContent::Table( table ) ) )
        },
        ( table, Some( e ) ) =>
        {
            AwpakResult::new_err( app.change_content( AppContent::Table( table ) ), e )
        }
    }
} 

fn focus(
    app : App,
    fn_select_focus : impl Fn( usize ) -> AppFocus
) -> AwpakResult<App>
{
    let focus = app.focus();

    match focus
    {
        AppFocus::Confirm( _ ) |
        AppFocus::Field => return AwpakResult::new_err( app, Error::Ignore ),
        _ => {}
    };
    
    AwpakResult::new(
        app
        .change_focus( 
            ORDER.iter()
            .enumerate()
            .find( 
                | ( _, e ) | **e == focus
            )
            .map(
                | ( i, _ ) | fn_select_focus( i )
            )
            .unwrap_or( AppFocus::Sources )
        )
    )
}

#[cfg(test)]
mod tests
{
    use crate::{application::app::init_app::init_app, domain::{directory::model::directory::Directory, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, header::{Header, HeaderData}, row::Row, table::Table}}};

    use super::*;

    fn test_app() -> Result<App, crate::domain::error::Error>
    {
        init_app(
            Table::new(
                vec![ Header::Visible( HeaderData { id : "sources".to_string(), name : "Sources".to_string() } ) ], 
                vec![ 
                    SelectableItem::CurrentSelected(
                        Row::Directory( 
                            Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Tmp".to_string() ) ) ] ).unwrap() 
                        ) 
                    )
                    ]
            )?
        )
    }

    #[test]
    fn test_next_focus()
    {
        let app = test_app();

        assert!( app.is_ok() );

        let app = next_focus( app.unwrap() );

        match app.collect()
        {
            ( a, None ) => assert_eq!( a.focus(), AppFocus::Content ),
            _ => assert!( false, "next focus err" )
        }

        let app = test_app();

        assert!( app.is_ok() );

        let app = app.unwrap().change_focus( AppFocus::Content );

        let app = next_focus( app );

        match app.collect()
        {
            ( a, None ) => assert_eq!( a.focus(), AppFocus::Back ),
            _ => assert!( false, "next focus err" )
        }
    }

    #[test]
    fn test_previous_focus()
    {
        let app = test_app();

        assert!( app.is_ok() );

        let app = previous_focus( app.unwrap() );

        match app.collect()
        {
            ( a, None ) => assert_eq!( a.focus(), AppFocus::Sources ),
            _ => assert!( false, "next focus err" )
        }

        let app = test_app();

        assert!( app.is_ok() );

        let app = app.unwrap().change_focus( AppFocus::Content );

        let app = previous_focus( app );

        match app.collect()
        {
            ( a, None ) => assert_eq!( a.focus(), AppFocus::Search ),
            _ => assert!( false, "next focus err" )
        }
    }
}