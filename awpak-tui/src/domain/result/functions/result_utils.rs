use crate::domain::error::Error;


pub fn bool_err( is_err : bool, err : Error ) -> Result<(), Error>
{
    if is_err { Err( err ) } else { Ok( () ) }
}

// pub fn unit_result<T>( result : &Result<T, Error> ) -> Result<(), Error>
// {
//     match result
//     {
//         Ok( _ ) => Ok( () ),
//         Err( e ) => Err( e.clone() )    
//     }
// }