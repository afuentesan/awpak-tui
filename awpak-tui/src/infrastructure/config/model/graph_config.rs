use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AwpakTUIGraphConfig
{
    #[serde(default)]
    pub id : String,

    pub name : String,
    pub path : String,

    #[serde(default)]
    pub output : AwpakTUIGraphOutputConfig
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AwpakTUIGraphOutputConfig
{
    #[serde(default)]
    pub graph_input : Vec<AwpakTUIGraphOutputDestinationConfig>,
    #[serde(default)]
    pub graph_output_ok : Vec<AwpakTUIGraphOutputDestinationConfig>,
    #[serde(default)]
    pub graph_output_err : Vec<AwpakTUIGraphOutputDestinationConfig>,

    #[serde(default)]
    pub agent_stream : Vec<AwpakTUIGraphOutputDestinationConfig>,
    #[serde(default)]
    pub agent_sync : Vec<AwpakTUIGraphOutputDestinationConfig>,
    #[serde(default)]
    pub agent_tool_call : Vec<AwpakTUIGraphOutputDestinationConfig>,
    #[serde(default)]
    pub agent_tool_result : Vec<AwpakTUIGraphOutputDestinationConfig>,

    #[serde(default)]
    pub command_and_args : Vec<AwpakTUIGraphOutputDestinationConfig>,
    #[serde(default)]
    pub command_result : Vec<AwpakTUIGraphOutputDestinationConfig>,

    #[serde(default)]
    pub node_destination : Vec<AwpakTUIGraphOutputDestinationConfig>,

    #[serde(default)]
    pub node_execution : Vec<AwpakTUIGraphOutputDestinationConfig>
}

impl Default for AwpakTUIGraphOutputConfig
{
    fn default() -> Self 
    {
        Self 
        { 
            graph_input : vec![ AwpakTUIGraphOutputDestinationConfig::Console ],
            graph_output_ok : vec![ AwpakTUIGraphOutputDestinationConfig::Console ],
            graph_output_err : vec![ AwpakTUIGraphOutputDestinationConfig::Console ],
            agent_stream : vec![], 
            agent_sync : vec![], 
            agent_tool_call : vec![], 
            agent_tool_result : vec![], 
            command_and_args : vec![],
            command_result : vec![], 
            node_destination : vec![],
            node_execution : vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AwpakTUIGraphOutputDestinationConfig
{
    Console,
    File( String )
}