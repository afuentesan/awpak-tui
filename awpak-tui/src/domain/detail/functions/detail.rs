use crate::domain::{app::model::app::{App, AppContent}, detail::model::detail::{Detail, DetailContent}, error::Error, field::model::{edit_field_output::EditFieldOutput, field::Field}, result::{functions::result_utils::bool_err, result::AwpakResult}, selectable::functions::selectable_utils::idx_current_selected_item, table::{functions::{table_detail::detail_content_from_table, table_save::{save_table_field, update_table_field}}, model::{rowable::Rowable, table::Table}}};

pub fn persist_detail_field( detail : &Detail, field : &Field ) -> Result<EditFieldOutput, Error>
{
    match detail.content()
    {
        DetailContent::Table( _ ) => persist_detail_table_field( detail.source(), field ),
        DetailContent::Empty => Err( Error::Ignore )    
    }
}

fn persist_detail_table_field( source : &AppContent, field : &Field ) -> Result<EditFieldOutput, Error>
{
    match source
    {
        AppContent::Table( t ) => save_table_field( t, field ),
        _ => Err( Error::Ignore )    
    }
}

pub fn update_detail_field( app : App, detail : Detail, value: &EditFieldOutput ) -> App
{
    let detail = update_detail_content( detail, value );
    let detail = update_detail_source( detail, value );

    app.change_content( AppContent::Detail( Box::new( detail ) ) )
}

pub fn update_detail_content( detail : Detail, value : &EditFieldOutput ) -> Detail
{
    let ( detail, content ) = detail.own_content();

    match content
    {
        DetailContent::Table( t ) => detail.change_content( update_detail_content_table( t, value ) ),
        DetailContent::Empty => detail.change_content( content )    
    }
}

pub fn update_detail_content_table( table : Table, value : &EditFieldOutput ) -> DetailContent
{
    let row_cell = idx_current_selected_item( table.rows() ).iter().flat_map(
        | i |
        {
            idx_current_selected_item( table.rows()[ *i ].cells() )
            .map(
                | cell_idx | ( *i, cell_idx )
            )
        }
    )
    .next();

    match row_cell
    {
        Some( ( idx_row, idx_col ) ) =>
        {
            DetailContent::Table( update_table_field( table, value, idx_row, idx_col ) )
        },
        None => DetailContent::Table( table )
    }
}

pub fn update_detail_source( detail : Detail, value : &EditFieldOutput ) -> Detail
{
    let ( detail, source ) = detail.own_source();

    match source
    {
        AppContent::Table( t ) => detail.change_source( update_detail_source_table( t, value ) ),
        _ => detail.change_source( source )
    }
}

pub fn update_detail_source_table( table : Table, value : &EditFieldOutput ) -> AppContent
{
    if value.to_option_id().is_none()
    {
        return AppContent::Table( table );
    }

    let row_cell = idx_current_selected_item( table.rows() ).iter().flat_map(
        | i |
        {
            table.headers().iter()
            .enumerate()
            .find( | h | h.1.as_id() == value.to_option_id().unwrap() )
            .map( | h | ( *i, h.0 ) )
        }
    )
    .next();

    match row_cell
    {
        Some( ( idx_row, idx_col ) ) =>
        {
            AppContent::Table( update_table_field( table, value, idx_row, idx_col ) )
        },
        None => AppContent::Table( table )
    }
}

pub fn detail_from_content( content : AppContent ) -> AwpakResult<AppContent>
{
    AwpakResult::new( content )
    .validate()
    .map_result( | c | bool_err( ! has_detail( &c ), Error::Ignore ) )
    .zip_result( | c | zip_detail( c ) )
    .write()
    .map(
        | ( c, d ) |
        {
            ( new_content( c, d.unwrap() ), Ok( DetailContent::Empty ) )
        }
    )
    .finalize()
    .unzip( | ( c, _ ) | c )
    .read()
}

pub fn detail_id_from_content( content : &AppContent ) -> Option<String>
{
    match content
    {
        AppContent::Detail( d ) => detail_id_from_source_detail( d ),
        AppContent::Table( _ ) |
        AppContent::Chat( _ ) |
        AppContent::Graph( _ ) |
        AppContent::Empty => None    
    }
}

fn detail_id_from_source_detail( detail : &Detail ) -> Option<String>
{
    match detail.source()
    {
        AppContent::Table( t ) => idx_current_selected_item( t.rows() ).map( | i | i.to_string() ),
        AppContent::Detail( _ ) => None,
        AppContent::Chat( _ ) => None,
        AppContent::Graph( _ ) |
        AppContent::Empty => None    
    }
}

fn new_content( content : AppContent, detail : DetailContent ) -> AppContent
{
    AppContent::Detail( Box::new( Detail::new( content, detail ) ) )
}

fn has_detail( content : &AppContent ) -> bool
{
    match content
    {
        AppContent::Table( _ ) => true,
        AppContent::Detail( _ ) => false,
        AppContent::Chat( _ ) => false,
        AppContent::Graph( _ ) |
        AppContent::Empty => false    
    }
}

fn zip_detail( content : &AppContent ) -> Result<DetailContent, Error>
{
    match content
    {
        AppContent::Table( t ) => detail_content_from_table( t ),
        AppContent::Detail( _ ) => Err( Error::Ignore ),
        AppContent::Empty => Err( Error::Ignore ),
        AppContent::Chat( _ ) => Err( Error::Ignore ),
        AppContent::Graph( _ ) => Err( Error::Ignore )
    }
}