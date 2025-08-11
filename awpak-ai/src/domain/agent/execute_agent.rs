
use rig::message::Message;
use tracing::info;

use crate::domain::{agent::{agent::AIAgent, agent_provider::AIAgentProvider, create_agent_provider::alternate_create_agent_provider, run_agent::run_agent}, data::{data::DataToString, data_selection::data_to_string}, error::Error, graph::graph::Graph, tracing::filter_layer::AGENT_PROMPT, utils::string_utils::option_string_to_str};


pub async fn execute_agent(
    graph : &Graph,
    agent : &AIAgent
) -> Result<( String, Vec<Message> ), Error>
{
    let provider = alternate_create_agent_provider( 
        graph,
        agent 
    ).await?;

    let id = graph.id.as_ref();

    let prompt = agent_prompt( graph, &agent.prompt ).await;

    info!(
        target:AGENT_PROMPT, 
        id=option_string_to_str( id ), 
        text=prompt
    );

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
    ).await
}

// async fn append_context_to_prompt( query : String, index : Box<dyn VectorStoreIndexDyn> ) -> String
// {
//     match VectorSearchRequestBuilder::default().query( query.clone() ).samples( 5 ).build()
//     {
//         Ok( r ) =>
//         {
//             match index.top_n( r ).await
//             {
//                 Ok( r ) if r.len() > 0 => generate_prompt_with_context( query, r ),
//                 _ => query
//             }
//         },
//         _ => query
//     }
// }

// fn generate_prompt_with_context( query : String, context : Vec<( f64, String, Value )> ) -> String
// {
//     let docs = merge_documents( context );

//     format!(
//         "Use the following context to answer the user's query.\n\nUser's Query:\n{}\n\nContext:\n{}",
//         query,
//         docs
//     )
// }

// fn merge_documents( docs : Vec<( f64, String, Value )> ) -> String
// {
//     docs.iter().fold( 
//         String::new(),
//         | mut a, ( _, _, doc ) |
//         {
//             let mut doc_str = format!( "{}{}\n\n---\n\n", a, value_to_string( doc ) );

//             a.push_str( &mut doc_str );

//             a
//         }
//     )
// }