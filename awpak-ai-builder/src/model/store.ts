import { AIAgentProviderGemini, AIAgentProviderOllama, AIAgentProviderOpenAI } from "./agent";

export class StoreConfig
{
    id : string;

    provider : StoreProvider = new InMemoryVectorStoreProvider();

    model : StoreModel = new OllamaStoreModel();

    documents : Array<StoreDocument> = [];

    constructor( id : string )
    {
        if( ! id?.trim() ) throw new Error( "StoreConfig id cannot be empty" );

        this.id = id;
    }
}

export type StoreDocument = StoreDocumentText | StoreDocumentPdf;

export enum StoreDocumentVariant
{
    Text = "Text",
    Pdf = "Pdf"
}

export class StoreDocumentText
{
    readonly _variant = StoreDocumentVariant.Text;

    path : string = "";
    sizer : StoreDocumentSizer = new StoreDocumentSizerNone();
}

export class StoreDocumentPdf
{
    readonly _variant = StoreDocumentVariant.Pdf;

    path : string = "";
    sizer : StoreDocumentSizer = new StoreDocumentSizerNone();
}

export type StoreDocumentSizer = StoreDocumentSizerChars |
                                 StoreDocumentSizerMarkdown | 
                                 StoreDocumentSizerNone;

export enum StoreDocumentSizerVariant
{
    Chars = "Chars",
    Markdown = "Markdown",
    None = "None"
}

export class StoreDocumentSizerChars
{
    readonly _variant = StoreDocumentSizerVariant.Chars;

    desired : number | undefined;
    max : number = 1024;
}

export class StoreDocumentSizerMarkdown
{
    readonly _variant = StoreDocumentSizerVariant.Markdown;

    desired : number | undefined;
    max : number = 1024;
}

export class StoreDocumentSizerNone
{
    readonly _variant = StoreDocumentSizerVariant.None;
}

export type StoreProvider = InMemoryVectorStoreProvider | 
                            PostgresStoreProvider;

export enum StoreProviderVariant
{
    InMemoryVectorStore = "InMemoryVectorStore",
    Postgres = "Postgres"
}

export class InMemoryVectorStoreProvider
{
    readonly _variant = StoreProviderVariant.InMemoryVectorStore;
}

export class PostgresStoreProvider
{
    readonly _variant = StoreProviderVariant.Postgres;

    database_url : string = "";
    table_name : string | undefined;
    raw_database_url : boolean = false;
}

export type StoreModel = OpenAIStoreModel | 
                         GeminiStoreModel | 
                         OllamaStoreModel;

export enum StoreModelVariant
{
    OpenAI = "OpenAI",
    Gemini = "Gemini",
    Ollama = "Ollama"
}

export class OpenAIStoreModel
{
    readonly _variant = StoreModelVariant.OpenAI;

    model : string = "";
    api_key : string = "";
}

export class GeminiStoreModel
{
    readonly _variant = StoreModelVariant.Gemini;

    model : string = "";
    api_key : string = "";
}

export class OllamaStoreModel
{
    readonly _variant = StoreModelVariant.Ollama;

    model : string = "";
}