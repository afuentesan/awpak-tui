use crate::domain::error::Error;

pub fn option_string_to_str( string : Option<&String> ) -> &str
{
    match string
    {
        Some( s ) => s.as_str(),
        _ => ""
    }
}

pub fn bytes_to_str( output : &Vec<u8> ) -> Result<String, Error>
{
    match core::str::from_utf8( output )
    {
        Ok( s ) => Ok( s.trim().to_string() ),
        Err( e ) => Err( Error::ParseData( e.to_string() ) )
    }
}

pub fn prefix_str_suffix( prefix : Option<&String>, suffix : Option<&String>, str : &str ) -> String
{
    format!( "{}{}{}", prefix.unwrap_or( &"".to_string() ), str, suffix.unwrap_or( &"".to_string() ) )
}

pub fn str_from_option<T: ToString>( input : Option<T> ) -> String
{
    match input
    {
        Some( s ) => s.to_string(),
        None => "".into()   
    }
}