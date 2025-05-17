use regex::Regex;

use crate::domain::{selectable::model::{selectable::Selectable, selectable_item::SelectableItem}, table::model::{row::Row, rowable::Rowable, table::Table}};


pub fn show_hide_rows_by_regex( table : Table, search : &str ) -> Table
{
    let search = search.trim();

    if search == ""
    {
        return show_all( table );
    }

    let ( table, rows ) = table.own_rows();

    let rows = rows.into_iter()
    .map(
        | selectable_row |
        {
            let str_found = search_regex_in_row( selectable_row.inner(), search );

            match ( selectable_row, str_found )
            {
                ( r , false ) => r.to_hidden(),
                ( SelectableItem::Hidden( r ), true ) => SelectableItem::Idle( r ),
                ( r, true ) => r
            }
        }
    )
    .collect::<Vec<_>>();
    
    table.change_rows( rows )
}

fn show_all( table : Table ) -> Table
{
    let ( table, rows ) = table.own_rows();

    let rows = rows.into_iter()
    .map( 
        | r | if r.hidden() { r.to_idle() } else { r }
    )
    .collect::<Vec<_>>();

    table.change_rows( rows )
}

fn search_regex_in_row( row : &Row, str : &str ) -> bool
{
    match &Regex::new( str )
    {
        Ok( r ) => 
        {
            row.cells().iter()
            .find( | c |  r.is_match( c.inner().to_string().as_str() ) )
            .is_some()
        },
        _ => false
    }
}