use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::{agent::agent::AIAgent, agent_history_mut::agent_history_mut::AgentHistoryMut, command::command::Command, context_mut::context_mut::ContextMut, data::data::{DataComparator, DataFrom, DataToContext, DataToString}, graph::graph_node::{GraphNode, GraphNodeOutput}, parallel::parallel::Parallel, web_client::web_client::WebClient};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfig
{
    pub id : String,
    pub executor : NodeExecutorConfig,
    #[serde(default)]
    pub output : Option<DataToContext>,
    #[serde(default)]
    pub destination : Vec<NodeDestination>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeExecutorConfig
{
    Agent( AIAgent ),
    Command( Command ),
    Graph( GraphNodeConfig ),
    ContextMut( Vec<ContextMut> ),
    WebClient( WebClient ),
    AgentHistoryMut( Vec<AgentHistoryMut> ),
    Parallel( Parallel )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphNodeConfig
{
    pub path : String,

    #[serde(default)]
    pub input : Vec<DataToString>,
    #[serde(default)]
    pub output : Vec<GraphNodeOutput>
}

#[derive(Clone)]
pub struct Node
{
    pub id : String,
    pub executor : NodeExecutor,
    pub output : Option<DataToContext>,
    pub destination : Vec<NodeDestination>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeDestination
{
    pub next : NodeNext,
    pub condition : DataComparator
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeNext
{
    Node( String ),
    ExitOk( Vec<DataToString> ),
    ExitErr( Vec<DataToString> )
}

#[derive(Clone)]
pub enum NodeExecutor
{
    Agent( AIAgent ),
    Command( Command ),
    Graph( GraphNode ),
    ContextMut( Vec<ContextMut> ),
    WebClient( WebClient ),
    AgentHistoryMut( Vec<AgentHistoryMut> ),
    Parallel( Parallel )
}

impl NodeExecutor
{
    pub fn context_mut( &self ) -> Option<&Vec<ContextMut>>
    {
        match self
        {
            NodeExecutor::ContextMut( c ) => Some( c ),
            _ => None    
        }
    }

    pub fn own_graph( self ) -> ( Self, Option<GraphNode> )
    {
        match self
        {
            NodeExecutor::Graph( g ) => ( 
                NodeExecutor::Command( 
                    Command 
                    { 
                        command : DataFrom::Static( Value::String( "fake command".into() ) ), 
                        args : vec![], 
                        output : vec![],
                        timeout : None
                    }
                ),
                Some( g )
            ),
            _ => ( self, None )
        }
    }

    pub fn agent( &self ) -> Option<&AIAgent>
    {
        match self
        {
            NodeExecutor::Agent( a ) => Some( a ),
            _ => None    
        }
    }

    pub fn own_agent( self ) -> ( Self, Option<AIAgent> )
    {
        match self
        {
            NodeExecutor::Agent( a ) =>
            {
                ( NodeExecutor::Agent( AIAgent::default() ), Some( a ) )
            },
            _ => ( self, None )
        }
    }
}

