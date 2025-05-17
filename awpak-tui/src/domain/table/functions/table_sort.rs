use std::cmp::Ordering;

use crate::domain::{error::Error, result::{functions::result_utils::bool_err, result::AwpakResult}, selectable::model::selectable_item::SelectableItem, sortable::model::sortable::{SortBy, Sortable}, table::model::{row::Row, rowable::Rowable, table::Table}};


pub fn sort_table( table : Table, sort_by : SortBy ) -> AwpakResult<Table>
{
    let cmp = compare_sort_by( sort_by );

    AwpakResult::new( table )
    .validate()
    .map_result( | t | bool_err( t.rows().len() == 0, Error::Ignore ) )
    .write()
    .map( 
        | t | 
        {
            let ( t, mut rows ) = t.own_rows();

            rows.sort_by( cmp );

            t.change_rows( rows )
        }
    )
    .read()
}

pub fn compare_sort_by( sort_by : SortBy ) -> impl Fn( &SelectableItem<Row>, &SelectableItem<Row> ) -> Ordering
{
    move | a, b | compare( a, b, sort_by )
}

fn compare( a : &SelectableItem<Row>, b : &SelectableItem<Row>, sort_by : SortBy ) -> Ordering
{
    a.inner().sort( b.inner(), sort_by )
}

pub fn default_row_sort( a : &Row, b : &Row ) -> Ordering
{
    match ( a.sort_preference(), b.sort_preference() )
    {
        ( aw, bw ) if aw == bw => row_str_cmp( a, 0 ).cmp( &row_str_cmp( b, 0 ) ),
        ( aw, bw ) if aw > bw => Ordering::Greater,
        ( aw, bw ) if aw < bw => Ordering::Less,
        _ => Ordering::Equal
    }
}

fn row_str_cmp( r : &Row, idx : usize ) -> String
{
    r.cells()[ idx ].inner().to_string()
}

pub fn sort_row_by_column( a : &Row, b : &Row, idx : usize ) -> Ordering
{
    if idx >= a.cells().len()
    {
        return default_row_sort( a, b )
    }

    row_str_cmp( a, idx ).cmp( &row_str_cmp( b, idx ) )
}