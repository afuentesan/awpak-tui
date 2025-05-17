use std::sync::Arc;

use rig::{agent::Agent, message::Message};

use super::node::NodeOutputDestination;


#[derive(Clone)]
pub struct NodeClient
{
    pub id : String, 
    pub history : Vec<Message>, 
    pub save_history : bool,
    pub output : NodeOutputDestination,
    pub provider : NodeClientProvider
}



#[derive(Clone)]
pub enum NodeClientProvider
{
    Ollama( Arc<Agent<rig::providers::ollama::CompletionModel>> ),
    OpenAI( Arc<Agent<rig::providers::openai::CompletionModel>> )
}