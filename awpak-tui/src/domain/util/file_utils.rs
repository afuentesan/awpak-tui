use std::{fs::{File, OpenOptions}, io::Write as _, path::{Path, PathBuf}};

use crate::domain::path::path_utils::{path_exists, path_for_file};


pub fn append_text_to_file( text : &str, path : &str )
{
    match path_for_file( path )
    {
        Ok( p ) => append_text_to_file_exist( text, p ),
        _ => append_text_to_file_new( text, path )
    }
}

fn append_text_to_file_exist( text : &str, path : PathBuf )
{
    match OpenOptions::new()
            .write( true )
            .append( true )
            .open( path )
    {
        Ok( mut f ) =>
        {
            let _ = write!( &mut f, "{}", text );
        },
        _ => {}
    }
}

fn append_text_to_file_new( text : &str, path : &str )
{
    if path_exists( Path::new( path ) ).is_ok() { return }

    match File::create( path )
    {
        Ok( mut f ) =>
        {
            let _ = write!( &mut f, "{}", text );
        },
        _ => {}
    }
}