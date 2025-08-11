use serde::{Deserialize, Serialize};

use crate::domain::data::data::{DataComparator, DataFrom, DataToContext};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextMut
{
    pub from : DataFrom,
    pub to : DataToContext,
    pub condition : DataComparator
}