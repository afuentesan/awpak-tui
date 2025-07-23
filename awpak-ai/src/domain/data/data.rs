use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum DataType
{
    Null,
    Bool,
    Number,
    #[default]
    String,
    Array,
    Object
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DataFrom
{
    Context( FromContext ),
    ParsedInput( FromParsedInput ),
    Input { #[serde(default)] required : bool },
    Static( Value ),
    Concat( Vec<DataFrom> ),
    Operation( Box<DataOperation> ),
    AgentHistory( FromAgentHistory ),
    Null
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DataOperation
{
    Len( DataFrom ),
    Substract { num_1 : DataFrom, num_2 : DataFrom },
    Add { num_1 : DataFrom, num_2 : DataFrom },
    StringSplit { from : DataFrom, sep : String }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FromContext
{
    pub path : String,
    #[serde(default)]
    pub required : bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FromParsedInput
{
    #[serde(default)]
    pub path : Option<String>,
    #[serde(default)]
    pub required : bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FromAgentHistory
{
    pub id : String,
    pub content : FromAgentHistoryContent
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FromAgentHistoryContent
{
    Full,
    FullMessages,

    First,
    FirstMessage,

    Last,
    LastMessage,

    Range { from : usize, to : usize },
    RangeMessages { from : usize, to : usize },

    Item( usize ),
    ItemMessage( usize )
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DataToString
{
    pub from : DataFrom,
    #[serde(default)]
    pub prefix : Option<String>,
    #[serde(default)]
    pub suffix : Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataToContext
{
    pub path : String,
    #[serde(default)]
    pub ty : DataType,
    #[serde(default)]
    pub merge : DataMerge,
    #[serde(default)]
    pub optional : bool
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum DataMerge
{
    #[default]
    Insert,
    Append,
    AppendToArray
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DataComparator
{
    Eq { from_1 : DataFrom, from_2 : DataFrom },
    NotEq { from_1 : DataFrom, from_2 : DataFrom },
    Gt { from_1 : DataFrom, from_2 : DataFrom },
    Lt { from_1 : DataFrom, from_2 : DataFrom },

    Regex{ regex : String, from : DataFrom },

    And { comp_1 : Box<DataComparator>, comp_2 : Box<DataComparator> },
    Or { comp_1 : Box<DataComparator>, comp_2 : Box<DataComparator> },
    Not( Box<DataComparator> ),

    True,
    False
}