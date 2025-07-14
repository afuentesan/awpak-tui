use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Clone)]
pub struct AwpakRequest
{
    pub url : String,
    pub method : AwpakMethod,
    #[serde(default)]
    pub headers : Vec<AwpakHeader>,
    #[serde(default)]
    pub query_params : Vec<AwpakQueryParam>,
    #[serde(default)]
    pub body : Option<AwpakBody>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AwpakMethod
{
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch
}

impl From<AwpakMethod> for Method
{
    fn from( value: AwpakMethod ) -> Self 
    {
        match value
        {
            AwpakMethod::Options => Method::OPTIONS,
            AwpakMethod::Get => Method::GET,
            AwpakMethod::Post => Method::POST,
            AwpakMethod::Put => Method::PUT,
            AwpakMethod::Delete => Method::DELETE,
            AwpakMethod::Head => Method::HEAD,
            AwpakMethod::Trace => Method::TRACE,
            AwpakMethod::Connect => Method::CONNECT,
            AwpakMethod::Patch => Method::PATCH
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AwpakQueryParam
{
    pub name : String,
    pub value : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AwpakHeader
{
    pub name : String,
    pub value : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AwpakBody
{
    Json( Value ),
    Form( Vec<AwpakFormField> )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AwpakFormField
{
    pub name : String,
    pub value : String
}