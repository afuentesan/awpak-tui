use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SourceConfig
{
    Home { name : Option<String> },
    Directory { path : String, name : String },
    Expandable { path : String, name : String },
    ExecutableExpandable { path : String, params : Option<String>, name : String },
    Title { name : String }
}
