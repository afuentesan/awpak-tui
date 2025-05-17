
#[derive(Clone)]
pub enum Message
{
    Info( String ),
    Warning( String ),
    Error( String )
}

impl Message
{
    pub fn as_str( &self ) -> &str
    {
        match self
        {
            Self::Info( s ) |
            Self::Warning( s ) |
            Self::Error( s ) => s
        }
    }
}

impl ToString for Message
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            Self::Info( s ) |
            Self::Warning( s ) |
            Self::Error( s ) => s.clone()
        }
    }
}