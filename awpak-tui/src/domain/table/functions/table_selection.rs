use crate::domain::{error::Error, result::result::AwpakResult, selectable::{functions::{change_selectable::{change_selection_from_to, change_to_idle}, selectable_utils::{idx_current_selected_item, idx_next_or_first_selectable_item, idx_previous_selectable_item_stop_at_start}}, model::{selectable::Selectable, selectable_item::SelectableItem}}, table::model::{cell::Cell, row::Row, rowable::Rowable, table::Table}};

pub fn current_selected_row_and_cell( table : &Table ) -> Option<( usize, usize )>
{
    idx_current_selected_item( table.rows() ).iter().flat_map(
        | i |
        {
            idx_current_selected_item( table.rows()[ *i ].cells() )
            .map( | x | ( *i, x ) )
        }
    )
    .next()
}

pub fn current_selected_row( table : &Table ) -> Option<&Row>
{
    table.rows().iter()
    .find( | s | s.current_selected() )
    .map( | s | s.inner() )
}

pub fn select_next_cell_in_selected_row( table : Table ) -> AwpakResult<Table>
{
    match idx_current_selected_item( table.rows() )
    {
        Some( idx ) => change_cell_selection_in_idx( table, idx, select_next_cell_in_row ),
        None => AwpakResult::new_err( table, Error::Ignore )    
    }
}

pub fn select_previous_cell_in_selected_row( table : Table ) -> AwpakResult<Table>
{
    match idx_current_selected_item( table.rows() )
    {
        Some( idx ) => change_cell_selection_in_idx( table, idx, select_previous_cell_in_row ),
        None => AwpakResult::new_err( table, Error::Ignore )    
    }
}

fn change_cell_selection_in_idx( 
    table : Table, 
    idx : usize,
    fn_select_cell : impl Fn( SelectableItem<Row> ) -> AwpakResult<SelectableItem<Row>>
) -> AwpakResult<Table>
{
    let ( table, row ) = table.own_row( idx );

    match fn_select_cell( row.unwrap() ).collect()
    {
        ( row, None ) =>
        {
            AwpakResult::new( table.change_row( idx, row ) )
        },
        ( row, Some( e ) ) =>
        {
            let table = table.change_row( idx, clear_row_selection( row ) );

            AwpakResult::new_err( table, e )
        }
    }
}

fn clear_row_selection( row : SelectableItem<Row> ) -> SelectableItem<Row>
{
    let ( row, cells ) = row.own_cells();

    let cells = cells.into_iter()
    .map( 
        | c | 
        {
            if c.selected() || c.current_selected()
            {
                c.to_idle()
            }
            else { c }
        }
    )
    .collect::<Vec<_>>();

    *row.change_cells( cells )
}

fn select_next_cell_in_row( row : SelectableItem<Row> ) -> AwpakResult<SelectableItem<Row>>
{
    let idx = idx_current_selected_item( row.cells() );

    select_next_cell_in_row_from( row, idx )
}

fn select_next_cell_in_row_from( row : SelectableItem<Row>, idx : Option<usize> ) -> AwpakResult<SelectableItem<Row>>
{
    let ( row, cells ) = row.own_cells();

    match select_next_cell_in_cells_from( cells, idx ).collect()
    {
        ( cells, None ) => AwpakResult::new( *row.change_cells( cells ) ),
        ( cells, Some( e ) ) => AwpakResult::new_err( *row.change_cells( cells ), e )
    }
}

fn select_next_cell_in_cells_from( cells : Vec<SelectableItem<Cell>>, idx : Option<usize> ) -> AwpakResult<Vec<SelectableItem<Cell>>>
{
    let to = idx_next_or_first_selectable_item( &cells );

    change_selection_from_to( cells, idx, to )
}

fn select_previous_cell_in_row( row : SelectableItem<Row> ) -> AwpakResult<SelectableItem<Row>>
{
    match idx_current_selected_item( row.cells() )
    {
        Some( idx ) => select_previous_cell_in_row_from( row, idx ),
        None => AwpakResult::new_err( row, Error::Ignore )
    }
}

fn select_previous_cell_in_row_from( row : SelectableItem<Row>, idx : usize ) -> AwpakResult<SelectableItem<Row>>
{
    let ( row, cells ) = row.own_cells();

    match select_previous_cell_in_cells_from( cells, idx ).collect()
    {
        ( cells, None ) => AwpakResult::new( *row.change_cells( cells ) ),
        ( cells, Some( e ) ) => AwpakResult::new_err( *row.change_cells( cells ), e )
    }
}

fn select_previous_cell_in_cells_from( cells : Vec<SelectableItem<Cell>>, idx : usize ) -> AwpakResult<Vec<SelectableItem<Cell>>>
{
    match idx_previous_selectable_item_stop_at_start( &cells, idx )
    {
        Some( previous ) => change_selection_from_to( cells, Some( idx ), Some( previous ) ),
        None => AwpakResult::new( change_to_idle( cells, idx ) )
    }
}

// pub fn select_first_cell_in_selected_row( table : Table ) -> AwpakResult<Table>
// {
//     AwpakResult::new( table )
//     .validate()
//     .zip_result( | t | idx_current_selected_item( t.rows() ).ok_or( Error::Ignore ) )
//     .map_result( 
//         | ( t, i ) | 
//         bool_err( idx_first_selectable_item( t.rows()[ *i.as_ref().unwrap() ].cells() ).is_none(), Error::Ignore )
//     )
//     .write()
//     .map( 
//         | ( t, i ) | 
//         {
//             let i = i.unwrap();

//             let ( t, row ) = t.own_row( i );

//             let s = select_first_cell_in_row( row.unwrap() );

//             let t = t.change_row(  i, Some( s ) );

//             ( t, Ok( i ) )
//         }
//     )
//     .finalize()
//     .unzip( | ( t, _ ) | t )
//     .read()
// }

// fn select_first_cell_in_row( row : SelectableItem<Row> ) -> SelectableItem<Row>
// {
//     let ( row, cells ) = row.own_cells();

//     let cells = match select_first_selectable_item( cells )
//     {
//         Ok( r ) => r,
//         Err( ( r, _ ) ) => r
//     };

//     *row.change_cells( cells )
// }

// pub fn idx_first_selected_row<T>( items : &Vec<T> ) -> Option<usize>
// where T: Selectable
// {
//     items.iter().enumerate()
//     .find( | s | s.1.selected() )
//     .map( | s | s.0 )
// }

// pub fn idx_last_selected_row<T>( items : &Vec<T> ) -> Option<usize>
// where T: Selectable
// {
//     items.iter().enumerate()
//     .rev()
//     .find( | s | s.1.selected() )
//     .map( | s | s.0 )
// }

#[cfg(test)]
mod tests
{
    use crate::domain::{directory::model::directory::Directory, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, header::{Header, HeaderData}, rowable::Rowable}};

    use super::*;

    #[test]
    fn test_no_first_selected_row()
    {
        let table = Table::new( 
            vec![ Header::Visible( HeaderData::new( "prueba", "Prueba" ) ) ], 
            vec![ 
                SelectableItem::Idle(
                    Row::Directory( 
                        Directory::new( 
                            "/tmp", 
                            vec![ Cell::Read( CellType::String( "Item 1".to_string() ) ) ] 
                        ).unwrap() 
                    ) 
                ),
                SelectableItem::Idle(
                    Row::Directory( 
                        Directory::new( 
                            "/home/angel", 
                            vec![ Cell::Read( CellType::String( "Item 2".to_string() ) ) ] 
                        ).unwrap() 
                    ) 
                )
            ]
        ).unwrap();

        assert!( current_selected_row( &table ).is_none() );

        let table = Table::new(
            vec![ Header::Visible( HeaderData::new( "header_prueba", "Header prueba" ) ) ], 
            vec![
                SelectableItem::Idle(
                    Row::Directory(
                        Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Cell prueba".to_string() ) ) ] ).unwrap()
                    )
                )
            ]
        );

        assert!( current_selected_row( &table.unwrap() ).is_none() );
    }

    #[test]
    fn test_first_selected_row()
    {
        let table = Table::new( 
            vec![ Header::Visible( HeaderData::new( "prueba", "Prueba" ) ) ], 
            vec![ 
                SelectableItem::Idle(
                    Row::Directory( 
                        Directory::new( 
                            "/tmp", 
                            vec![ Cell::Read( CellType::String( "Item 1".to_string() ) ) ] 
                        ).unwrap() 
                    ) 
                ),
                SelectableItem::CurrentSelected(
                    Row::Directory( 
                        Directory::new( 
                            "/home/angel", 
                            vec![ Cell::Read( CellType::String( "Item 2".to_string() ) ) ] 
                        ).unwrap() 
                    ) 
                )
            ]
        ).unwrap();

        let first_selected = current_selected_row( &table );

        assert!( first_selected.is_some() );

        let first_selected = first_selected.unwrap();

        let cells = first_selected.cells();

        assert_eq!( cells.len(), 1 );

        assert_eq!( cells[ 0 ].inner().to_string(), "Item 2" );

    }
}