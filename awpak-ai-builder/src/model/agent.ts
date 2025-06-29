import type { DataFrom, DataToString } from "./data";


export class AIAgent
{
    provider : AIAgentProvider;

    system_prompt : string | undefined;
    save_history : boolean = false;
    
    servers : Array<NodeMCPServer> = [];

    prompt : Array<DataToString> = [];

    constructor()
    {
        this.provider = new AIAgentProviderOllama();
    }
}

export class NodeMCPServer
{
    command : string | undefined;
    args : Array<DataFrom> = [];
}

export type AIAgentProvider = AIAgentProviderOllama |
                              AIAgentProviderOpenAI |
                              AIAgentProviderAnthropic |
                              AIAgentProviderDeepSeek |
                              AIAgentProviderGemini;


export enum AIAgentProviderConfigVariant
{
    Ollama = "Ollama",
    OpenAI = "OpenAI",
    Anthropic = "Anthropic",
    DeepSeek = "DeepSeek",
    Gemini = "Gemini"
}

export class AIAgentProviderOllama
{
    readonly _variant = AIAgentProviderConfigVariant.Ollama;

    model : string | undefined
}

export class AIAgentProviderOpenAI
{
    readonly _variant = AIAgentProviderConfigVariant.OpenAI;

    model : string | undefined;
    api_key : string | undefined;
}

export class AIAgentProviderAnthropic
{
    readonly _variant = AIAgentProviderConfigVariant.Anthropic;

    model : string | undefined;
    api_key : string | undefined;
    max_tokens : number | undefined;
}

export class AIAgentProviderDeepSeek
{
    readonly _variant = AIAgentProviderConfigVariant.DeepSeek;

    model : string | undefined;
    api_key : string | undefined;
    max_tokens : number | undefined;
}

export class AIAgentProviderGemini
{
    readonly _variant = AIAgentProviderConfigVariant.Gemini;

    model : string | undefined;
    api_key : string | undefined;
}