
pub mod request;
pub mod response;
use std::collections::HashMap;

use reqwest::RequestBuilder;

use crate::{request::{AwpakBody, AwpakHeader, AwpakMethod, AwpakQueryParam, AwpakRequest}, response::AwpakResponse};

pub async fn send_request( request : AwpakRequest ) -> Result<AwpakResponse, reqwest::Error>
{
    let builder = request_builder( &request.url, request.method );

    let builder = append_headers(builder, request.headers );

    let builder = append_query_params( builder, request.query_params );

    let builder = match request.body
    {
        Some( b ) => append_body( builder, b ),
        _ => builder
    };

    send( builder ).await
}

async fn send( builder : RequestBuilder ) -> Result<AwpakResponse, reqwest::Error>
{
    let response = builder.send().await?;

    let version =  format!( "{:?}", response.version() );

    let status = usize::from( response.status().as_u16() );

    let headers = response.headers().into_iter()
    .fold( 
        HashMap::new(), 
        | mut a,  ( n, v ) | 
        {
            a.insert( n.to_string(), v.to_str().unwrap_or( "" ).to_string() );

            a
        }
    );

    let text = response.text().await?;

    Ok(
        AwpakResponse
        {
            version,
            status,
            headers,
            text
        }
    )
}

fn append_query_params( builder : RequestBuilder, query_params : Vec<AwpakQueryParam> ) -> RequestBuilder
{
    if query_params.len() == 0 { return builder }

    builder.query( 
        query_params.into_iter()
        .map( | q | ( q.name, q.value ) )
        .collect::<Vec<_>>()
        .as_slice()
    )
}

fn append_headers( mut builder : RequestBuilder, headers : Vec<AwpakHeader> ) -> RequestBuilder
{
    if headers.len() == 0 { return builder; }

    for h in headers
    {
        builder = builder.header( h.name, h.value )
    }
    
    builder
}

fn append_body( builder : RequestBuilder, body : AwpakBody ) -> RequestBuilder
{
    match body
    {
        AwpakBody::Json( j ) => builder.json( &j ),
        AwpakBody::Form( f ) =>
        {
            if f.len() == 0
            {
                builder
            }
            else
            {
                builder.form(
                    f.into_iter()
                    .map( | q | ( q.name, q.value ) )
                    .collect::<Vec<_>>()
                    .as_slice()
                )    
            }
        }
    }
}

fn request_builder( url : &str, method : AwpakMethod ) -> RequestBuilder
{
    let client = reqwest::Client::new();

    client.request( method.into(), url )
}