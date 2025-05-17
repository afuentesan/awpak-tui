

use std::sync::Arc;

use rig::{agent::AgentBuilder, completion::ToolDefinition, streaming::StreamingCompletionModel, tool::Tool};
use rmcp::{model::{CallToolRequestParam, Tool as MCPTool}, service::RunningService, RoleClient};
use serde_json::{Map, Value};

use crate::domain::error::Error;


#[macro_export]
macro_rules! define_tools {
    ( $( { $x:tt, $y:tt, $z:tt } ),* ) => 
    {
            $(
                pub struct $x
                {
                    name : String,
                    description : String,
                    args : Map<String, Value>,
                    service : Arc<RunningService<RoleClient, ()>>
                }

                impl Tool for $x
                {
                    const NAME: &'static str = $y;

                    type Error = Error;

                    type Args = Map<String, Value>;

                    type Output = Value;

                    async fn definition(&self, _prompt: String) -> ToolDefinition
                    {
                        ToolDefinition
                        {
                            name : $x::NAME.to_string(),
                            description : self.description.clone(),
                            parameters : serde_json::to_value( self.args.clone() ).unwrap()
                        }
                    }

                    async fn call(
                        &self,
                        args: Self::Args,
                    ) -> Result<Self::Output, Self::Error>
                    {
                        let tool_result = self.service
                        .call_tool(CallToolRequestParam {
                            name: self.name.clone().into(),
                            arguments: Some(args),
                        })
                        .await.map_err( | e | Error::MCPToolErr( e.to_string() ) )?;

                        serde_json::to_value( tool_result.content ).map_err( | e | Error::MCPToolErr( format!( "MCPToolErr: {}", e.to_string() ) ) )
                    }
                }
            )*

            pub fn add_tool<M: StreamingCompletionModel>( 
                agent: AgentBuilder<M>,
                service : Arc<RunningService<RoleClient, ()>>,
                tool : MCPTool,
                idx : usize
            ) -> Result<AgentBuilder<M>, Error>
            {
                if idx == 0
                {
                    Ok(
                        agent.tool(
                            AwpakTool0 
                            { 
                                name : tool.name.to_string(), 
                                description : tool.description.to_string(), 
                                args : (*tool.input_schema).clone(), 
                                service
                            }
                        )
                    )
                }
                $(
                else if idx == $z
                {
                    Ok(
                        agent.tool(
                            $x
                            { 
                                name : tool.name.to_string(), 
                                description : tool.description.to_string(), 
                                args : (*tool.input_schema).clone(), 
                                service
                            }
                        )
                    )
                }   
                )*
                else
                {
                    Err( Error::MCPToolErr( "MCPToolErr: Only 36 tools allowed".into() ) )    
                }
            }
    };
}

define_tools![ 
    { AwpakTool0, "tool_0", 0 }, 
    { AwpakTool1, "tool_1", 1 }, 
    { AwpakTool2, "tool_2", 2 }, 
    { AwpakTool3, "tool_3", 3 }, 
    { AwpakTool4, "tool_4", 4 }, 
    { AwpakTool5, "tool_5", 5 }, 
    { AwpakTool6, "tool_6", 6 }, 
    { AwpakTool7, "tool_7", 7 }, 
    { AwpakTool8, "tool_8", 8 }, 
    { AwpakTool9, "tool_9", 9 }, 
    { AwpakTool10, "tool_10", 10 }, 
    { AwpakTool11, "tool_11", 11 }, 
    { AwpakTool12, "tool_12", 12 }, 
    { AwpakTool13, "tool_13", 13 }, 
    { AwpakTool14, "tool_14", 14 }, 
    { AwpakTool15, "tool_15", 15 }, 
    { AwpakTool16, "tool_16", 16 }, 
    { AwpakTool17, "tool_17", 17 }, 
    { AwpakTool18, "tool_18", 18 }, 
    { AwpakTool19, "tool_19", 19 }, 
    { AwpakTool20, "tool_20", 20 }, 
    { AwpakTool21, "tool_21", 21 }, 
    { AwpakTool22, "tool_22", 22 }, 
    { AwpakTool23, "tool_23", 23 }, 
    { AwpakTool24, "tool_24", 24 }, 
    { AwpakTool25, "tool_25", 25 }, 
    { AwpakTool26, "tool_26", 26 }, 
    { AwpakTool27, "tool_27", 27 }, 
    { AwpakTool28, "tool_28", 28 }, 
    { AwpakTool29, "tool_29", 29 }, 
    { AwpakTool30, "tool_30", 30 }, 
    { AwpakTool31, "tool_31", 31 }, 
    { AwpakTool32, "tool_32", 32 }, 
    { AwpakTool33, "tool_33", 33 }, 
    { AwpakTool34, "tool_34", 34 }, 
    { AwpakTool35, "tool_35", 35 }
];