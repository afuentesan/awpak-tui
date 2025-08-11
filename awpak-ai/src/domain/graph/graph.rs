use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::{data::data::DataType, graph::node::{Node, NodeConfig}, store::store::{Store, StoreConfig}};

#[derive(Default, Clone)]
pub struct Graph
{
    pub id : Option<String>,

    pub stores : HashMap<String, Store>,

    pub input : Option<String>,
    pub input_type : Option<DataType>,
    pub parsed_input : Value,

    pub context : HashMap<String, Value>,

    pub first : String,
    pub nodes : HashMap<String, Node>,
    
    pub final_output : Option<Result<String, String>>,

    __clean_context : bool,
    __initial_context : HashMap<String, Value>
}

impl Graph
{
    pub fn new(
        stores : HashMap<String, Store>,
        input_type : Option<DataType>,
        context : HashMap<String, Value>,
        first : String,
        nodes : HashMap<String, Node>,
        preserve_context : bool
    ) -> Self
    {
        let initial_context = if preserve_context
        {
            HashMap::new()
        }
        else
        {
            context.clone()
        };

        Self 
        { 
            id : None,

            stores,

            input : None, 
            input_type, 
            parsed_input : Value::Null, 

            context, 

            first, 
            nodes, 

            final_output : None, 

            __clean_context: ! preserve_context, 
            __initial_context : initial_context
        }
    }

    pub fn init_context( mut self ) -> Graph
    {
        if ! self.__clean_context { return self };

        self.context = self.__initial_context.clone();

        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphConfig
{
    #[serde(default)]
    pub stores : Vec<StoreConfig>,

    #[serde(default)]
    pub context : HashMap<String, Value>,

    pub first : NodeConfig,
    #[serde(default)]
    pub nodes : Vec<NodeConfig>,
    
    #[serde(default)]
    pub input_type : Option<DataType>,
    #[serde(default)]
    pub preserve_context : bool
}