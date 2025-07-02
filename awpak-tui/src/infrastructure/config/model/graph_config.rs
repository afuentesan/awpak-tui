use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AwpakTUIGraphConfig
{
    #[serde(default)]
    pub id : String,

    pub name : String,
    pub path : String
}