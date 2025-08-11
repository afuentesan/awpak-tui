use serde::{Deserialize, Serialize};

use crate::domain::{data::data::DataToString, graph::graph::Graph};


#[derive(Clone)]
pub struct GraphNode
{
    pub graph : Graph,

    pub input : Vec<DataToString>,

    pub output : Vec<GraphNodeOutput>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GraphNodeOutput
{
    Success { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Out { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Err { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Object { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
}