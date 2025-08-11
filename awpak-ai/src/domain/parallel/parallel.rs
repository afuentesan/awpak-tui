use serde::{Deserialize, Serialize};

use crate::domain::{command::command::Command, data::data::{DataComparator, DataType}, web_client::web_client::WebClient};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parallel
{
    pub executors : Vec<ParallelExecutor>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParallelExecutor
{
    Command { #[serde(default)] ty : DataType, executor : Command, condition : DataComparator },
    WebClient { #[serde(default)] ty : DataType, executor : WebClient, condition : DataComparator }
}

impl ParallelExecutor
{
    pub fn condition( &self ) -> &DataComparator
    {
        match self
        {
            ParallelExecutor::Command { ty : _, executor : _, condition } |
            ParallelExecutor::WebClient { ty : _, executor : _, condition } =>
            {
                condition
            }
        }
    }

    pub fn ty( &self ) -> DataType
    {
        match self
        {
            ParallelExecutor::Command { ty, executor : _, condition : _ } |
            ParallelExecutor::WebClient { ty, executor : _, condition : _ } =>
            {
                ty.clone()
            }
        }
    }
}