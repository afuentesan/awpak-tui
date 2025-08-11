use serde::{Deserialize, Serialize};

use crate::domain::data::data::DataFrom;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command
{
    pub command : DataFrom,
    #[serde(default)]
    pub args : Vec<DataFrom>,
    #[serde(default)]
    pub output : Vec<CommandOutput>,
    #[serde(default)]
    pub timeout : Option<u64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandOutput
{
    Out { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Err { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },

    Success { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Code { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },

    Object { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
}

impl Default for CommandOutput
{
    fn default() -> Self
    {
        CommandOutput::Out { prefix : None, suffix : None }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandResult
{
    #[serde(default)]
    pub out : Option<String>,
    #[serde(default)]
    pub err : Option<String>,
    
    #[serde(default)]
    pub success : bool,
    #[serde(default)]
    pub code : Option<i32>
}

impl ToString for CommandResult
{
    fn to_string( &self ) -> String 
    {
        format!(
            "Success: {}\nCode: {:?}\nOutput:\n{:?}\nError:\n{:?}",
            self.success,
            self.code,
            self.out,
            self.err
        )
    }
}