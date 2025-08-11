use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::data::data::DataFrom;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMCPServer
{
    pub command : String,
    #[serde(default)]
    pub arguments : Vec<DataFrom>,
    #[serde(default)]
    pub env : HashMap<String, String>
}
