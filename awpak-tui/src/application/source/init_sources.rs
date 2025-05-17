use crate::domain::{error::Error, selectable::model::selectable_item::SelectableItem, table::model::{header::{Header, HeaderData}, row::Row, table::Table}};

// pub fn change_sources( rows : Vec<Row> ) -> impl FnOnce( App ) -> AwpakResult<App>
// {
//     move | a |
//     {
//         match table_sources( rows )
//         {
//             Ok( t ) => AwpakResult::new( a.change_sources( t ) ),
//             Err( e ) => AwpakResult::new_err( a, e )
//         }
//     }
// }

pub fn table_sources( rows : Vec<Row> ) -> Result<Table, Error>
{
    let headers = vec![ Header::Visible( HeaderData { id : "sources".to_string(), name : "Sources".to_string() } ) ];

    let rows = rows.into_iter().enumerate().map(
        | ( i, r ) |
        {
            if i == 0 { SelectableItem::CurrentSelected( r ) } else { SelectableItem::Idle( r ) }
        }
    )
    .collect::<Vec<_>>();

    Table::new( headers, rows )
}

// pub fn init_sources() -> Result<Table, Error>
// {
//     let home = match path_for_home()
//     {
//         Ok( h ) => Some( Row::Directory( 
//             Directory::new( h.as_str(), vec![ Cell::Read( CellType::String( "Home".to_string() ) ) ] )?
//         ) ),
//         _ => None
//     };

//     let mut rows = vec![];

//     if let Some( home ) = home
//     {
//         rows.push( home );
//     }

//     let tmp = Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "/tmp".to_string() ) ) ] );

//     if let Ok( tmp ) = tmp
//     {
//         rows.push( Row::Directory( tmp ) );
//     }

//     let dirs = Expandable::new( "/home/angel/util/index.csv", vec![ Cell::Read( CellType::String( "Directorios".to_string() ) ) ] );

//     if let Ok( dirs ) = dirs
//     {
//         rows.push( Row::Expandable( dirs ) );
//     }

//     let trab = Expandable::new( "/home/angel/trabajo/scripts/index.csv", vec![ Cell::Read( CellType::String( "Trabajo".to_string() ) ) ] );

//     if let Ok( trab ) = trab
//     {
//         rows.push( Row::Expandable( trab ) );
//     }

//     table_sources( rows )
// }

// #[cfg(test)]
// mod tests
// {
//     use super::*;

//     #[test]
//     fn test_init_sources()
//     {
//         let sources = init_sources();

//         assert!( sources.is_ok() );

//         let sources = sources.unwrap();

//         assert_eq!( sources.headers().len(), 1 );
//         assert_eq!( sources.rows().len(), 4 );
//     }
// }