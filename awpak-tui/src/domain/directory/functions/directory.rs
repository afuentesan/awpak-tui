use std::path::Path;

use crate::domain::{directory::model::directory::Directory, error::Error, field::model::{edit_field_output::EditFieldOutput, field::Field}, path::path_utils::{change_path_name_if_not_exists, path_exists, path_from_dest_dir_and_src}};


pub fn copy_directory_to_path( directory : &Directory, path : &Path ) -> Result<(), Error>
{
    let dest = path_from_dest_dir_and_src( path, directory.path() )?;

    if path_exists( dest.as_path() ).is_ok()
    {
        return Err( Error::DestinationPathExists( format!( "Destination path exists. Destination: {:?}", dest ) ) );
    }

    copy_dir_all( directory.path(), dest.as_path(), dest.as_path() )
    .map_err( | e | Error::CopyDirectory( e.to_string() ) )?;

    Ok( () )
}

pub fn delete_directory( directory : &Directory ) -> Result<(), Error>
{
    std::fs::remove_dir_all( directory.path() ).map_err( | e | Error::DeleteDirectory( e.to_string() ) )
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>, original_dest : &Path ) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {

            if entry.path().as_path() != original_dest
            {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()), original_dest)?;
            }
            
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn save_directory_field( directory : &Directory, field : &Field ) -> Result<EditFieldOutput, Error>
{
    if field.id != "name"
    {
        return Err( Error::Ignore )
    }

    let dst_path = change_path_name_if_not_exists( directory.path(), &field.input.text )?;

    std::fs::rename( directory.path(), &dst_path ).map_err( | e | Error::RenameDirectory( e.to_string() ) )?;

    Ok( EditFieldOutput::ChangeFileName { cell_id : field.id.clone(), name : field.input.text.to_string(), path : dst_path } )
}