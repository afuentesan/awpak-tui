
use rig::message::Message;
use tracing::info;

use crate::domain::{agent::{agent::AIAgent, agent_provider::AIAgentProvider, create_agent_provider::create_agent_provider, run_agent::run_agent}, data::{data::DataToString, data_selection::data_to_string}, error::Error, graph::graph::Graph, tracing::filter_layer::AGENT_PROMPT, utils::string_utils::option_string_to_str};


pub async fn execute_agent(
    graph : &Graph,
    agent : &AIAgent
) -> Result<( String, Vec<Message> ), Error>
{
    let id = graph.id.as_ref();

    let prompt = agent_prompt( graph, &agent.prompt ).await;

    info!(
        target:AGENT_PROMPT, 
        id=option_string_to_str( id ), 
        text=prompt
    );

    let provider = create_agent_provider( 
        graph,
        agent 
    ).await?;

    match provider
    {
        AIAgentProvider::Ollama( p, _ ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::OpenAI( p, _ ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::Gemini( p, _ ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::Anthropic( p, _ ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::DeepSeek( p, _ ) => run_agent( id, prompt, p, agent ).await
    }
}

async fn agent_prompt(
    graph : &Graph,
    from : &Vec<DataToString>
) -> String
{
    data_to_string(
        graph, 
        from.clone()
    )
}