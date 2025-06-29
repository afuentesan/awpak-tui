use rig::agent::Agent;


pub enum AIAgentProvider
{
    Ollama( Agent<rig::providers::ollama::CompletionModel> ),
    OpenAI( Agent<rig::providers::openai::CompletionModel> ),
    Anthropic( Agent<rig::providers::anthropic::completion::CompletionModel> ),
    DeepSeek( Agent<rig::providers::deepseek::DeepSeekCompletionModel> ),
    Gemini( Agent<rig::providers::gemini::completion::CompletionModel> )
}