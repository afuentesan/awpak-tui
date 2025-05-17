use std::path::{Path, PathBuf};

use crate::domain::{content_generator::model::content_generator::ContentGenerator, directory::functions::directory::{copy_directory_to_path, delete_directory}, error::Error, file::functions::file::{copy_file_to_path, delete_file}, movible::model::movible::Movible, selectable::model::selectable_item::SelectableItem, table::model::{row::Row, table::Table}};


pub fn selected_movibles_from_table( table : &Table ) -> Result<Vec<Movible>, Error>
{
    let movibles = table.rows().iter()
    .flat_map( | r | selected_movible_from_selectable( r ) )
    .collect::<Vec<_>>();

    if movibles.len() == 0 { Err( Error::Ignore ) } else { Ok( movibles ) }
}

fn selected_movible_from_selectable( selectable : &SelectableItem<Row> ) -> Option<Movible>
{
    match selectable
    {
        SelectableItem::CurrentSelected( r ) | SelectableItem::Selected( r ) => selected_movible_from_row( r ),
        _ => None    
    }
}

fn selected_movible_from_row( row : &Row ) -> Option<Movible>
{
    match row
    {
        Row::Directory( d ) => Some( Movible::Directory( d.clone() ) ),
        Row::File( f ) => Some( Movible::File( f.clone() ) ),
        _ => None
    }
}

pub fn paste_and_delete_movibles( movibles : &Vec<Movible>, destination : &ContentGenerator ) -> Result<(), Error>
{
    let _ = paste_movibles( movibles, destination )?;

    delete_movibles( movibles )
}

pub fn delete_movibles( movibles : &Vec<Movible> ) -> Result<(), Error>
{
    for m in movibles { delete_movible( m )? }

    Ok( () )
}

fn delete_movible( movible : &Movible ) -> Result<(), Error>
{
    match movible
    {
        Movible::Directory( d ) => delete_directory( d ),
        Movible::File( f ) => delete_file( f )    
    }
}

pub fn paste_movibles( movibles : &Vec<Movible>, destination : &ContentGenerator ) -> Result<(), Error>
{
    let destination = destination_path( destination )?;

    copy_movibles_to_path( movibles, destination )
}

fn copy_movibles_to_path( movibles : &Vec<Movible>, destination : PathBuf ) -> Result<(), Error>
{
    for m in movibles { copy_movible_to_path( m, destination.as_path() )? }

    Ok( () )
}

fn copy_movible_to_path( movible : &Movible, destination : &Path ) -> Result<(), Error>
{
    match movible
    {
        Movible::File( f ) => copy_file_to_path( f, destination ),
        Movible::Directory( d ) => copy_directory_to_path( d, destination ),
    }
}

fn destination_path( dest : &ContentGenerator ) -> Result<PathBuf, Error>
{
    match dest
    {
        ContentGenerator::Directory( d ) => Ok( PathBuf::from( d.path() ) ),
        _ => Err( Error::Ignore )
    }
}