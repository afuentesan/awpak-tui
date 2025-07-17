use serde::{Deserialize, Serialize};

use crate::domain::{data::data::DataToString, graph::graph::Graph};


#[derive(Serialize, Deserialize, Clone)]
pub struct GraphNode
{
    pub graph : Graph,

    #[serde(default)]
    pub input : Vec<DataToString>,
    #[serde(default)]
    pub output : Vec<GraphNodeOutput>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GraphNodeOutput
{
    Success { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Out { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Err { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Object { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
}