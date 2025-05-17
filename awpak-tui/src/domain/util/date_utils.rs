use chrono::NaiveDate;

use crate::domain::error::Error;


pub fn naive_date_from_str( str_date : &str ) -> Result<NaiveDate, Error>
{
    NaiveDate::parse_from_str( str_date, "%d/%m/%Y" ).map_err( | e | Error::InvalidStrDate( e.to_string() ) )
}