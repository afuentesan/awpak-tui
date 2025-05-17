use std::path::PathBuf;

use crate::domain::{app::model::app::{App, AppContent}, directory::functions::directory::save_directory_field, error::Error, field::model::{edit_field_output::EditFieldOutput, field::Field}, file::functions::file::save_file_field, selectable::model::selectable_item::SelectableItem, table::{functions::table_selection::current_selected_row_and_cell, model::{cell::Cell, row::Row, rowable::Rowable, table::Table}}};

use super::table_selection::current_selected_row;


pub fn save_table_field( table : &Table, field : &Field ) -> Result<EditFieldOutput, Error>
{
    match current_selected_row( table ).ok_or( Error::Ignore )?
    {
        Row::Directory( d ) => save_directory_field( d, field ),
        Row::File( f ) => save_file_field( f, field ),
        Row::Executable( _ ) |
        Row::Expandable( _ ) |
        Row::ExecutableExpandable( _ ) |
        Row::Data( _ ) |
        Row::DataRef { id : _, cells : _ } => Err( Error::Ignore )
    }
}

pub fn update_content_table_selected_field( app : App, table : Table, value : &EditFieldOutput ) -> App
{
    if let Some( ( r, c ) ) = current_selected_row_and_cell( &table )
    {
        app.change_content( 
            AppContent::Table( 
                update_table_field( table, value, r, c )
            ) 
        )
    }
    else
    {
        app.change_content( AppContent::Table( table ) )
    }
}

pub fn update_table_field( table : Table, value : &EditFieldOutput, idx_row : usize, idx_col : usize ) -> Table
{
    let ( table, row ) = table.own_row( idx_row );

    let row = row.unwrap();

    let ( row, mut cells ) = row.own_cells();

    let old = std::mem::replace( &mut cells[ idx_col ], SelectableItem::<Cell>::default() );

    match value.to_option_name()
    {
        Some( s ) => cells[ idx_col ] = old.change_value( s.as_str() ).own(),
        None => cells[ idx_col ] = old
    };

    let row = row.change_cells( cells );

    let row = change_row_path( *row, value.to_option_path() );

    table.change_row( idx_row, row )
}

fn change_row_path( row : SelectableItem<Row>, path : Option<PathBuf> ) -> SelectableItem<Row>
{
    if path.is_none()
    {
        return row;
    }

    let ( row, inner ) = row.own_inner();

    match inner
    {
        Row::Directory( d ) => row.change_inner( Row::Directory( d.change_path( path.unwrap() ) ) ),
        Row::File( f ) => row.change_inner( Row::File( f.change_path( path.unwrap() ) ) ),
        _ => row.change_inner( inner )
    }
}