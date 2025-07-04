use std::{collections::HashMap, sync::{Arc, Mutex, OnceLock}};



fn graphs_cancel_state() -> &'static Arc<Mutex<HashMap<String, bool>>>
{
    static I : OnceLock<Arc<Mutex<HashMap<String, bool>>>> = OnceLock::new();
    I.get_or_init(|| Arc::new( Mutex::new( HashMap::new() ) ) )
}

pub fn init_cancel_state( id : String )
{
    graphs_cancel_state().lock().unwrap().insert( id, false );
}

pub fn cancel_graph( id : &str )
{
    let mut lock = graphs_cancel_state().lock().unwrap();

    lock.insert( id.to_string(), true );
}

pub fn is_graph_cancelled( id : &str ) -> bool
{
    let lock = graphs_cancel_state().lock().unwrap();

    match lock.get( id )
    {
        Some( b ) => *b,
        _ => false    
    }
}