


#[derive(Clone)]
pub struct AwpakTUIGraph
{
    pub initial_id : String,
    pub id : String,

    pub name : String,

    pub request : GraphRequest,

    pub response : Vec<String>
}

impl Default for AwpakTUIGraph
{
    fn default() -> Self 
    {
        Self 
        { 
            id : "".into(), 
            initial_id : "".into(),
            name : "".into(), 
            request : GraphRequest::Empty,
            response : vec![]
        }
    }
}

impl ToString for AwpakTUIGraph
{
    fn to_string( &self ) -> String
    {
        self.name.clone()
    }
}

impl AwpakTUIGraph
{
    pub fn prompt( &self ) -> Option<&String>
    {
        match &self.request
        {
            GraphRequest::Pending( p ) => Some( p ),
            _ => None
        }
    }

    pub fn own_prompt( mut self ) -> ( Self, Option<String> )
    {
        match self.request
        {
            GraphRequest::Pending( p ) =>
            {
                self.request = GraphRequest::Waiting;

                ( self, Some( p ) )
            },
            r =>
            {
                self.request = r;

                ( self, None )
            }
        }
    }

    pub fn change_request( mut self, request : GraphRequest ) -> Self
    {
        self.request = request;

        self
    }
}

#[derive(Clone)]
pub enum GraphRequest
{
    Pending( String ),
    Waiting,
    Empty
}