use crate::domain::{detail::model::detail::DetailContent, error::Error, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, header::{Header, HeaderData}, row::Row, rowable::Rowable, table::Table}};

use super::table_selection::current_selected_row;

pub fn detail_content_from_table( table : &Table ) -> Result<DetailContent, Error>
{
    let row = current_selected_row( table ).ok_or( Error::Ignore )?;

    let rows = row.cells().into_iter().zip( table.headers().iter() )
    .map( | ( c, h ) | SelectableItem::Idle( detail_row( c, h ) ) )
    .collect::<Vec<_>>();

    let headers = vec![ 
        Header::Visible( HeaderData::new( "name", "Name" ) ), 
        Header::Visible( HeaderData::new( "value", "Value" ) ) 
        ];

    Ok(
        DetailContent::Table( Table::new( headers, rows )? )
    )
}

fn detail_row( cell : &SelectableItem<Cell>, header : &Header ) -> Row
{
    Row::DataRef
    { 
        id : header.as_id().to_string(),
        cells : vec![
            SelectableItem::Idle( Cell::Read( CellType::String( header.to_string() ) ) ),
            cell.clone()
        ]
    }
}