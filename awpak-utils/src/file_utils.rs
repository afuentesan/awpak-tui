use std::{io::Error};

pub fn path_for_file( str_path : &str ) -> Result<std::path::PathBuf, Error>
{
    let path = std::path::Path::new( str_path );

    path_for_file_from_path( path )
}

pub fn path_for_file_from_path( path : &std::path::Path ) -> Result<std::path::PathBuf, Error>
{
    path_exists( path )?;
    path_is_file( path )?;

    Ok( path.into() )
}

pub fn path_exists( path : &std::path::Path ) -> Result<(), Error>
{
    match path.exists()
    {
        true => Ok( () ),
        false => Err( std::io::Error::new( std::io::ErrorKind::NotFound, format!( "{:?} not exists.", path ) ) )
    }
}

pub fn path_is_dir( path : &std::path::Path ) -> Result<(), Error>
{
    match path.is_dir()
    {
        true => Ok( () ),
        false => Err( std::io::Error::new( std::io::ErrorKind::NotADirectory, format!( "{:?} is not a directory", path ) ) )
    }
}

pub fn path_is_file( path : &std::path::Path ) -> Result<(), Error>
{
    match path.is_file()
    {
        true => Ok( () ),
        false => Err( std::io::Error::new( std::io::ErrorKind::InvalidFilename, format!( "{:?} is not a file", path ) ) )
    }
}

// fn path_is_exec( path : &std::path::Path ) -> Result<(), impl Error>
// {
//     match path.is_executable()
//     {
//         true => Ok( () ),
//         false => Err( Error::NotExec( path.to_str().unwrap_or( "" ).to_string() ) )  
//     }
// }