use serde::{Deserialize, Serialize};

use crate::domain::{agent::agent::AIAgent, data::data::InputData};


#[derive(Serialize, Deserialize, Clone)]
pub struct Repeat
{
    #[serde(default)]
    pub name : String,

    pub agent : Box<AIAgent>,

    pub input : Vec<InputData>
}

impl Default for Repeat
{
    fn default() -> Self 
    {
        Self { name : Default::default(), agent : Default::default(), input : Default::default() }
    }
}