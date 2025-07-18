use rig::{message::{AssistantContent, Message, ToolResult, ToolResultContent, UserContent}, OneOrMany};
use serde_json::Value;

use crate::domain::{data::data::{FromAgentHistory, FromAgentHistoryContent}, error::Error, graph::graph::Graph};


pub fn data_from_agent_history(
    graph : &Graph,
    from_history : &FromAgentHistory
) -> Result<Value, Error>
{
    match graph.nodes.get( from_history.id.as_str() )
    {
        Some( n ) if n.executor.agent().is_some() => data_from_history( &n.executor.agent().unwrap().history, &from_history.content ),
        _ => Err( Error::Agent( format!( "Agent with id {} not found.", from_history.id ) ) )
    }
}

fn data_from_history(
    history : &Vec<Message>,
    from : &FromAgentHistoryContent
) -> Result<Value, Error>
{
    if history.len() == 0 { return Ok( Value::Null ) }

    let history = flat_history( history );

    match from
    {
        FromAgentHistoryContent::Full => serde_json::to_value( history ).map_err( | e | Error::ParseData( e.to_string() ) ),
        FromAgentHistoryContent::FullMessages => to_text_messages( history.iter().collect() ),

        FromAgentHistoryContent::First => serde_json::to_value( &history[ 0 ] ).map_err( | e | Error::ParseData( e.to_string() ) ),
        FromAgentHistoryContent::FirstMessage => to_text_messages( vec![ &history[ 0 ] ] ),

        FromAgentHistoryContent::Last =>
        {
            let last = history.len() - 1;

            serde_json::to_value( &history[ last ] ).map_err( | e | Error::ParseData( e.to_string() ) )
        },
        FromAgentHistoryContent::LastMessage =>
        {
            let last = history.len() - 1;

            to_text_messages( vec![ &history[ last ] ] )
        },

        FromAgentHistoryContent::Item( i ) =>
        {
            if *i >= history.len() { return Ok( Value::Null ) }

            serde_json::to_value( &history[ *i ] ).map_err( | e | Error::ParseData( e.to_string() ) )
        },
        FromAgentHistoryContent::ItemMessage( i ) =>
        {
            if *i >= history.len() { return Ok( Value::Null ) }

            to_text_messages( vec![ &history[ *i ] ] )
        },

        FromAgentHistoryContent::Range { from, to } =>
        {
            let to = usize::min( *to, history.len() );

            if *from >= history.len() || *from >= to { return Ok( Value::Null ) }

            serde_json::to_value( &history[ *from..to ] ).map_err( | e | Error::ParseData( e.to_string() ) )
        },
        FromAgentHistoryContent::RangeMessages { from, to } =>
        {
            let to = usize::min( *to, history.len() );

            if *from >= history.len() || *from >= to { return Ok( Value::Null ) }

            to_text_messages( history[ *from..to ].iter().collect() )
        }
    }
}

fn flat_history( history : &Vec<Message> ) -> Vec<Message>
{
    history.iter()
    .flat_map(
        | m | flat_message( m )
    )
    .collect()
}

fn flat_message( message : &Message ) -> Vec<Message>
{
    match message
    {
        Message::User { content } =>
        {
            flat_one_or_many_user_content( content )
        },
        Message::Assistant { id, content } =>
        {
            flat_one_or_many_assistant_content( id, content )
        }
    }
}

fn flat_one_or_many_user_content( content : &OneOrMany<UserContent> ) -> Vec<Message>
{
    content.iter().flat_map( 
        | c | 
        {
            match c
            {
                UserContent::ToolResult( r ) => flat_tool_result( r ),
                _ => vec![ Message::User { content : OneOrMany::one( c.clone() ) }  ]    
            }
            
        }
    ).collect()
}

fn flat_tool_result( result : &ToolResult ) -> Vec<Message>
{
    result.content.iter()
    .map( 
        | r |
        {
            Message::User 
            { 
                content : OneOrMany::one(
                    UserContent::ToolResult(
                        ToolResult 
                        { 
                            id : result.id.clone(), 
                            call_id : result.call_id.clone(), 
                            content : OneOrMany::one( r.clone() )
                        }
                    )
                )
            }
        }
    )
    .collect()
}

fn flat_one_or_many_assistant_content( id : &Option<String>, content : &OneOrMany<AssistantContent> ) -> Vec<Message>
{
    content.iter().map( | c | Message::Assistant { id : id.clone(), content : OneOrMany::one( c.clone() ) } ).collect()
}

fn to_text_messages(
    history : Vec<&Message>
) -> Result<Value, Error>
{
    serde_json::to_value( text_from_messages( history )? ).map_err( | e | Error::ParseData( e.to_string() ) )
}

fn text_from_messages(
    messages : Vec<&Message>
) -> Result<Vec<&String>, Error>
{
    let mut ret = vec![];

    for message in messages
    {
        ret.append(
            &mut match message
            {
                Message::User { content } => text_from_one_or_many_user_content( content )?,
                Message::Assistant { id : _, content } => text_from_one_or_many_assistant_content( content )?
            }
        );
    }

    Ok( ret )
}

fn text_from_one_or_many_assistant_content( content : &OneOrMany<AssistantContent> ) -> Result<Vec<&String>, Error>
{
    let mut ret = vec![];

    for c in content.iter()
    {
        ret.push( text_from_assistant_content( c )? );
    }

    Ok( ret )
}

fn text_from_assistant_content( content : &AssistantContent ) -> Result<&String, Error>
{
    match content
    {
        AssistantContent::Text( t ) => Ok( &t.text ),
        AssistantContent::ToolCall( t ) => Ok( &t.function.name )
    }
}

fn text_from_one_or_many_user_content( content : &OneOrMany<UserContent> ) -> Result<Vec<&String>, Error>
{
    let mut ret = vec![];

    for c in content.iter()
    {
        ret.append( &mut text_from_user_content( c )? );
    }

    Ok( ret )
}

fn text_from_user_content( user_content : &UserContent ) -> Result<Vec<&String>, Error>
{
    match user_content
    {
        UserContent::Text( t ) => Ok( vec![ &t.text ] ),
        UserContent::ToolResult( r ) => text_from_tool_result( &r.content ),
        u => Err( Error::ParseData( format!( "Invalid UserContent. {:?}", u ) ) )
    }
}

fn text_from_tool_result( content : &OneOrMany<ToolResultContent> ) -> Result<Vec<&String>, Error>
{
    let mut ret = vec![];

    for c in content.iter()
    {
        ret.push( text_from_tool_result_content( c )? );
    }

    Ok( ret )
}

fn text_from_tool_result_content( content : &ToolResultContent ) -> Result<&String, Error>
{
    match content
    {
        ToolResultContent::Text( t ) => Ok( &t.text ),
        c => Err( Error::ParseData( format!( "Invalid ToolResultContent. {:?}", c ) ) )
    }
}