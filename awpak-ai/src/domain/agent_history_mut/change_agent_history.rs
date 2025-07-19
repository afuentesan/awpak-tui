use awpak_utils::result::result::AwpakResult;
use rig::message::{AssistantContent, Message, ToolResultContent, UserContent};
use serde_json::Value;

use crate::domain::{agent_history_mut::agent_history_mut::{AgentHistoryMut, DataToAgentHistory}, data::{data_compare::compare_data, data_history::flat_history, data_selection::data_selection, data_utils::value_to_string}, error::Error, graph::{graph::Graph, node::NodeExecutor}};


pub async fn change_agent_history( 
    history_mut : &Vec<AgentHistoryMut>, 
    mut graph : Graph 
) -> AwpakResult<Graph, Error>
{
    for h in history_mut
    {
        match compare_data( 
            &graph,
            &h.condition 
        )
        {
            Ok( r ) if r =>
            {
                graph = match change_item_history( h, graph ).await.collect()
                {
                    ( g, None ) => g,
                    ( g, Some( e ) ) => return AwpakResult::new_err( g, e )    
                };
            },
            Ok( _ ) => continue,
            Err( e ) => return AwpakResult::new_err( graph, e )
        }
    }
    
    AwpakResult::new( graph )
}

async fn change_item_history( history_mut : &AgentHistoryMut, mut graph : Graph ) -> AwpakResult<Graph, Error>
{
    let data = match data_selection( &graph, &history_mut.from )
    {
        Ok( v ) => v,
        Err( e ) => return AwpakResult::new_err( graph, e )
    };

    let history = match find_history( &mut graph, &history_mut.id )
    {
        Some( h ) => h,
        None => return AwpakResult::new_err( graph, Error::Agent( format!( "Agent {} not found", history_mut.id ) ) )
    };

    if history.len() == 0 && history_mut.to.is_single() { return AwpakResult::new( graph ); }

    if history_mut.to.is_single()
    {
        let mut history_flatten = flat_history( history );

        std::mem::swap( history, &mut history_flatten );
    }

    match history_mut.to
    {
        DataToAgentHistory::Replace =>
        {
            match value_to_vec_of_messages( data )
            {
                Ok( mut m ) =>
                {
                    std::mem::swap( history, &mut m );

                    AwpakResult::new( graph )
                },
                Err( e ) => AwpakResult::new_err( graph, e )
            }
        },
        DataToAgentHistory::ReplaceFirst =>
        {
            match value_to_message( data )
            {
                Ok( m ) =>
                {
                    history[ 0 ] = m;

                    AwpakResult::new( graph )
                },
                Err( e ) => AwpakResult::new_err( graph, e )
            }
        },
        DataToAgentHistory::ReplaceLast =>
        {
            match value_to_message( data )
            {
                Ok( m ) =>
                {
                    let last = history.len() - 1;

                    history[ last ] = m;

                    AwpakResult::new( graph )
                },
                Err( e ) => AwpakResult::new_err( graph, e )
            }
        },
        DataToAgentHistory::ReplaceItem( i ) =>
        {
            if i >= history.len() { return AwpakResult::new( graph ); }

            match value_to_message( data )
            {
                Ok( m ) =>
                {
                    history[ i ] = m;

                    AwpakResult::new( graph )
                },
                Err( e ) => AwpakResult::new_err( graph, e )
            }
        },
        DataToAgentHistory::StringToLast =>
        {
            let last = history.len() - 1;

            value_string_to_message( data, &mut history[ last ] );

            AwpakResult::new( graph )
        },
        DataToAgentHistory::StringToFirst =>
        {
            value_string_to_message( data, &mut history[ 0 ] );

            AwpakResult::new( graph )
        },
        DataToAgentHistory::StringToItem( i ) =>
        {
            if i >= history.len() { return AwpakResult::new( graph ); }

            value_string_to_message( data, &mut history[ i ] );

            AwpakResult::new( graph )
        }
    }
}

fn value_string_to_message( value : Value, message : &mut Message )
{
    match message
    {
        Message::User { content } =>
        {
            // OneOrMany siempre contiene al menos un elemento así que se puede hacer el unwrap
            let first = content.iter_mut().next().unwrap();

            match first
            {
                UserContent::Text( t ) =>
                {
                    t.text = value_to_string( &value );
                },
                UserContent::ToolResult( t ) =>
                {
                    let content = t.content.iter_mut().next().unwrap();

                    match content
                    {
                        ToolResultContent::Text( t ) =>
                        {
                            t.text = value_to_string( &value );
                        },
                        ToolResultContent::Image( _ ) => {}
                    }
                },
                _ => {}
            }
        },
        Message::Assistant { id : _, content } =>
        {
            let first = content.iter_mut().next().unwrap();

            match first
            {
                AssistantContent::Text( t ) =>
                {
                    t.text = value_to_string( &value );
                },
                _ => {}
            }
        }
    }
}

fn value_to_vec_of_messages( mut value : Value ) -> Result<Vec<Message>, Error>
{
    match value
    {
        Value::Array( a ) =>
        {
            value = Value::Array( a );

            serde_json::from_value( value ).map_err( | e | Error::ParseData( e.to_string() ) )
        },
        Value::String( s ) =>
        {
            match serde_json::from_str::<Vec<Message>>( s.as_str() )
            {
                Ok( v ) => Ok( v ),
                Err( _ ) =>
                {
                    Ok( vec![ value_to_message( Value::String( s ) )? ] )
                }
            }
        }
        value =>
        {
            Ok( vec![ value_to_message( value )? ] )
        }
    }
}

fn value_to_message( value : Value ) -> Result<Message, Error>
{
    match value
    {
        Value::String( s ) =>
        {
            serde_json::from_str( s.as_str() ).map_err( | e | Error::ParseData( e.to_string() ) )
        },
        value =>
        {
            serde_json::from_value( value ).map_err( | e | Error::ParseData( e.to_string() ) )
        }
    }
}

fn find_history<'a, 'b>( graph : &'a mut Graph, id : &'b str ) -> Option<&'a mut Vec<Message>>
{
    match graph.nodes.values_mut().find( | n | n.id == id )
    {
        Some( n ) =>
        {
            match &mut n.executor
            {
                NodeExecutor::Agent( a ) => Some( &mut a.history ),
                _ => None
            }
        },
        None => None
    }
}