use std::collections::HashMap;

use rig::message::Message;
use serde_json::Value;

use crate::domain::{agent::{agent::AIAgent, agent_provider::AIAgentProvider, create_agent_provider::create_agent_provider, run_agent::run_agent}, data::{data::DataToString, data_selection::data_to_string}, error::Error};


pub async fn execute_agent(
    id : Option<&String>,
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    agent : &AIAgent
) -> Result<( String, Vec<Message> ), Error>
{
    let prompt = agent_prompt( input, parsed_input, context, &agent.prompt ).await;

    let provider = create_agent_provider( 
        input,
        parsed_input,
        context,
        agent 
    ).await?;

    match provider
    {
        AIAgentProvider::Ollama( p ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::OpenAI( p ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::Gemini( p ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::Anthropic( p ) => run_agent( id, prompt, p, agent ).await,
        AIAgentProvider::DeepSeek( p ) => run_agent( id, prompt, p, agent ).await
    }
}

async fn agent_prompt(
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    from : &Vec<DataToString>
) -> String
{
    data_to_string(
        input, 
        parsed_input, 
        context, 
        from.clone()
    )
}