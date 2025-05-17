use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct JSONContent
{
    #[serde(default)]
    pub table : Option<JSONContentTable>
}

#[derive(Serialize, Deserialize)]
pub struct JSONContentTable
{
    pub headers : Vec<JSONContentHeader>,
    #[serde(default)]
    pub rows : Vec<JSONContentRow>
}

#[derive(Serialize, Deserialize)]
pub struct JSONContentHeader
{
    pub id : Option<String>,
    pub text : String,
    #[serde(default = "default_visibility")]
    pub visible : bool
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JSONRowType
{
    Directory,
    File,
    Expandable,
    ExecutableExpandable,
    Executable,
    ReadOnly
}

#[derive(Serialize, Deserialize)]
pub struct JSONContentRow
{
    pub path : String,
    pub params : Option<String>,
    pub row_type : JSONRowType,
    pub cells : Vec<JSONContentCell>
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum JSONContentCellAccess
{
    Read,
    Write
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum JSONContentCellType
{
    Date,
    String,
    Empty
}

#[derive(Serialize, Deserialize)]
pub struct JSONContentCell
{
    pub text : String,
    #[serde(default = "default_access")]
    pub access : JSONContentCellAccess,
    #[serde(default = "default_type")]
    pub ty : JSONContentCellType
}

fn default_visibility() -> bool
{
    true
}

fn default_access() -> JSONContentCellAccess
{
    JSONContentCellAccess::Read
}

fn default_type() -> JSONContentCellType
{
    JSONContentCellType::String
}