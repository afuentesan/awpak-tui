use std::collections::HashMap;

use rig::client::CompletionClient;
use serde_json::Value;

use crate::domain::{agent::{agent::{AIAgent, AIAgentProviderConfig, AnthropicConfig, DeepSeekConfig, GeminiConfig, OllamaConfig, OpenAIConfig}, agent_provider::AIAgentProvider}, error::Error, mcp::mcp_functions::add_mcp_clients_to_agent};

// CREATE AGENT PROVIDER

pub async fn create_agent_provider( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    agent : &AIAgent
) -> Result<AIAgentProvider, Error>
{
    match &agent.provider
    {
        AIAgentProviderConfig::Ollama( c ) => ollama_agent_provider( input, parsed_input, context, agent, c ).await,
        AIAgentProviderConfig::OpenAI( c ) => openai_agent_provider( input, parsed_input, context, agent,c ).await,
        AIAgentProviderConfig::Anthropic( c ) => anthropic_agent_provider( input, parsed_input, context, agent, c ).await,
        AIAgentProviderConfig::DeepSeek( c ) => deepseek_agent_provider( input, parsed_input, context, agent, c ).await,
        AIAgentProviderConfig::Gemini( c ) => gemini_agent_provider( input, parsed_input, context, agent,c ).await
    }
}

async fn gemini_agent_provider( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    ai_agent : &AIAgent,
    config : &GeminiConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::gemini::Client::new( &api_key );

    let agent = client.agent( &config.model );

    let mut agent = add_mcp_clients_to_agent( input, parsed_input, context, agent, &ai_agent.servers ).await?;

    if let Some( p ) = ai_agent.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }
    
    let agent = agent.build();

    Ok( AIAgentProvider::Gemini( agent ) )
}

async fn deepseek_agent_provider( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    ai_agent : &AIAgent,
    config : &DeepSeekConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::deepseek::Client::new( &api_key );

    let mut agent = client.agent( &config.model );

    if let Some( m ) = config.max_tokens
    {
        agent = agent.max_tokens( m );
    }

    let mut agent = add_mcp_clients_to_agent( input, parsed_input, context, agent, &ai_agent.servers ).await?;

    if let Some( p ) = ai_agent.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }
    
    let agent = agent.build();

    Ok( AIAgentProvider::DeepSeek( agent ) )
}

async fn ollama_agent_provider( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    ai_agent : &AIAgent,
    config : &OllamaConfig
) -> Result<AIAgentProvider, Error>
{
    let client = rig::providers::ollama::Client::new();

    let agent = client.agent( &config.model );

    let mut agent = add_mcp_clients_to_agent( input, parsed_input, context, agent, &ai_agent.servers ).await?;

    if let Some( p ) = ai_agent.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }

    Ok( AIAgentProvider::Ollama( agent.build() ) )
}

async fn openai_agent_provider( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    ai_agent : &AIAgent,
    config : &OpenAIConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::openai::Client::new( &api_key );

    let agent = client.agent( &config.model );

    let mut agent = add_mcp_clients_to_agent( input, parsed_input, context, agent, &ai_agent.servers ).await?;

    if let Some( p ) = ai_agent.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }
    
    let agent = agent.build();

    Ok( AIAgentProvider::OpenAI( agent ) )
}

async fn anthropic_agent_provider( 
    input : Option<&String>, 
    parsed_input : &Value, 
    context : &HashMap<String, Value>,
    ai_agent : &AIAgent,
    config : &AnthropicConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::anthropic::ClientBuilder::new( &api_key ).build();

    // let client = rig::providers::anthropic::Client::from_env()

    let agent = client.agent( &config.model ).max_tokens( config.max_tokens );

    let mut agent = add_mcp_clients_to_agent( input, parsed_input, context, agent, &ai_agent.servers ).await?;

    if let Some( p ) = ai_agent.system_prompt.as_ref()
    {
        agent = agent.preamble( p.as_str() );
    }
    
    let agent = agent.build();

    Ok( AIAgentProvider::Anthropic( agent ) )
}

// END CREATE AGENT PROVIDER