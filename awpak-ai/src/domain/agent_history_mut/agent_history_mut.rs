use serde::{Deserialize, Serialize};

use crate::domain::data::data::{DataComparator, DataFrom};


#[derive(Serialize, Deserialize, Clone)]
pub struct AgentHistoryMut
{
    pub id : String,

    pub from : DataFrom,
    pub to : DataToAgentHistory,

    pub condition : DataComparator
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DataToAgentHistory
{
    Replace,
    ReplaceFirst,
    RelaceLast,
    ReplaceItem( usize ),
    StringToLast,
    StringToFirst,
    StringToItem( usize )
}

impl DataToAgentHistory
{
    pub fn is_single( &self ) -> bool
    {
        match self
        {
            DataToAgentHistory::Replace => false,
            _ => true    
        }
    }
}