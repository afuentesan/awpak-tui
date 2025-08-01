use rig::agent::Agent;


pub enum AIAgentProvider
{
    Ollama( Agent<rig::providers::ollama::CompletionModel>, Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> ),
    OpenAI( Agent<rig::providers::openai::CompletionModel>, Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> ),
    Anthropic( Agent<rig::providers::anthropic::completion::CompletionModel>, Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> ),
    DeepSeek( Agent<rig::providers::deepseek::CompletionModel>, Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> ),
    Gemini( Agent<rig::providers::gemini::completion::CompletionModel>, Vec<rmcp::service::RunningService<rmcp::RoleClient, ()>> )
}