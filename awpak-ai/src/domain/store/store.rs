use std::sync::Arc;

use rig::{embeddings::embedding::EmbeddingModelDyn, providers::ollama::ALL_MINILM, vector_store::in_memory_store::InMemoryVectorStore, Embed};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub trait EmbeddingModelDynClone : EmbeddingModelDyn + Clone {}

#[derive(Default, Clone)]
pub struct Store
{
    pub provider : Arc<Mutex<Option<StoreProvider>>>,
    pub model : StoreModel
}

#[derive(Clone)]
pub enum StoreProvider
{
    InMemoryVectorStore( InMemoryVectorStore<EmbeddingDocument> )
}

impl Default for StoreProvider
{
    fn default() -> Self 
    {
        StoreProvider::InMemoryVectorStore( InMemoryVectorStore::from_documents( vec![] ) )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreConfig
{
    pub id : String,

    #[serde(default)]
    pub documents : Vec<StoreDocument>,

    pub provider : StoreProviderConfig,

    pub model : StoreModel
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StoreModel
{
    OpenAI( OpenAIStoreModel ),
    Gemini( GeminiStoreModel ),
    Ollama( OllamaStoreModel )
}

impl StoreModel
{
    pub fn model( &self ) -> &str
    {
        match self
        {
            StoreModel::OpenAI( s ) => &s.model,
            StoreModel::Gemini( s ) => &s.model,
            StoreModel::Ollama( s ) => &s.model
        }
    }

    pub fn api_key( &self ) -> Option<&str>
    {
        match self
        {
            StoreModel::OpenAI( s ) => Some( &s.api_key ),
            StoreModel::Gemini( s ) => Some( &s.api_key ),
            StoreModel::Ollama( _ ) => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIStoreModel
{
    pub api_key : String,
    pub model : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeminiStoreModel
{
    pub api_key : String,
    pub model : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaStoreModel
{
    pub model : String
}

impl Default for StoreModel
{
    fn default() -> Self
    {
        StoreModel::Ollama(
            OllamaStoreModel { model : ALL_MINILM.to_string() }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StoreProviderConfig
{
    InMemoryVectorStore
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StoreDocument
{
    Text { path : String, #[serde(default)] sizer : StoreDocumentSizer },
    Pdf { path : String, #[serde(default)] sizer : StoreDocumentSizer }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum StoreDocumentSizer
{
    Chars { #[serde(default)] desired : Option<usize>, max : usize },
    Markdown { #[serde(default)] desired : Option<usize>, max : usize },
    #[default]
    None
}

impl StoreDocumentSizer
{
    pub fn is_none( &self ) -> bool
    {
        match self
        {
            StoreDocumentSizer::None => true,
            _ => false    
        }
    }
}

impl StoreDocument
{
    pub fn from( &self ) -> &String
    {
        match self
        {
            StoreDocument::Text { path, sizer : _ } |
            StoreDocument::Pdf { path, sizer : _ } => path    
        }
    }

    pub fn prefix( &self ) -> &'static str
    {
        match self
        {
            StoreDocument::Text { path : _, sizer : _ } => "text_",
            StoreDocument::Pdf { path : _, sizer : _ } => "pdf_"
        }
    }

    pub fn sizer( &self ) -> Option<&StoreDocumentSizer>
    {
        match self
        {
            StoreDocument::Text { path : _, sizer } => Some( sizer ),
            StoreDocument::Pdf { path : _, sizer } => Some( sizer )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmbeddingDocument
{
    pub id : String,
    pub content : String
}

impl Embed for EmbeddingDocument
{
    fn embed( &self, embedder: &mut rig::embeddings::TextEmbedder ) -> Result<(), rig::embeddings::EmbedError> 
    {
        embedder.embed( self.content.clone() );
        
        Ok( () )
    }
}