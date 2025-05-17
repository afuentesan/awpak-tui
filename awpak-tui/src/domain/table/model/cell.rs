use chrono::NaiveDate;

use crate::domain::{error::Error, result::result::AwpakResult, util::date_utils::naive_date_from_str};



#[derive(Clone)]
pub enum Cell
{
    Read( CellType ),
    Write( CellType ),
}

impl Cell
{
    pub fn change_value( self, value : &str ) -> AwpakResult<Cell>
    {
        match self
        {
            Cell::Read( t ) => AwpakResult::new_err( Cell::Read( t ), Error::Ignore ),
            Cell::Write( t ) => t.change_value( value ).finalize().unzip( | t | Cell::Write( t ) ).read()
        }
    }
}

impl Default for Cell
{
    fn default() -> Self
    {
        Self::Read( CellType::default() )
    }
}

// impl Cell
// {
//     pub fn to_read( self ) -> Self
//     {
//         match self
//         {
//             Cell::Read( t ) |
//             Cell::Write( t ) => Cell::Read( t )    
//         }
//     }
// }

#[derive(Clone)]
pub enum CellType
{
    Date( chrono::NaiveDate ),
    String( String ),
    Empty
}

impl CellType
{
    pub fn change_value( self, value : &str ) -> AwpakResult<Self>
    {
        match self
        {
            CellType::Date( d ) => CellType::date_from_str( d, value ),
            CellType::String( _ ) => AwpakResult::new( CellType::String( value.to_string() ) ),
            CellType::Empty => AwpakResult::new_err( CellType::Empty, Error::Ignore )
        }
    }

    fn date_from_str( default : NaiveDate, value : &str ) -> AwpakResult<Self>
    {
        match naive_date_from_str( value )
        {
            Ok( d ) => AwpakResult::new( CellType::Date( d ) ),
            Err( e ) => AwpakResult::new_err( CellType::Date( default ), e )    
        }
    }
}

impl Default for CellType
{
    fn default() -> Self 
    {
        Self::Empty
    }
}

impl ToString for CellType
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            CellType::Date( d ) => d.format( "%d/%m/%Y" ).to_string(),
            CellType::String( s ) => s.clone(),
            CellType::Empty => "".to_string()   
        }
    }
}

impl ToString for Cell
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            Cell::Read( c ) | 
            Cell::Write( c ) => c.to_string()
        }
    }
}

#[cfg(test)]
mod tests
{
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_cell_string_to_string()
    {
        let cell = Cell::Read( CellType::String( "hola".to_string() ) );

        assert_eq!( cell.to_string(), "hola".to_string() );
    }

    #[test]
    fn test_cell_empty_to_string()
    {
        let cell = Cell::Read( CellType::Empty );

        assert_eq!( cell.to_string(), "".to_string() );
    }

    #[test]
    fn test_cell_date_to_string()
    {
        let cell = Cell::Read( CellType::Date( NaiveDate::from_ymd_opt( 2025, 4, 11 ).unwrap() ) );

        assert_eq!( cell.to_string(), "11/04/2025".to_string() );
    }
}