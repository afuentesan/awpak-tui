use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};

use crate::domain::{error::Error, repeat::{repeat::Repeat, repeat_client::RepeatClient, repeat_functions::create_repeat_client}};

// MANAGE REPEAT CLIENTS

pub async fn repeat_client( id : &str, repeat : &Repeat ) -> Result<RepeatClient, Error>
{
    match repeat_existing_client( id )
    {
        Ok( c ) => Ok( c ),
        _ => 
        {
            let client = create_repeat_client( id, repeat ).await?;

            repeat_clients().lock().unwrap().insert( id.to_string(), client.clone() );

            Ok( client )
        }
    }
}

fn repeat_clients() -> &'static Arc<Mutex<HashMap<String, RepeatClient>>>
{
    static REPEAT_CLIENTS : OnceLock<Arc<Mutex<HashMap<String, RepeatClient>>>> = OnceLock::new();
    REPEAT_CLIENTS.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

fn repeat_existing_client( id : &str ) -> Result<RepeatClient, Error>
{
    match repeat_clients().as_ref().lock().unwrap().get( id )
    {
        Some( c ) => Ok( c.clone() ),
        None => Err( Error::Ignore )
    }
}

// END MANAGE REPEAT CLIENTS