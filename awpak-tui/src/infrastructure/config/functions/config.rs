use std::{fs::File, io::BufReader, sync::OnceLock};

use crate::{domain::{directory::model::directory::Directory, executable_expandable::model::executable_expandable::ExecutableExpandable, expandable::model::expandable::Expandable, path::path_utils::{path_for_file, path_for_home}, selectable::model::selectable_item::SelectableItem, table::model::{cell::{Cell, CellType}, row::Row}}, infrastructure::config::model::source_config::SourceConfig};


const CONFIG_SOURCES_VAR : &'static str = "AWPAK_TUI_SOURCES";

pub fn sources_config() -> &'static Vec<SourceConfig> {
    static ARRAY_SOURCES: OnceLock<Vec<SourceConfig>> = OnceLock::new();
    ARRAY_SOURCES.get_or_init(|| init_sources_config() )
}

pub fn rows_from_sources_config( config : &Vec<SourceConfig> ) -> Vec<Row>
{
    config.iter()
    .flat_map( | c | row_from_source_config( c ) )
    .collect()
}

fn row_from_source_config( config : &SourceConfig ) -> Option<Row>
{
    match config
    {
        SourceConfig::Home { name } =>
        {
            let name = name.clone().unwrap_or( "Home".to_string() );

            let path = path_for_home().ok()?;

            Some( Row::Directory( Directory::new( path.as_str(), vec![ Cell::Read( CellType::String( name ) ) ] ).ok()? ) )
        },
        SourceConfig::Directory { path, name } =>
        {
            Some( Row::Directory( Directory::new( path.as_str(), vec![ Cell::Read( CellType::String( name.clone() ) ) ] ).ok()? ) )
        },
        SourceConfig::Expandable { path, name } =>
        {
            Some( Row::Expandable( Expandable::new( path.as_str(), vec![ Cell::Read( CellType::String( name.clone() ) ) ] ).ok()? ) )
        },
        SourceConfig::ExecutableExpandable { path, params, name } =>
        {
            Some( Row::ExecutableExpandable( ExecutableExpandable::new( path.as_str(), params.clone(), vec![ Cell::Read( CellType::String( name.clone() ) ) ] ).ok()? ) )
        },
        SourceConfig::Title { name } =>
        {
            Some( Row::Data( vec![ SelectableItem::Idle( Cell::Read( CellType::String( name.clone() ) ) ) ] ) )
        }
    }
}

fn init_sources_config() -> Vec<SourceConfig>
{
    match std::env::var( CONFIG_SOURCES_VAR )
    {
        Ok( v ) if v.trim() != "" => init_sources_from_path( v ).unwrap_or( default_sources() ),
        _ => default_sources()
    }
}

fn default_sources() -> Vec<SourceConfig>
{
    vec![ SourceConfig::Home { name : Some( "Home".to_string() ) } ]
}

fn init_sources_from_path( path : String ) -> Result<Vec<SourceConfig>, ()>
{
    let path = path_for_file( path.as_str() ).map_err( | _ | () )?;

    let file = File::open( path ).map_err( | _ | () )?;
    let reader = BufReader::new( file );

    serde_json::from_reader(reader).map_err( | _ | () )
}