use crate::domain::{content_generator::model::content_generator::ContentGenerator, directory::functions::expand_dir::dir_expand, error::Error, executable::functions::executable::execute_command, executable_expandable::functions::executable_expandable::expand_excutable_expandable, expandable::functions::expandable::expand_expandable, field::model::field::Field, file::functions::file::open_file, message::model::message::Message, table::{functions::table_selection::current_selected_row, model::{cell::Cell, row::Row, row_output::RowOutput, rowable::Rowable, table::Table}}};

use super::table_selection::current_selected_row_and_cell;

pub fn exec_current_selected_row( table : &Table ) -> Result<RowOutput, Error>
{
    match current_selected_row_and_cell( table )
    {
        Some( ( idx_row, idx_cell ) ) => cell_execution( table, idx_row, idx_cell ),
        None => row_execution( table )
    }
}

fn cell_execution( table : &Table, idx_row : usize, idx_cell : usize ) -> Result<RowOutput, Error>
{
    match table.rows()[ idx_row ].cells()[ idx_cell ].inner()
    {
        Cell::Read( _ ) => cell_read_execution( table, idx_row, idx_cell ),
        Cell::Write( _ ) => cell_write_execution( table, idx_row, idx_cell )
    }
}

fn cell_read_execution( table : &Table, idx_row : usize, idx_cell : usize ) -> Result<RowOutput, Error>
{
    let text = format!( "{}:\n{}", table.headers()[ idx_cell ].to_string(), table.rows()[ idx_row ].cells()[ idx_cell ].inner().to_string() );

    Ok(
        RowOutput::ShowMessage(
            Message::Info( text )
        )
    )
}

fn cell_write_execution( table : &Table, idx_row : usize, idx_cell : usize ) -> Result<RowOutput, Error>
{
    let ( id, name ) = cell_id_and_name( table, idx_row, idx_cell );

    Ok(
        RowOutput::EditField( 
            Field::from( 
                ( table.rows()[ idx_row ].cells()[ idx_cell ].inner(), id, name ) 
            ) 
        )
    )
}

fn cell_id_and_name( table : &Table, idx_row : usize, idx_cell : usize ) -> ( String, String )
{
    match table.rows()[ idx_row ].inner()
    {
        Row::DataRef { id, .. } => ( id.clone(), table.headers()[ idx_cell ].to_string() ),
        _ => ( table.headers()[ idx_cell ].as_id().to_string(), table.headers()[ idx_cell ].to_string() )
    }
}

fn row_execution( table : &Table ) -> Result<RowOutput, Error>
{
    Ok(
        match current_selected_row( table ).ok_or( Error::Ignore )?
        {
            Row::Directory( d ) => RowOutput::NewContent 
            { 
                generator : ContentGenerator::Directory( d.clone() ), 
                content : dir_expand( d )?
            },
            Row::File( f ) =>
            {
                open_file( f );

                RowOutput::Ignore
            },
            Row::Executable( e ) =>
            {
                let _ = execute_command( e.to_string().as_str(), e.params() );

                RowOutput::Ignore
            },
            Row::Expandable( e ) => RowOutput::NewContent 
            { 
                generator : ContentGenerator::Expandable( e.clone() ), 
                content : expand_expandable( e )?
            },
            Row::ExecutableExpandable( e ) => RowOutput::NewContent 
            {
                generator : ContentGenerator::ExecutableExpandable( e.clone() ), 
                content : expand_excutable_expandable( e )?
            },
            Row::Data( _ ) => return Err( Error::Ignore ),
            Row::DataRef { id : _, cells : _ } => return Err( Error::Ignore )
        }
    )
}