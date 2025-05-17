use crate::domain::{directory::model::directory::Directory, executable_expandable::model::executable_expandable::ExecutableExpandable, expandable::model::expandable::Expandable};


#[derive(Clone)]
pub enum ContentGenerator
{
    Directory( Directory ),
    Expandable( Expandable ),
    ExecutableExpandable( ExecutableExpandable ),
    Detail( Box<ContentGenerator>, String ),
    Chat( Box<ContentGenerator>, String ),
    Empty
}

impl ContentGenerator
{
    pub fn own_without_detail( self ) -> Self
    {
        match self
        {
            Self::Detail( g, _ ) => *g,
            _ => self    
        }
    }

    pub fn try_eq( &self, other : Option<&Self> ) -> bool
    {
        if other.is_none() || ! self.eq( other.unwrap() )
        {
            return false
        }

        true
    }

    // pub fn self_detail_eq( &self, other : &Self ) -> bool
    // {
    //     match ( self, other )
    //     {
    //         ( ContentGenerator::Detail( c, _ ), o ) => c.eq( o ),
    //         _ => false
    //     }
    // }

    pub fn eq( &self, other : &Self ) -> bool
    {
        // TODO: Crear método para comparar los paths y tener en cuenta que en windows no es case sensitive
        match ( self, other )
        {
            ( 
                ContentGenerator::Directory( d1 ), 
                ContentGenerator::Directory( d2 ) 
            ) if d1.to_string() == d2.to_string() => true,
            ( 
                ContentGenerator::Expandable( e1 ), 
                ContentGenerator::Expandable( e2 ) 
            ) if e1.to_string() == e2.to_string() => true,
            ( 
                ContentGenerator::ExecutableExpandable( e1 ), 
                ContentGenerator::ExecutableExpandable( e2 ) 
            ) if e1.to_string() == e2.to_string() => true,
            ( 
                ContentGenerator::Detail( e1, s1 ), 
                ContentGenerator::Detail( e2, s2 ) 
            ) if e1.to_string() == e2.to_string() && s1 == s2 => true,
            ( 
                ContentGenerator::Chat( e1, s1 ), 
                ContentGenerator::Chat( e2, s2 ) 
            ) if e1.to_string() == e2.to_string() && s1 == s2 => true,
            ( ContentGenerator::Empty, ContentGenerator::Empty ) => true,
            (
                ContentGenerator::Directory( _ ) | 
                ContentGenerator::Expandable( _ ) | 
                ContentGenerator::ExecutableExpandable( _ ) |
                ContentGenerator::Detail( _, _ ) |
                ContentGenerator::Chat( _, _ ) |
                ContentGenerator::Empty,

                ContentGenerator::Directory( _ ) | 
                ContentGenerator::Expandable( _ ) | 
                ContentGenerator::ExecutableExpandable( _ ) | 
                ContentGenerator::Detail( _, _ ) |
                ContentGenerator::Chat( _, _ ) |
                ContentGenerator::Empty
            ) => false
        }
    }

    pub fn is_empty( &self ) -> bool
    {
        match self
        {
            ContentGenerator::Empty => true,
            _ => false    
        }
    }
}

impl ToString for ContentGenerator
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            ContentGenerator::Directory( d ) => d.to_string(),
            ContentGenerator::Expandable( e ) => e.to_string(),
            ContentGenerator::ExecutableExpandable( e ) => e.to_string(),
            ContentGenerator::Detail( d, s ) |
            ContentGenerator::Chat( d, s ) => format!( "{}#{}", d.to_string(), s ),
            ContentGenerator::Empty => "Empty".to_string()
        }
    }
}