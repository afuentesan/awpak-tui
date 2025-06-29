use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct NodeMCPServer
{
    pub command : String,
    #[serde(default)]
    pub arguments : Vec<String>,
    #[serde(default)]
    pub env : HashMap<String, String>
}
