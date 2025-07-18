use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct AwpakResponse
{
    pub version : String,
    pub status : usize,
    pub headers : HashMap<String, String>,
    pub text : String,
    pub time_millis : u128,
    pub time_str : String
}