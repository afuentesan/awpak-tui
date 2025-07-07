use serde::{Deserialize, Serialize};

use crate::domain::data::data::DataFrom;


#[derive(Serialize, Deserialize, Clone)]
pub struct Command
{
    pub command : String,
    #[serde(default)]
    pub args : Vec<DataFrom>,
    #[serde(default)]
    pub output : Vec<CommandOutput>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CommandOutput
{
    Out { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Err { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },

    Success { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
    Code { #[serde(default)] prefix : Option<String>, #[serde(default)] suffix : Option<String> },
}

impl Default for CommandOutput
{
    fn default() -> Self
    {
        CommandOutput::Out { prefix : None, suffix : None }
    }
}

pub struct CommandResult
{
    pub out : Option<String>,
    pub err : Option<String>,
    
    pub success : bool,
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