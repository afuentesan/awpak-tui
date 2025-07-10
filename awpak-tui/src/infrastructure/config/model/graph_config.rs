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
            agent_stream : vec![ AwpakTUIGraphOutputDestinationConfig::Console ], 
            agent_sync : vec![ AwpakTUIGraphOutputDestinationConfig::Console ], 
            agent_tool_call : Default::default(), 
            agent_tool_result : Default::default(), 
            command_and_args : vec![ AwpakTUIGraphOutputDestinationConfig::Console ],
            command_result : vec![ AwpakTUIGraphOutputDestinationConfig::Console ], 
            node_destination : Default::default(),
            node_execution : vec![ AwpakTUIGraphOutputDestinationConfig::Console ]
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AwpakTUIGraphOutputDestinationConfig
{
    Console,
    File( String )
}