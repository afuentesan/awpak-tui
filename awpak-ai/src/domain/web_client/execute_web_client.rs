use awpak_web_client::{request::{AwpakBody, AwpakFormField, AwpakHeader, AwpakQueryParam, AwpakRequest}, response::AwpakResponse, send_request};
use tracing::info;

use crate::domain::{data::{data_selection::data_selection, data_utils::value_to_string}, error::Error, graph::graph::Graph, tracing::filter_layer::{WEB_CLIENT_REQUEST, WEB_CLIENT_REQUEST_BODY, WEB_CLIENT_REQUEST_HEADERS, WEB_CLIENT_REQUEST_QUERY_PARAMS, WEB_CLIENT_RESPONSE, WEB_CLIENT_RESPONSE_BODY, WEB_CLIENT_RESPONSE_HEADERS}, utils::string_utils::{option_string_to_str, prefix_str_suffix}, web_client::web_client::{WebClient, WebClientBody, WebClientNameValue, WebClientOutput}};


pub async fn execute_web_client(
    graph : &Graph,
    client : &WebClient
) -> Result<String, Error>
{
    let id = graph.id.as_ref();

    let request = request( graph, client )?;

    let response = send_request( request ).await.map_err( | e | Error::WebClient( e.to_string() ) )?;

    output(
        id, 
        &client.output, 
        response
    )
}

fn output(
    id : Option<&String>,
    output : &Vec<WebClientOutput>,
    response : AwpakResponse,
) -> Result<String, Error>
{
    let mut ret = String::new();

    for out in output
    {
        ret.push_str( item_output( out, &response )?.as_str() );
    }

    trace_web_client_response( id, response );
    
    Ok( ret )
}

fn trace_web_client_response(
    graph_id : Option<&String>,
    response : AwpakResponse
)
{
    info!(
        target:WEB_CLIENT_RESPONSE, 
        id=option_string_to_str( graph_id ), 
        text=format!(
            "Version: {}\nStatus: {:?}",
            response.version,
            response.status
        )
    );

    info!(
        target:WEB_CLIENT_RESPONSE_HEADERS, 
        id=option_string_to_str( graph_id ), 
        text=format!( "{:?}", response.headers )
    );

    info!(
        target:WEB_CLIENT_RESPONSE_BODY, 
        id=option_string_to_str( graph_id ), 
        text=response.text
    );
}

fn item_output(
    output : &WebClientOutput,
    response : &AwpakResponse
) -> Result<String, Error>
{
    match output
    {
        WebClientOutput::Version { prefix, suffix } =>
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), response.version.as_str() ) )
        },
        WebClientOutput::Status { prefix, suffix } =>
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), response.status.to_string().as_str() ) )
        },
        WebClientOutput::Header { name, prefix, suffix } =>
        {
            Ok( 
                prefix_str_suffix( 
                    prefix.as_ref(), 
                    suffix.as_ref(), 
                    response.headers.get( name ).unwrap_or( &"".to_string() )
                ) 
            )
        },
        WebClientOutput::Body { prefix, suffix } =>
        {
            Ok( prefix_str_suffix( prefix.as_ref(), suffix.as_ref(), &response.text ) )
        },
        WebClientOutput::Object { prefix, suffix } =>
        {
            Ok( 
                prefix_str_suffix( 
                    prefix.as_ref(), 
                    suffix.as_ref(), 
                    &serde_json::to_string( response ).map_err( | e | Error::ParseData( e.to_string() ) )?
                ) 
            )
        }
    }
}

fn request( 
    graph : &Graph,
    client : &WebClient
) -> Result<AwpakRequest, Error>
{
    let id = graph.id.as_ref();

    let url = value_to_string( &data_selection( graph, &client.url )? );
    let method = client.method.clone();
    let headers = request_headers( graph, &client.headers )?;
    let query_params = request_query_params( graph, &client.query_params )?;
    let body = body( graph, client.body.as_ref() )?;

    let request = AwpakRequest 
    { 
        url, 
        method, 
        headers, 
        query_params, 
        body
    };

    trace_request( id, &request );

    Ok( request )
}

fn trace_request(
    graph_id : Option<&String>,
    request : &AwpakRequest
)
{
    info!(
        target:WEB_CLIENT_REQUEST, 
        id=option_string_to_str( graph_id ), 
        text=format!(
            "URL: {}\nMethod: {:?}",
            request.url,
            request.method
        )
    );

    info!(
        target:WEB_CLIENT_REQUEST_BODY, 
        id=option_string_to_str( graph_id ), 
        text=format!( "{:?}", request.body )
    );

    info!(
        target:WEB_CLIENT_REQUEST_HEADERS, 
        id=option_string_to_str( graph_id ), 
        text=format!( "{:?}", request.headers )
    );

    info!(
        target:WEB_CLIENT_REQUEST_QUERY_PARAMS, 
        id=option_string_to_str( graph_id ), 
        text=format!( "{:?}", request.query_params )
    );
}

fn body(
    graph : &Graph,
    body : Option<&WebClientBody>
) -> Result<Option<AwpakBody>, Error>
{
    match body
    {
        Some( b ) =>
        {
            match b
            {
                WebClientBody::Json( j ) =>
                {
                    Ok(
                        Some(
                            AwpakBody::Json(
                                data_selection( graph, j )?
                            )
                        )
                    )
                },
                WebClientBody::Form( f ) =>
                {
                    let mut fields = vec![];

                    for field in f
                    {
                        fields.push(
                            AwpakFormField
                            {
                                name : value_to_string( &data_selection( graph, &field.name )? ),
                                value : value_to_string( &data_selection( graph, &field.value )? )
                            }
                        );
                    }

                    Ok(
                        Some(
                            AwpakBody::Form( fields )
                        )
                    )
                }
            }
        },
        _ => Ok( None )    
    }
}

fn request_headers(
    graph : &Graph,
    headers : &Vec<WebClientNameValue>
) -> Result<Vec<AwpakHeader>, Error>
{
    let mut ret = vec![];

    for h in headers
    {
        ret.push(
            AwpakHeader 
            { 
                name : value_to_string( &data_selection( graph, &h.name )? ), 
                value : value_to_string( &data_selection( graph, &h.value )? )
            }
        );
    }

    Ok( ret )
}

fn request_query_params(
    graph : &Graph,
    query_params : &Vec<WebClientNameValue>
) -> Result<Vec<AwpakQueryParam>, Error>
{
    let mut ret = vec![];

    for q in query_params
    {
        ret.push(
            AwpakQueryParam 
            { 
                name : value_to_string( &data_selection( graph, &q.name )? ), 
                value : value_to_string( &data_selection( graph, &q.value )? )
            }
        );
    }

    Ok( ret )
}