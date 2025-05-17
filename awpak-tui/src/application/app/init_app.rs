use crate::domain::{app::model::app::App, error::Error, table::model::table::Table};

use super::exec_focus::exec_focus;


pub fn init_app( sources : Table ) -> Result<App, Error>
{
    let app = App::new( sources );

    match exec_focus( app ).collect()
    {
        ( a, _ ) => Ok( a )
    }
}

#[cfg(test)]
mod tests
{
    use crate::domain::{app::model::app::AppContent, directory::model::directory::Directory, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, header::{Header, HeaderData}, row::Row, rowable::Rowable}};

    use super::*;

    #[test]
    fn test_init_app()
    {
        let app = init_app(
            Table::new(
                vec![ Header::Visible( HeaderData { id : "sources".to_string(), name : "Sources".to_string() } ) ], 
                vec![ 
                    SelectableItem::CurrentSelected(
                        Row::Directory( 
                            Directory::new( "/tmp", vec![ Cell::Read( CellType::String( "Tmp".to_string() ) ) ] ).unwrap() 
                        ) 
                    )
                    ]
            ).unwrap()
        );

        assert!( app.is_ok() );

        let app = app.unwrap();

        match app.content()
        {
            AppContent::Table( t ) => {
                assert!( t.headers().len() > 0 );
                assert!( t.rows().len() > 0 );

                for row in t.rows()
                {
                    assert!( t.headers().len() == row.inner().cells().len() );
                }
            },
            _ => assert!( false, "App content is not a table" )    
        }
    }
}