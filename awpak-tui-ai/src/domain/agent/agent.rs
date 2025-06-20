use serde::{Deserialize, Serialize};

use crate::domain::{chain::chain::Chain, command::command::Command, node::node::Node, repeat::repeat::Repeat};


#[derive(Serialize, Deserialize, Clone)]
pub enum AIAgent
{
    Node( Node ),
    Chain( Chain ),
    Repeat( Repeat ),
    Command( Command )
}

impl AIAgent
{
    pub fn own_node( self ) -> ( Self, Option<Node> )
    {
        match self
        {
            AIAgent::Node( n ) =>
            {
                ( AIAgent::Node( Node::default() ), Some( n ) )
            },
            _ => ( self, None )
        }
    }

    pub fn own_chain( self ) -> ( Self, Option<Chain> )
    {
        match self
        {
            AIAgent::Chain( c ) =>
            {
                ( AIAgent::Chain( Chain::default() ), Some( c ) )
            },
            _ => ( self, None )
        }
    }

    pub fn own_repeat( self ) -> ( Self, Option<Repeat> )
    {
        match self
        {
            AIAgent::Repeat( r ) =>
            {
                ( AIAgent::Repeat( Repeat::default() ), Some( r ) )
            },
            _ => ( self, None )
        }
    }

    pub fn own_command( self ) -> ( Self, Option<Command> )
    {
        match self
        {
            AIAgent::Command( c ) =>
            {
                ( AIAgent::Command( Command::default() ), Some( c ) )
            },
            _ => ( self, None )
        }
    }
}

impl Default for AIAgent
{
    fn default() -> Self 
    {
        AIAgent::Node( Node::default() )
    }
}

impl ToString for AIAgent
{
    fn to_string( &self ) -> String 
    {
        match self
        {
            AIAgent::Node( n ) => n.name.clone(),
            AIAgent::Chain( c ) => c.name.clone(),
            AIAgent::Repeat( r ) => r.name.clone(),
            AIAgent::Command( c ) => c.name.clone()
        }
    }
}