
use rig::client::CompletionClient;
use tracing::info;

use crate::domain::{agent::{agent::{AIAgent, AIAgentProviderConfig, AnthropicConfig, DeepSeekConfig, GeminiConfig, OllamaConfig, OpenAIConfig}, agent_provider::AIAgentProvider}, data::{data_selection::{data_selection, data_to_string}, data_utils::value_to_string}, error::Error, graph::graph::Graph, mcp::mcp_functions::add_mcp_clients_to_agent, tracing::filter_layer::AGENT_SYSTEM_PROMPT, utils::string_utils::option_string_to_str};

// CREATE AGENT PROVIDER

pub async fn alternate_create_agent_provider( 
    graph : &Graph,
    agent : &AIAgent
) -> Result<AIAgentProvider, Error>
{
    match &agent.provider
    {
        AIAgentProviderConfig::Ollama( c ) => ollama_agent_provider( graph, agent, c ).await,
        AIAgentProviderConfig::OpenAI( c ) => openai_agent_provider( graph, agent,c ).await,
        AIAgentProviderConfig::Anthropic( c ) => anthropic_agent_provider( graph, agent, c ).await,
        AIAgentProviderConfig::DeepSeek( c ) => deepseek_agent_provider( graph, agent, c ).await,
        AIAgentProviderConfig::Gemini( c ) => gemini_agent_provider( graph, agent,c ).await
    }
}

fn trace_system_prompt( id : Option<&String>, system_prompt : &str )
{
    info!(
        target:AGENT_SYSTEM_PROMPT, 
        id=option_string_to_str( id ), 
        text=system_prompt
    );
}

async fn gemini_agent_provider( 
    graph : &Graph,
    ai_agent : &AIAgent,
    config : &GeminiConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::gemini::Client::new( &api_key );

    let model = value_to_string( &data_selection( graph, &config.model ).await? );

    let agent = client.agent( &model );

    let ( mut agent, clients ) = add_mcp_clients_to_agent( graph, agent, &ai_agent.servers ).await?;

    let system_prompt = data_to_string( graph, ai_agent.system_prompt.clone() ).await;

    if system_prompt.trim() != ""
    {
        agent = agent.preamble( system_prompt.as_str() );
    }

    trace_system_prompt( graph.id.as_ref(), &system_prompt );

    let agent = agent.build();

    Ok( AIAgentProvider::Gemini( agent, clients ) )
}

async fn deepseek_agent_provider( 
    graph : &Graph,
    ai_agent : &AIAgent,
    config : &DeepSeekConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::deepseek::Client::new( &api_key );

    let model = value_to_string( &data_selection( graph, &config.model ).await? );

    let mut agent = client.agent( &model );

    if let Some( m ) = config.max_tokens
    {
        agent = agent.max_tokens( m );
    }

    let ( mut agent, clients ) = add_mcp_clients_to_agent( graph, agent, &ai_agent.servers ).await?;

    let system_prompt = data_to_string( graph, ai_agent.system_prompt.clone() ).await;

    if system_prompt.trim() != ""
    {
        agent = agent.preamble( system_prompt.as_str() );
    }

    trace_system_prompt( graph.id.as_ref(), &system_prompt );
    
    let agent = agent.build();

    Ok( AIAgentProvider::DeepSeek( agent, clients ) )
}

async fn ollama_agent_provider( 
    graph : &Graph,
    ai_agent : &AIAgent,
    config : &OllamaConfig
) -> Result<AIAgentProvider, Error>
{
    let client = rig::providers::ollama::Client::new();

    let model = value_to_string( &data_selection( graph, &config.model ).await? );

    let agent = client.agent( &model );

    let ( mut agent, clients ) = add_mcp_clients_to_agent( graph, agent, &ai_agent.servers ).await?;

    let system_prompt = data_to_string( graph, ai_agent.system_prompt.clone() ).await;

    if system_prompt.trim() != ""
    {
        agent = agent.preamble( system_prompt.as_str() );
    }

    trace_system_prompt( graph.id.as_ref(), &system_prompt );

    Ok( AIAgentProvider::Ollama( agent.build(), clients ) )
}

async fn openai_agent_provider( 
    graph : &Graph,
    ai_agent : &AIAgent,
    config : &OpenAIConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::openai::Client::new( &api_key );

    let model = value_to_string( &data_selection( graph, &config.model ).await? );

    let agent = client.agent( &model );

    let ( mut agent, clients ) = add_mcp_clients_to_agent( graph, agent, &ai_agent.servers ).await?;

    let system_prompt = data_to_string( graph, ai_agent.system_prompt.clone() ).await;

    if system_prompt.trim() != ""
    {
        agent = agent.preamble( system_prompt.as_str() );
    }

    trace_system_prompt( graph.id.as_ref(), &system_prompt );
    
    let agent = agent.build();

    Ok( AIAgentProvider::OpenAI( agent, clients ) )
}

async fn anthropic_agent_provider( 
    graph : &Graph,
    ai_agent : &AIAgent,
    config : &AnthropicConfig
) -> Result<AIAgentProvider, Error>
{
    let api_key = std::env::var( &config.api_key ).map_err( | e | Error::Agent( e.to_string() ) )?;

    let client = rig::providers::anthropic::ClientBuilder::new( &api_key ).build().map_err( | e | Error::Agent( e.to_string() ) )?;

    let model = value_to_string( &data_selection( graph, &config.model ).await? );

    let agent = client.agent( &model ).max_tokens( config.max_tokens );

    let ( mut agent, clients ) = add_mcp_clients_to_agent( graph, agent, &ai_agent.servers ).await?;

    let system_prompt = data_to_string( graph, ai_agent.system_prompt.clone() ).await;

    if system_prompt.trim() != ""
    {
        agent = agent.preamble( system_prompt.as_str() );
    }
    
    trace_system_prompt( graph.id.as_ref(), &system_prompt );

    let agent = agent.build();

    Ok( AIAgentProvider::Anthropic( agent, clients ) )
}

// END CREATE AGENT PROVIDER