use std::{fs::File, io::BufReader};

use crate::{domain::path::path_utils::path_for_file, infrastructure::config::model::ai_config::AIConfig};

const CONFIG_AI_VAR : &'static str = "AWPAK_TUI_AI";

pub fn ai_config() -> Option<AIConfig>
{
    match std::env::var( CONFIG_AI_VAR )
    {
        Ok( v ) => ai_config_from_path( v ),
        _ => None
    }
}

fn ai_config_from_path( path : String ) -> Option<AIConfig>
{
    let path = path_for_file( path.as_str() ).ok()?;

    let file = File::open( path ).map_err( | _ | () ).ok()?;
    let reader = BufReader::new( file );

    serde_json::from_reader(reader ).ok()
}