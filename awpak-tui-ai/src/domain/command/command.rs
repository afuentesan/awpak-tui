use serde::{Deserialize, Serialize};

use crate::domain::node::node::NodeOutputDestination;


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Command
{
    #[serde(default)]
    pub name : String,

    pub command : String,

    #[serde(default)]
    pub args : Vec<String>,

    #[serde(default)]
    pub output : NodeOutputDestination,
    #[serde(default)]
    pub output_err : NodeOutputDestination
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandResult
{
    #[serde(default)]
    pub out : Option<String>,
    #[serde(default)]
    pub err : Option<String>,
    #[serde(default)]
    pub success : bool,
    #[serde(default)]
    pub code : Option<i32>
}