use std::{collections::HashMap, sync::{Arc, OnceLock}};

use rig::{embeddings::EmbeddingModel, vector_store::VectorSearchRequest};
use rig::vector_store::InsertDocuments;
use rig::vector_store::VectorStoreIndex;
use rig_postgres::{PgVectorDistanceFunction, PostgresVectorStore};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::Mutex;

use crate::domain::{error::Error, store::{store::{EmbeddingDocument, PostgresStoreProvider, PostgresStoreProviderConfig, StoreDocument, StoreModel, StoreProvider}, store_from_config::{gemini_store_model, ollama_store_model, openai_store_model, provider_embeddings}}};

pub async fn query_postgres_store(
    provider : &PostgresStoreProvider,
    query : &str,
    samples : u64
) -> Result<Vec<( f64, String, EmbeddingDocument )>, Error>
{
    let req = VectorSearchRequest::builder()
    .query( query )
    .samples( samples )
    .build()
    .map_err( | e | Error::Store( e.to_string() ) )?;

    match provider
    {
        PostgresStoreProvider::Gemini( p ) =>
        {
            p
            .top_n::<EmbeddingDocument>(req).await
            .map_err( | e | Error::Store( e.to_string() ) )
        },
        PostgresStoreProvider::OpenAI( p ) =>
        {
            p
            .top_n::<EmbeddingDocument>(req).await
            .map_err( | e | Error::Store( e.to_string() ) )
        },
        PostgresStoreProvider::Ollama( p ) =>
        {
            p
            .top_n::<EmbeddingDocument>(req).await
            .map_err( | e | Error::Store( e.to_string() ) )
        }
    }
    
}

pub async fn postgres_store_provider( 
    model : StoreModel, 
    documents : Vec<StoreDocument>,
    config : PostgresStoreProviderConfig
) -> Result<StoreProvider, Error>
{
    let database_url = database_url( &config )?;

    let pool = pg_pool( database_url ).await?;

    match model
    {
        StoreModel::Gemini( m ) =>
        {
            let model = gemini_store_model( &m )?;

            let store = provider_from_model( model, pool, documents, config ).await?;

            Ok(
                StoreProvider::Postgres(
                    PostgresStoreProvider::Gemini( Arc::new( store ) )
                )
            )
        },
        StoreModel::Ollama( m ) =>
        {
            let model = ollama_store_model( &m )?;

            let store = provider_from_model( model, pool, documents, config ).await?;

            Ok(
                StoreProvider::Postgres(
                    PostgresStoreProvider::Ollama( Arc::new( store ) )
                )
            )
        },
        StoreModel::OpenAI( m ) =>
        {
            let model = openai_store_model( &m )?;

            let store = provider_from_model( model, pool, documents, config ).await?;

            Ok(
                StoreProvider::Postgres(
                    PostgresStoreProvider::OpenAI( Arc::new( store ) )
                )
            )
        }
    }
}

async fn provider_from_model<M: EmbeddingModel + Send + Sync>(
    model : M,
    pool : PgPool,
    documents : Vec<StoreDocument>,
    config : PostgresStoreProviderConfig
) -> Result<PostgresVectorStore<M>, Error>
{
    let store = vector_store( model.clone(), pool, config.table_name, PgVectorDistanceFunction::Cosine ).await;

    if documents.len() > 0
    {
        let documents = provider_embeddings( documents, model ).await?;

        store.insert_documents( documents ).await.map_err( | e | Error::Store( e.to_string() ) )?;
    }
    
    Ok( store )
}

async fn vector_store<M: EmbeddingModel + Send + Sync>(
    model : M,
    pool : PgPool,
    documents_table : Option<String>,
    distance_function : PgVectorDistanceFunction
) -> PostgresVectorStore<M>
{
    PostgresVectorStore::new(
        model, 
        pool, 
        documents_table, 
        distance_function
    )
}

async fn pg_pool( database_url : String ) -> Result<PgPool, Error>
{
    match pool( &database_url ).await
    {
        Some( p ) => Ok( p ),
        None => create_pool( database_url ).await
    }
}

fn database_url( config : &PostgresStoreProviderConfig ) -> Result<String, Error>
{
    match config.raw_database_url
    {
        true => Ok( config.database_url.clone() ),
        false =>
        {
            std::env::var(&config.database_url ).map_err( | e | Error::Store( format!( "database_url not set. {:?}", e ) ) )
        }
    }
}

fn pools() -> &'static Arc<Mutex<HashMap<String, PgPool>>>
{
    static P : OnceLock<Arc<Mutex<HashMap<String, PgPool>>>> = OnceLock::new();
    P.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

async fn pool( database_url : &str ) -> Option<PgPool>
{
    let lock = pools().lock().await;

    match lock.get( database_url )
    {
        Some( p ) => Some( p.clone() ),
        None => None    
    }
}

async fn create_pool( database_url : String ) -> Result<PgPool, Error>
{
    let pool = PgPoolOptions::new()
    .max_connections( 10 )
    .idle_timeout( std::time::Duration::from_secs( 5 ) )
    .connect( &database_url )
    .await
    .map_err( | e | Error::Store( e.to_string() ) )?;

    save_pool( database_url, pool.clone() ).await;

    Ok( pool )
}

async fn save_pool( key : String, pool : PgPool )
{
    let mut lock = pools().lock().await;

    lock.insert( key, pool );
}