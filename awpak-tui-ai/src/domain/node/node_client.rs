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
    pub provider : NodeClientProvider,

    pub tools_output : Option<NodeOutputDestination>,
    pub millis_between_tool_call : Option<u64>,

    pub millis_between_streams : Option<u64>
}



#[derive(Clone)]
pub enum NodeClientProvider
{
    Ollama( Arc<Agent<rig::providers::ollama::CompletionModel>> ),
    OpenAI( Arc<Agent<rig::providers::openai::CompletionModel>> ),
    Anthropic( Arc<Agent<rig::providers::anthropic::completion::CompletionModel>> ),
    DeepSeek( Arc<Agent<rig::providers::deepseek::DeepSeekCompletionModel>> ),
    Gemini( Arc<Agent<rig::providers::gemini::completion::CompletionModel>> )
}