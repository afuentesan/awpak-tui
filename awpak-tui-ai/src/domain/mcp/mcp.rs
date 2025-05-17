use std::{collections::HashMap, sync::Arc};

use rmcp::{model::Tool, service::RunningService, RoleClient};
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

#[derive(Clone)]
pub struct MCPClient
{
    pub service : Arc<RunningService<RoleClient, ()>>,
    pub tools : Vec<Tool>
}
