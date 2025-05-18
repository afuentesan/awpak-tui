use serde_json::Value;

use crate::domain::data::data::{InputData, OutputData};


#[derive(Clone)]
pub struct ChainClient
{
    pub id : String,
    pub items : Vec<ChainClientItem>,
    pub initial_context : Option<Value>
}

#[derive(Clone)]
pub struct ChainClientItem
{
    pub id : String,
    pub input : Vec<InputData>,
    pub input_separator : Option<String>,
    pub output : OutputData,
    pub provider : ChainClientProvider
}

#[derive(Clone)]
pub enum ChainClientProvider
{
    Node( String ),
    Chain( String ),
    Repeat( String )
}