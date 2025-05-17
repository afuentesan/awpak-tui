use crate::domain::{error::Error, selectable::model::selectable_item::SelectableItem, table::model::rowable::Rowable};

use super::{header::Header, row::Row};

#[derive(Clone)]
pub struct Table
{
    headers : Vec<Header>,
    rows : Vec<SelectableItem<Row>>
}

impl Table
{
    pub fn new( headers : Vec<Header>, rows : Vec<SelectableItem<Row>> ) -> Result<Self, Error>
    {
        if headers.len() == 0
        {
            return Err( Error::InvalidTableData( "No headers".to_string() ) )
        }

        let rows = rows.into_iter()
        .filter( | r | r.inner().cells().len() == headers.len() )
        .collect::<Vec<_>>();

        Ok(
            Self
            {
                headers,
                rows
            }
        )
    }

    pub fn rows( &self ) -> &Vec<SelectableItem<Row>>
    {
        &self.rows
    }

    pub fn headers( &self ) -> &Vec<Header>
    {
        &self.headers
    }

    // pub fn str_headers( &self ) -> Vec<&str>
    // {
    //     self.headers.iter().map( | h | h.as_str() ).collect()
    // }

    pub fn own_rows( mut self ) -> ( Self, Vec<SelectableItem<Row>> )
    {
        let old = std::mem::replace( &mut self.rows, vec![] );

        ( self, old )
    }

    pub fn change_rows( mut self, new : Vec<SelectableItem<Row>> ) -> Self
    {
        self.rows = new;

        self
    }

    pub fn own_row( mut self, idx : usize ) -> ( Self, Option<SelectableItem<Row>> )
    {
        if idx >= self.rows.len()
        {
            return ( self, None )
        }

        let old = std::mem::replace( &mut self.rows[ idx ], SelectableItem::<Row>::default() );

        ( self, Some( old ) )
    }

    pub fn change_row( mut self, idx : usize, new : SelectableItem<Row> ) -> Self
    {
        if idx >= self.rows.len()
        {
            return self
        }

        self.rows[ idx ] = new;

        self
    }

    // pub fn visible_headers( &self ) -> Vec<usize>
    // {
    //     self.headers.iter()
    //     .enumerate()
    //     .filter( | h | h.1.visible() ).map( | h | h.0 )
    //     .collect()
    // }
}

#[cfg(test)]
mod tests
{
    use crate::domain::{directory::model::directory::Directory, table::model::{cell::{Cell, CellType}, header::HeaderData}};

    use super::*;

    #[test]
    fn test_new_no_headers()
    {
        let table = Table::new(
            vec![], 
            vec![
                SelectableItem::Selected(
                    Row::Directory(
                        Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Cell prueba".to_string() ) ) ] ).unwrap()
                    )
                )
            ]
        );

        assert!( table.is_err() );
    }

    #[test]
    fn test_new()
    {
        let table = Table::new(
            vec![ Header::Visible( HeaderData::new( "header_prueba", "Header prueba" ) ) ], 
            vec![
                SelectableItem::Selected(
                    Row::Directory(
                        Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Cell prueba".to_string() ) ) ] ).unwrap()
                    )
                )
            ]
        );

        assert!( table.is_ok() );

        let table = table.unwrap();

        assert_eq!( table.headers.len(), 1 );

        assert_eq!( table.headers[ 0 ].to_string(), "Header prueba" );

        assert_eq!( table.rows.len(), 1 );

        assert_eq!( table.rows[ 0 ].inner().cells().len(), 1 );

        assert_eq!( table.rows[ 0 ].inner().cells()[ 0 ].inner().to_string(), "Cell prueba" );
    }

    #[test]
    fn test_new_different_num_headers_and_cells()
    {
        let table = Table::new(
            vec![ Header::Visible( HeaderData::new( "header_1", "Header 1" ) ), Header::Visible( HeaderData::new( "header_2", "Header 2" ) ) ], 
            vec![
                SelectableItem::Selected(
                    Row::Directory(
                        Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Cell prueba".to_string() ) ) ] ).unwrap()
                    )
                )
            ]
        );

        assert!( table.is_ok() );

        let table = table.unwrap();

        assert_eq!( table.headers.len(), 2 );

        assert_eq!( table.headers[ 0 ].to_string(), "Header 1" );
        assert_eq!( table.headers[ 1 ].to_string(), "Header 2" );
        assert_eq!( table.headers[ 0 ].as_id(), "header_1" );
        assert_eq!( table.headers[ 1 ].as_id(), "header_2" );

        assert_eq!( table.rows.len(), 0 );
    }
}