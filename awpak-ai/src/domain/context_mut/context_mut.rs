use serde::{Deserialize, Serialize};

use crate::domain::data::data::{DataFrom, DataToContext};


#[derive(Serialize, Deserialize, Clone)]
pub struct ContextMut
{
    pub from : DataFrom,
    pub to : DataToContext
}