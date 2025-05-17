use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn log_to_file(
    log : &str,
    file_path : &str
) -> Result<(), ()>
{
    create_file_if_not_exists( file_path )?;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open( file_path )
        .map_err( | _ | () )?;

    let _ = write!( file, "{}", log );

    Ok( () )
}

fn create_file_if_not_exists( path : &str ) -> Result<(), ()>
{
    let path = Path::new( path );

    if path.exists()
    {
        if path.is_file() { return Ok( () ) } else { return Err( () ) };
    }
    
    let _ = File::create( path ).map_err( | _ | () )?;

    Ok( () )
}