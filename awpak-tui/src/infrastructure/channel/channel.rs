use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex, OnceLock};


pub fn init_global_channels()
{
    init_abort_chat_channel();
}

pub fn send_abort_chat( abort : bool ) -> bool
{
    match tx_abort_chat().lock().unwrap().as_ref()
    {
        Some( t ) =>
        {
            let _ = t.send( abort );

            true
        },
        _ => false  
    }
}

// pub async fn recv_abort_chat() -> Option<bool>
// {
//     match rx_abort_chat().lock().unwrap().as_mut()
//     {
//         Some( r ) =>
//         {
//             match r.recv()
//             {
//                 Ok( r ) => Some( r ),
//                 _ => None
//             }
//         },
//         _ => None    
//     }
// }

pub fn try_recv_abort_chat() -> Option<bool>
{
    match rx_abort_chat().lock().unwrap().as_mut()
    {
        Some( r ) => r.try_recv().ok(),
        _ => None    
    }
}

pub fn clean_recv_abort_chat()
{
    while let Some( _ ) = try_recv_abort_chat() {}
}

pub fn rx_abort_chat() -> &'static Arc<Mutex<Option<Receiver<bool>>>>
{
    static C : OnceLock<Arc<Mutex<Option<Receiver<bool>>>>> = OnceLock::new();
    C.get_or_init(|| Arc::new( Mutex::new( None ) ) )
}

fn tx_abort_chat() -> &'static Arc<Mutex<Option<Sender<bool>>>>
{
    static C : OnceLock<Arc<Mutex<Option<Sender<bool>>>>> = OnceLock::new();
    C.get_or_init(|| Arc::new( Mutex::new( None ) ) )
}

fn init_abort_chat_channel()
{
    let ( tx, rx ) = channel::<bool>();

    rx_abort_chat().lock().unwrap().replace( rx );
    tx_abort_chat().lock().unwrap().replace( tx );
}