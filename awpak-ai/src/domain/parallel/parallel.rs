use serde::{Deserialize, Serialize};

use crate::domain::{command::command::Command, data::data::DataType, web_client::web_client::WebClient};

#[derive(Serialize, Deserialize, Clone)]
pub struct Parallel
{
    pub executors : Vec<ParallelExecutor>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ParallelExecutor
{
    Command { #[serde(default)] ty : DataType, executor : Command },
    WebClient { #[serde(default)] ty : DataType, executor : WebClient }
}