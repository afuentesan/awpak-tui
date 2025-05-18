use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Default)]
pub enum DataSelection
{
    FromMap( String ),
    FromString,

    #[default]
    None
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum DataMerge
{
    FromMapToMap { #[serde(default)] parent_key : Option<String>, keys : Vec<String> },
    FromStringToString { #[serde(default)] sep : Option<String> },
    FromStringToMap( String ),
    FromArrayToMap( String ),

    #[default]
    None
}

#[derive(Serialize, Deserialize, Clone)]
pub enum InputData
{
    Prompt { title : Option<String>, footer : Option<String>, selection : DataSelection },
    Context { title : Option<String>, footer : Option<String>, selection : DataSelection },
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum OutputData
{
    Output( DataMerge ),
    Context( DataMerge ),

    #[default]
    None
}