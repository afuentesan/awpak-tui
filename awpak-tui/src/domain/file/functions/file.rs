use std::path::Path;

use crate::domain::{error::Error, field::model::{edit_field_output::EditFieldOutput, field::Field}, file::model::file::File, path::path_utils::{change_path_name_if_not_exists, path_exists, path_from_dest_dir_and_src}};


pub fn open_file( file : &File )
{
    open::that_in_background( file.path() );
}

pub fn copy_file_to_path( file : &File, path : &Path ) -> Result<(), Error>
{
    let dest = path_from_dest_dir_and_src( path, file.path() )?;

    if path_exists( dest.as_path() ).is_ok()
    {
        return Err( Error::DestinationPathExists( format!( "Destination path exists. Destination: {:?}", dest ) ) );
    }

    std::fs::copy( file.path(), dest ).map_err( | e | Error::CopyFile( e.to_string() ) )?;

    Ok( () )
}

pub fn delete_file( file : &File ) -> Result<(), Error>
{
    std::fs::remove_file( file.path() ).map_err( | e | Error::DeleteFile( e.to_string() ) )
}

pub fn copy_file_to_clipboard( _file : &File ) -> Result<(), Error>
{
    // TODO: No me funciona ni arboard ni doe
    Ok( () )
}

pub fn save_file_field( file : &File, field : &Field ) -> Result<EditFieldOutput, Error>
{
    if field.id != "name"
    {
        return Err( Error::Ignore )
    }

    let dst_path = change_path_name_if_not_exists( file.path(), &field.input.text )?;

    std::fs::rename( file.path(), &dst_path ).map_err( | e | Error::RenameDirectory( e.to_string() ) )?;

    Ok( EditFieldOutput::ChangeFileName { cell_id : field.id.clone(), name : field.input.text.to_string(), path : dst_path } )
}