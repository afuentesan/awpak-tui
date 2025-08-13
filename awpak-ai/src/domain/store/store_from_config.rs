use std::sync::Arc;

use rig::{client::EmbeddingsClient, embeddings::{Embedding, EmbeddingModel, EmbeddingsBuilder}, loaders::{FileLoader, PdfFileLoader}, vector_store::in_memory_store::InMemoryVectorStore, OneOrMany};
use text_splitter::{MarkdownSplitter, TextSplitter};
use tokio::sync::Mutex;

use crate::domain::{error::{ChangeError, Error}, store::{postgres_store::postgres_store_provider, store::{EmbeddingDocument, GeminiStoreModel, OllamaStoreModel, OpenAIStoreModel, Store, StoreConfig, StoreDocument, StoreDocumentSizer, StoreModel, StoreProvider, StoreProviderConfig}}};


pub async fn store_from_config( config : StoreConfig ) -> Result<Store, Error>
{
    let config_model = parse_config_model( config.model )?;

    let provider = provider_from_config( config.provider, config_model.clone(), config.documents ).await?;

    Ok(
        Store
        {
            model : config_model,
            provider : Arc::new( Mutex::new( Some( provider ) ) )
        }
    )
}

fn parse_config_model( config : StoreModel ) -> Result<StoreModel, Error>
{
    match config
    {
        StoreModel::Ollama( m ) => Ok( StoreModel::Ollama( m ) ),
        StoreModel::OpenAI( m ) =>
        {
            Ok(
                StoreModel::OpenAI(
                    OpenAIStoreModel 
                    { 
                        api_key : api_key( &m.api_key )?, 
                        model : m.model
                    }
                )
            )
        },
        StoreModel::Gemini( m ) =>
        {
            {
                Ok(
                    StoreModel::Gemini(
                        GeminiStoreModel
                        { 
                            api_key : api_key( &m.api_key )?, 
                            model : m.model
                        }
                    )
                )
            }
        }
    }
}

fn api_key( api_key : &str ) -> Result<String, Error>
{
    std::env::var( &api_key ).map_err( | e | Error::Agent( e.to_string() ) )
}

async fn provider_from_config( 
    config : StoreProviderConfig, 
    model : StoreModel, 
    documents : Vec<StoreDocument> 
) -> Result<StoreProvider, Error>
{
    match config
    {
        StoreProviderConfig::InMemoryVectorStore => in_memory_vector_store_provider( model, documents ).await,
        StoreProviderConfig::Postgres( p ) => postgres_store_provider( model, documents, p ).await
    }
}

async fn in_memory_vector_store_provider( model : StoreModel, documents : Vec<StoreDocument> ) -> Result<StoreProvider, Error>
{
    match model
    {
        StoreModel::OpenAI( m ) =>
        {
            let model = openai_store_model( &m )?;

            let documents = provider_embeddings( documents, model ).await?;

            Ok( StoreProvider::InMemoryVectorStore( InMemoryVectorStore::from_documents( documents ) ) )
        },
        StoreModel::Ollama( m ) =>
        {
            let model = ollama_store_model( &m )?;

            let documents = provider_embeddings( documents, model ).await?;

            Ok( StoreProvider::InMemoryVectorStore( InMemoryVectorStore::from_documents( documents ) ) )
        },
        StoreModel::Gemini( m ) =>
        {
            let model = gemini_store_model( &m )?;

            let documents = provider_embeddings( documents, model ).await?;

            Ok( StoreProvider::InMemoryVectorStore( InMemoryVectorStore::from_documents( documents ) ) )
        }
    }
}

pub fn openai_store_model( model : &OpenAIStoreModel ) -> Result<rig::providers::openai::embedding::EmbeddingModel, Error>
{
    let client = rig::providers::openai::Client::new( &model.api_key );

    Ok( client.embedding_model( &model.model ) )
}

pub fn gemini_store_model( model : &GeminiStoreModel ) -> Result<rig::providers::gemini::embedding::EmbeddingModel, Error>
{
    let client = rig::providers::gemini::Client::new( &model.api_key );

    Ok( client.embedding_model( &model.model ) )
}

pub fn ollama_store_model( model : &OllamaStoreModel ) -> Result<rig::providers::ollama::EmbeddingModel, Error>
{
    let client = rig::providers::ollama::Client::new();

    Ok( client.embedding_model( &model.model ) )
}

pub async fn provider_embeddings<M: EmbeddingModel>(
    documents : Vec<StoreDocument> ,
    embedding_model : M
) -> Result<Vec<(EmbeddingDocument, OneOrMany<Embedding>)>, Error>
{
    let embeddings_str = embedding_document_from_documents( documents ).await?;

    if embeddings_str.len() == 0 { return Ok( vec![] ) }

    Ok(
        EmbeddingsBuilder::new( embedding_model.clone() )
        .documents( embeddings_str )
        .map_err( | e | Error::Store( e.to_string() ) ).prepend_err( "Embeddings documents" )?
        .build()
        .await
        .map_err( | e | Error::Store( e.to_string() ) ).prepend_err( "Embeddings build" )?
    )
}

async fn embedding_document_from_documents( documents : Vec<StoreDocument> ) -> Result<Vec<EmbeddingDocument>, Error>
{
    let mut ret = vec![];

    let mut count : usize = 0;

    for doc in documents
    {
        let text = match &doc
        {
            StoreDocument::Text { path, sizer : _ } => embedding_text( path ).await?,
            StoreDocument::Pdf { path, sizer : _ } => embedding_pdf( path ).await?,
        };

        let text = chunks_from_vec_str( text, doc.sizer() );

        for t in text
        {
            ret.push( embedding_document_from_str( t, count ) );

            count += 1;
        }
    }

    Ok( ret )
}

fn chunks_from_vec_str( text : Vec<String>, sizer : Option<&StoreDocumentSizer> ) -> Vec<String>
{
    if text.len() == 0 { return text }

    match sizer
    {
        Some( s ) if ! s.is_none() => chunks_with_sizer( text, s ),
        _ => text
    }
}

fn chunks_with_sizer( text : Vec<String>, sizer : &StoreDocumentSizer ) -> Vec<String>
{
    let join_docs = text.join( "\n\n\n" );

    match sizer
    {
        StoreDocumentSizer::Chars { desired, max } => chunks_by_chars( join_docs, desired.as_ref(), max ),
        StoreDocumentSizer::Markdown { desired, max } => chunks_by_markdown( join_docs, desired.as_ref(), max ),
        StoreDocumentSizer::None => unreachable!()
    }
}

fn chunks_by_markdown( text : String, desired : Option<&usize>, max : &usize ) -> Vec<String>
{
    match desired
    {
        Some( d ) if d < max =>
        {
            MarkdownSplitter::new( *d..*max )
        },
        _ => MarkdownSplitter::new( *max )
    }
    .chunks( &text )
    .map( | s | s.to_string() )
    .collect()
}

fn chunks_by_chars( text : String, desired : Option<&usize>, max : &usize ) -> Vec<String>
{
    match desired
    {
        Some( d ) if d < max =>
        {
            TextSplitter::new( *d..*max )
        },
        _ => TextSplitter::new( *max )
    }
    .chunks( &text )
    .map( | s | s.to_string() )
    .collect()
}

fn embedding_document_from_str( text : String, idx : usize ) -> EmbeddingDocument
{
    EmbeddingDocument 
    { 
        id : format!( "{idx}" ), 
        content : text 
    }
}

async fn embedding_text( path : &str ) -> Result<Vec<String>, Error>
{
    Ok(
        FileLoader::with_glob( path )
        .map_err( | e | Error::Agent( e.to_string() ) )?
        .read_with_path()
        .ignore_errors()
        .into_iter()
        .map( | ( _, c ) | c )
        .collect::<Vec<_>>()
    )
}

async fn embedding_pdf( path : &str ) -> Result<Vec<String>, Error>
{
    Ok(
        PdfFileLoader::with_glob( path )
        .map_err( | e | Error::Agent( e.to_string() ) )?
        .read_with_path()
        .ignore_errors()
        .into_iter()
        .map( | ( _, c ) | c )
        .collect::<Vec<_>>()
    )
}