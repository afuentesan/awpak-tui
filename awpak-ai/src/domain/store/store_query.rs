use rig::{embeddings::EmbeddingModel, vector_store::{in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore}, VectorSearchRequest, VectorStoreIndex}};
use serde_json::Value;

use crate::domain::{data::{data::FromStore, data_selection::data_selection, data_utils::value_to_string}, error::Error, graph::graph::Graph, store::{postgres_store::query_postgres_store, store::{EmbeddingDocument, Store, StoreModel, StoreProvider}, store_from_config::{gemini_store_model, ollama_store_model, openai_store_model}}};


pub async fn store_query_from_graph_store(
    graph : &Graph,
    from_store : &FromStore
) -> Result<Value, Error>
{
    let query = value_to_string( &data_selection( graph, &from_store.query ).await? );

    match graph.stores.get( &from_store.id )
    {
        Some( s ) =>
        {
            Ok( Value::String( store_query_as_str( s, &query, from_store.samples ).await? ) )
        },
        None => Err( Error::Store( format!( "Store {} not found", from_store.id ) ) )    
    }
}

pub async fn store_query_as_str(
    store : &Store,
    query : &str,
    samples : u64
) -> Result<String, Error>
{
    let result = store_query( store, query, samples ).await?;

    Ok(
        result.into_iter().fold(
            "".to_string(), 
            | a, ( _, _, d ) |
            {
                format!( "{}{}\n\n", a, d.content )
            }
        )
    )
}

pub async fn store_query(
    store : &Store,
    query : &str,
    samples : u64
) -> Result<Vec<( f64, String, EmbeddingDocument )>, Error>
{
    let mut lock = store.provider.lock().await;

    let provider = lock.take().ok_or( Error::Store( "Store is None".into() ) )?;

    match provider
    {
        StoreProvider::InMemoryVectorStore( s ) =>
        {
            let ( s, result ) = query_in_memory_vector_store( s, query, samples, &store.model ).await;

            let _ = lock.insert( StoreProvider::InMemoryVectorStore( s ) );

            drop( lock );

            result
        },
        StoreProvider::Postgres( s ) =>
        {
            match query_postgres_store( &s, query, samples ).await
            {
                Ok( r ) =>
                {
                    let _ = lock.insert( StoreProvider::Postgres( s ) );

                    drop( lock );

                    Ok( r )
                },
                Err( e ) =>
                {
                    let _ = lock.insert( StoreProvider::Postgres( s ) );

                    drop( lock );

                    Err( e )
                }
            }
        }
    }
}

async fn query_in_memory_vector_store( 
    store : InMemoryVectorStore<EmbeddingDocument>,
    query : &str,
    samples : u64,
    model : &StoreModel
) -> ( InMemoryVectorStore<EmbeddingDocument>, Result<Vec<( f64, String, EmbeddingDocument )>, Error> )
{
    match model
    {
        StoreModel::OpenAI( m ) =>
        {
            match openai_store_model( m )
            {
                Ok( m ) => query_in_memory_vector_store_with_model( store, query, samples, m ).await,
                Err( e ) => ( store, Err( e ) )
            }
        },
        StoreModel::Gemini( m ) =>
        {
            match gemini_store_model( m )
            {
                Ok( m ) => query_in_memory_vector_store_with_model( store, query, samples, m ).await,
                Err( e ) => ( store, Err( e ) )
            }
        },
        StoreModel::Ollama( m ) =>
        {
            match ollama_store_model( m )
            {
                Ok( m ) => query_in_memory_vector_store_with_model( store, query, samples, m ).await,
                Err( e ) => ( store, Err( e ) )
            }
        }
    }
    
}

async fn query_in_memory_vector_store_with_model<M: EmbeddingModel>(
    store : InMemoryVectorStore<EmbeddingDocument>,
    query : &str,
    samples : u64,
    model : M
) -> ( InMemoryVectorStore<EmbeddingDocument>, Result<Vec<( f64, String, EmbeddingDocument )>, Error> )
{
    let index = InMemoryVectorIndex::new( model, store );

    match VectorSearchRequest::builder().query( query ).samples( samples ).build()
    {
        Ok( r ) =>
        {
            match index.top_n::<EmbeddingDocument>( r ).await
            {
                Ok( r ) => ( index.store, Ok( r ) ),
                Err( e ) => ( index.store, Err( Error::Store( e.to_string() ) ) )
            }
        },
        Err( e ) => ( index.store, Err( Error::Store( e.to_string() ) ) )
    }
}