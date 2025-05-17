use crate::domain::{chain::chain_client::ChainClient, data::data::InputData, node::node_client::NodeClient};


#[derive(Clone)]
pub struct RepeatClient
{
    pub id : String,
    pub provider : Box<RepeatClientProvider>,
    pub input : Vec<InputData>
}

#[derive(Clone)]
pub enum RepeatClientProvider
{
    Node( NodeClient ),
    Chain( ChainClient ),
    Repeat( RepeatClient )
}