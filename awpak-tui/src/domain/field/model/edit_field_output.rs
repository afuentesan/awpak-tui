use std::path::PathBuf;


pub enum EditFieldOutput
{
    ChangeFileName { cell_id : String, name : String, path : PathBuf },
    // Ignore
}

impl EditFieldOutput
{
    pub fn to_option_name( &self ) -> Option<String>
    {
        match self
        {
            Self::ChangeFileName { cell_id : _, name, path : _ } => Some( name.clone() ),
            // Self::Ignore => None
        }
    }

    pub fn to_option_path( &self ) -> Option<PathBuf>
    {
        match self
        {
            Self::ChangeFileName { cell_id : _, name : _, path } => Some( path.clone() ),
            // Self::Ignore => None
        }
    }

    pub fn to_option_id( &self ) -> Option<String>
    {
        match self
        {
            Self::ChangeFileName { cell_id, name : _, path : _ } => Some( cell_id.clone() ),
            // Self::Ignore => None
        }
    }
}