use crate::domain::data::data::InputData;


#[derive(Clone)]
pub struct RepeatClient
{
    pub id : String,
    pub provider : Box<RepeatClientProvider>,
    pub input : Vec<InputData>
}

#[derive(Clone)]
pub enum RepeatClientProvider
{
    Node( String ),
    Chain( String ),
    Repeat( String )
}