use awpak_web_client::request::AwpakMethod;
use serde::{Deserialize, Serialize};

use crate::domain::data::data::DataFrom;

#[derive(Serialize, Deserialize, Clone)]
pub struct WebClient
{
    pub url : DataFrom,
    pub method : AwpakMethod,
    #[serde(default)]
    pub headers : Vec<WebClientNameValue>,
    #[serde(default)]
    pub query_params : Vec<WebClientNameValue>,
    #[serde(default)]
    pub body : Option<WebClientBody>,
    #[serde(default)]
    pub output : Vec<WebClientOutput>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WebClientOutput
{
    Version { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Status { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },

    Header { name : String, #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Body { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },

    Object { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebClientNameValue
{
    pub name : DataFrom,
    pub value : DataFrom
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WebClientBody
{
    Json( DataFrom ),
    Form( Vec<WebClientNameValue> )
}