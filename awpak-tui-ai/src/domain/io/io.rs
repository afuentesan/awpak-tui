use serde::{Deserialize, Serialize};

use crate::domain::data::data::DataSelection;


#[derive(Serialize, Deserialize, Clone)]
pub struct AgentInput
{
    prompt : Option<DataSelection>,
    context : Option<DataSelection>
}