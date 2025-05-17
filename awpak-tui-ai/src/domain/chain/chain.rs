use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::{agent::agent::AIAgent, data::data::{InputData, OutputData}};

#[derive(Serialize, Deserialize, Clone)]
pub struct Chain
{
    #[serde(default)]
    pub name : String,

    pub items : Vec<ChainItem>,

    #[serde(default)]
    pub initial_context : Option<Value>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChainItem
{
    pub agent : AIAgent,

    #[serde(default)]
    pub input : Vec<InputData>,

    #[serde(default)]
    pub input_separator : Option<String>,

    #[serde(default)]
    pub output : OutputData
}

impl Default for Chain
{
    fn default() -> Self 
    {
        Self 
        { 
            name : "".into(), 
            items : vec![],
            initial_context : None
        }
    }
}