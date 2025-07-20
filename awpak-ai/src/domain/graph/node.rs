use serde::{Deserialize, Serialize};

use crate::domain::{agent::agent::AIAgent, agent_history_mut::agent_history_mut::AgentHistoryMut, command::command::Command, context_mut::context_mut::ContextMut, data::data::{DataComparator, DataToContext, DataToString}, graph::graph_node::{GraphNode, GraphNodeOutput}, parallel::parallel::Parallel, web_client::web_client::WebClient};


#[derive(Serialize, Deserialize, Clone)]
pub enum NodeConfig
{
    Node( Node ),
    Graph( GraphNodeConfig )
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphNodeConfig
{
    pub id : String,
    pub path : String,

    #[serde(default)]
    pub input : Vec<DataToString>,
    #[serde(default)]
    pub output : Vec<GraphNodeOutput>,

    #[serde(default)]
    pub node_output : Option<DataToContext>,
    
    #[serde(default)]
    pub node_destination : Vec<NodeDestination>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node
{
    pub id : String,
    pub executor : NodeExecutor,
    #[serde(default)]
    pub output : Option<DataToContext>,
    #[serde(default)]
    pub destination : Vec<NodeDestination>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeDestination
{
    pub next : NodeNext,
    pub condition : DataComparator
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NodeNext
{
    Node( String ),
    ExitOk( Vec<DataToString> ),
    ExitErr( Vec<DataToString> )
}

#[derive(Serialize, Deserialize, Clone)]
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
                        command : "fake command".into(), 
                        args : vec![], 
                        output : vec![]
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

