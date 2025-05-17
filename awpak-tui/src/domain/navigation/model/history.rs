use crate::domain::content_generator::model::content_generator::ContentGenerator;


#[derive(Clone)]
pub struct History
{
    pub next : Vec<ContentGenerator>,
    pub back : Vec<ContentGenerator>
}

impl History
{
    pub fn new() -> Self
    {
        Self { next : vec![], back: vec![] }
    }
}
