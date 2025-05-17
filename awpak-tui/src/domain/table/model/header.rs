
#[derive(Clone)]
pub struct HeaderData
{
    pub id : String,
    pub name : String
}

impl HeaderData
{
    pub fn new( id : &str, name : &str ) -> Self
    {
        Self { id : id.to_string(), name : name.to_string() }
    }
}

#[derive(Clone)]
pub enum Header
{
    Visible( HeaderData ),
    Hidden( HeaderData )
}

impl Header
{
    pub fn visible( &self ) -> bool
    {
        match self
        {
            Header::Visible( _ ) => true,
            Header::Hidden( _ ) => false    
        }
    }

    // pub fn as_str( &self ) -> &str
    // {
    //     match self
    //     {
    //         Header::Visible( s ) |
    //         Header::Hidden( s ) => s.name.as_str(),
    //     }
    // }

    pub fn as_id( &self ) -> &str
    {
        match self
        {
            Header::Visible( s ) |
            Header::Hidden( s ) => s.id.as_str(),
        }
    }
}

impl ToString for Header
{
    fn to_string( &self ) -> String
    {
        match self
        {
            Header::Visible( s ) |
            Header::Hidden( s ) => s.name.clone()    
        }
    }
}