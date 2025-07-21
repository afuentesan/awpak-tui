import { FromStatic, type DataFrom, type DataToString } from "./data";


export class AIAgent
{
    provider : AIAgentProvider;

    system_prompt : Array<DataToString> = [];
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

    model : DataFrom = new FromStatic();
}

export class AIAgentProviderOpenAI
{
    readonly _variant = AIAgentProviderConfigVariant.OpenAI;

    model : DataFrom = new FromStatic();
    api_key : string | undefined;
}

export class AIAgentProviderAnthropic
{
    readonly _variant = AIAgentProviderConfigVariant.Anthropic;

    model : DataFrom = new FromStatic();
    api_key : string | undefined;
    max_tokens : number | undefined;
}

export class AIAgentProviderDeepSeek
{
    readonly _variant = AIAgentProviderConfigVariant.DeepSeek;

    model : DataFrom = new FromStatic();
    api_key : string | undefined;
    max_tokens : number | undefined;
}

export class AIAgentProviderGemini
{
    readonly _variant = AIAgentProviderConfigVariant.Gemini;

    model : DataFrom = new FromStatic();
    api_key : string | undefined;
}