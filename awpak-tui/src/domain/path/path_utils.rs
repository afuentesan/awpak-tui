use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use is_executable::IsExecutable as _;

use crate::domain::{error::Error, table::model::cell::{Cell, CellType}};

pub fn change_path_name_if_not_exists( path : &Path, new_name : &str ) -> Result<PathBuf, Error>
{
    let mut path = path.to_path_buf();

    let has_parent = path.pop();

    if ! has_parent
    {
        return Err( Error::NoParent );
    }

    path.push( new_name );

    if path_exists( path.as_path() ).is_ok()
    {
        return Err( Error::DestinationPathExists( format!( "Destination path exists. Destination: {:?}", path ) ) );
    }

    Ok( path )
}

pub fn path_from_dest_dir_and_src( dest_dir : &Path, src : &Path ) -> Result<PathBuf, Error>
{
    path_exists( src )?;

    path_exists( dest_dir )?;
    path_is_dir( dest_dir )?;

    Ok(
        dest_dir.join( 
            src.file_name().ok_or( 
                Error::InvalidPath( format!( "Filename not found. Path: {:?}", src ) )
            )? 
        )
    )
}

pub fn get_row_from_path( path : &std::path::Path ) -> Result<Vec<Cell>, Error>
{
    std::fs::metadata( path )
        .iter().flat_map( | m | {

            let modified = m.modified()
            .map( | t | 
                {
                    CellType::Date( DateTime::<Utc>::from( t ).naive_utc().date() )
                }
            ).unwrap_or( CellType::Empty );

            Ok::<Vec<Cell>, Error>(
                vec![
                    Cell::Write( CellType::String( path_name( path )? ) ),
                    Cell::Read( modified ),
                    Cell::Read( CellType::String( m.len().to_string() ) )
                ]
            )
        }
    )
    .next()
    .ok_or( Error::Ignore )
}

pub fn path_for_home() -> Result<String, Error>
{
    home::home_dir()
    .iter().flat_map( | h | h.to_str() )
    .filter( | h | ! h.is_empty() )
    .map( | h | h.to_string() )
    .next()
    .ok_or( Error::NotHomeDir )
}

pub fn path_for_exec( str_path : &str ) -> Result<std::path::PathBuf, Error>
{
    path_for_exec_from_path( std::path::Path::new( str_path ) )
}

pub fn path_for_exec_from_path( path : &std::path::Path ) -> Result<std::path::PathBuf, Error>
{
    path_exists( path )?;
    path_is_file( path )?;
    path_is_exec( path )?;

    Ok( path.into() )
}

pub fn path_for_dir( str_path : &str ) -> Result<std::path::PathBuf, Error>
{
    path_for_dir_from_path( std::path::Path::new( str_path ) )
}

pub fn path_for_dir_from_path( path : &std::path::Path ) -> Result<std::path::PathBuf, Error>
{
    path_exists( path )?;
    path_is_dir( path )?;

    Ok( path.into() )
}

pub fn path_parent_of_dir( path : &std::path::Path ) -> Result<std::path::PathBuf, Error>
{
    path_exists( path )?;
    path_is_dir( path )?;

    let parent = path.parent().ok_or( Error::NoParent )?;

    path_exists( parent )?;
    path_is_dir( parent )?;

    Ok( parent.into() )
}

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

pub fn string_from_path( path : &std::path::Path ) -> Result<String, Error>
{
    std::fs::read_to_string( path_for_file_from_path( path )? )
    .map_err( | e | Error::FailReadFile( e.to_string() ) )
}

pub fn extension_from_path( path : &std::path::Path ) -> Result<String, Error>
{
    Ok( 
        path.extension().ok_or( Error::PathWithoutExtension( path.to_string_lossy().to_string() ) )?
        .to_string_lossy().to_string() 
    )
}

pub fn path_exists( path : &std::path::Path ) -> Result<(), Error>
{
    match path.exists()
    {
        true => Ok( () ),
        false => Err( Error::InvalidPath( path.to_str().unwrap_or( "" ).to_string() ) )    
    }
}

fn path_is_dir( path : &std::path::Path ) -> Result<(), Error>
{
    match path.is_dir()
    {
        true => Ok( () ),
        false => Err( Error::NotDir( path.to_str().unwrap_or( "" ).to_string() ) )    
    }
}

fn path_is_file( path : &std::path::Path ) -> Result<(), Error>
{
    match path.is_file()
    {
        true => Ok( () ),
        false => Err( Error::NotFile( path.to_str().unwrap_or( "" ).to_string() ) )    
    }
}

fn path_is_exec( path : &std::path::Path ) -> Result<(), Error>
{
    match path.is_executable()
    {
        true => Ok( () ),
        false => Err( Error::NotExec( path.to_str().unwrap_or( "" ).to_string() ) )  
    }
}

fn full_path( path : &std::path::Path ) -> Result<String, Error>
{
    Ok(
        path
        .canonicalize()
        .map_err( | e | Error::InvalidPath( format!( "Invalid path: {:?}. {}", path, e.to_string() ) ) )?
        .to_str()
        .ok_or( Error::InvalidPath( format!( "Invalid path: {:?}.", path ) ) )?
        .to_string()
    )
}

fn path_name( path : &std::path::Path ) -> Result<String, Error>
{
    match path.file_name()
    .iter().flat_map( | f | f.to_str() )
    .next()
    {
        Some( s ) => Ok( s.to_string() ),
        None => full_path( path )
    }
}

#[cfg(test)]
mod tests
{
    use std::{fs::File, path::Path};

    use super::*;

    #[test]
    fn test_change_path_name_if_not_exists_path_exists()
    {
        let _ = File::create( "/tmp/hola.txt" );
        let _ = File::create( "/tmp/hola2.txt" );

        let result = change_path_name_if_not_exists( Path::new( "/tmp/hola.txt" ), "hola2.txt" );

        assert!( result.is_err() );
    }

    #[test]
    fn test_change_path_name_if_not_exists_no_parent()
    {
        let result = change_path_name_if_not_exists( Path::new( "/" ), "hola2.txt" );

        assert!( result.is_err() );
    }

    #[test]
    fn test_change_path_name_if_not_exists()
    {
        let _ = File::create( "/tmp/hola.txt" );

        let result = change_path_name_if_not_exists( Path::new( "/tmp/hola.txt" ), "hola_nuevo_no_existe.txt" );

        assert!( result.is_ok() );

        assert_eq!( result.unwrap().to_string_lossy(), "/tmp/hola_nuevo_no_existe.txt" );
    }

    #[test]
    fn test_path_parent_of_dir_path_is_file()
    {
        let _ = File::create( "/tmp/hola.txt" );
        let path = Path::new( "/tmp/hola.txt" );

        assert!( path_parent_of_dir( path ).is_err() );
    }

    #[test]
    fn test_path_parent_of_dir_path_not_exists()
    {
        let path = Path::new( "/tmp/path_inventado" );

        assert!( path_parent_of_dir( path ).is_err() );
    }

    #[test]
    fn test_path_parent_of_dir_path_is_root()
    {
        let path = Path::new( "/" );

        assert!( path_parent_of_dir( path ).is_err() );
    }

    #[test]
    fn test_path_parent_of_dir()
    {
        let path = Path::new( "/tmp" );

        let parent = path_parent_of_dir( path );

        assert!( parent.is_ok() );

        assert_eq!( parent.unwrap().to_string_lossy(), "/" );
    }

    #[test]
    fn test_path_for_home()
    {
        let path = path_for_home();

        assert!( path.is_ok() );

        let path = path.unwrap();

        assert_eq!( path, "/home/angel" );
    }

    #[test]
    fn test_get_row_from_path()
    {
        let _ = File::create( "/tmp/hola.txt" );
        let path = Path::new( "/tmp/hola.txt" );

        let row = get_row_from_path( path );

        assert!( row.is_ok() );

        let row = row.unwrap();

        assert!( row.len() == 3 );

        assert_eq!( row[ 0 ].to_string(), "hola.txt" );
        assert_eq!( row[ 2 ].to_string(), "0" );
    }

    #[test]
    fn test_get_row_from_invalid_path()
    {
        let path = Path::new( "/tmp/inventado_sdef.txt" );

        let row = get_row_from_path( path );

        assert!( row.is_err() );
    }

    #[test]
    fn test_path_for_file_err()
    {
        assert!( path_for_file( "/asdf" ).is_err() );
        assert!( path_for_file( "/tmp" ).is_err() );
    }

    #[test]
    fn test_path_for_file()
    {
        assert!( path_for_file( "Cargo.toml" ).is_ok() );
    }

    #[test]
    fn test_path_for_dir_err()
    {
        assert!( path_for_dir( "/asdf" ).is_err() );
        assert!( path_for_dir( "Cargo.toml" ).is_err() );
    }

    #[test]
    fn test_path_for_dir()
    {
        let tmp = path_for_dir( "/tmp" );

        assert!( tmp.is_ok() );

        assert_eq!( tmp.unwrap().to_str().unwrap(), "/tmp" );

        assert!( path_for_dir( "/" ).is_ok() );
    }

    #[test]
    fn test_full_path_not_exists()
    {
        assert!( full_path( std::path::Path::new( "/asdf" ) ).is_err() );
    }

    #[test]
    fn test_full_path()
    {
        let _ = File::create( "/tmp/hola" );
        let _ = File::create( "/tmp/hola.txt" );

        assert!( full_path( std::path::Path::new( "/tmp" ) ).is_ok() );
        assert_eq!( full_path( std::path::Path::new( "/tmp" ) ).unwrap(), "/tmp" );

        assert!( full_path( std::path::Path::new( "/" ) ).is_ok() );
        assert_eq!( full_path( std::path::Path::new( "/" ) ).unwrap(), "/" );

        assert!( full_path( std::path::Path::new( "/tmp/hola" ) ).is_ok() );
        assert_eq!( full_path( std::path::Path::new( "/tmp/hola" ) ).unwrap(), "/tmp/hola" );

        assert!( full_path( std::path::Path::new( "/tmp/hola.txt" ) ).is_ok() );
        assert_eq!( full_path( std::path::Path::new( "/tmp/hola.txt" ) ).unwrap(), "/tmp/hola.txt" );
    }

    #[test]
    fn test_path_not_exists()
    {
        assert!( path_exists( std::path::Path::new( "/asdf" ) ).is_err() );
    }

    #[test]
    fn test_path_exists()
    {
        assert!( path_exists( std::path::Path::new( "/tmp" ) ).is_ok() );
    }

    #[test]
    fn test_path_is_not_dir()
    {
        assert!( path_is_dir( std::path::Path::new( "Cargo.toml" ) ).is_err() );
        assert!( path_is_dir( std::path::Path::new( "/asdf" ) ).is_err() );
    }

    #[test]
    fn test_path_is_dir()
    {
        assert!( path_is_dir( std::path::Path::new( "/tmp" ) ).is_ok() );
    }

    #[test]
    fn test_path_is_not_file()
    {
        assert!( path_is_file( std::path::Path::new( "/tmp" ) ).is_err() );
        assert!( path_is_file( std::path::Path::new( "/asdf" ) ).is_err() );
    }

    #[test]
    fn test_path_is_file()
    {
        assert!( path_is_file( std::path::Path::new( "Cargo.toml" ) ).is_ok() );
    }

    #[test]
    fn test_path_name()
    {
        let _ = File::create( "/tmp/hola" );
        let _ = File::create( "/tmp/hola.txt" );

        assert!( path_name( std::path::Path::new( "/tmp/hola" ) ).is_ok() );
        assert_eq!( path_name( std::path::Path::new( "/tmp/hola" ) ).unwrap(), "hola" );

        assert!( path_name( std::path::Path::new( "/tmp/hola.txt" ) ).is_ok() );
        assert_eq!( path_name( std::path::Path::new( "/tmp/hola.txt" ) ).unwrap(), "hola.txt" );

        assert!( path_name( std::path::Path::new( "/inventado" ) ).is_ok() );
        assert_eq!( path_name( std::path::Path::new( "/inventado" ) ).unwrap(), "inventado" );

        assert!( path_name( std::path::Path::new( "/" ) ).is_ok() );
        assert_eq!( path_name( std::path::Path::new( "/" ) ).unwrap(), "/" );
    }
}