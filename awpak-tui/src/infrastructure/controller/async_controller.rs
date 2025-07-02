use std::{sync::mpsc::{Receiver, Sender}, thread};

use crate::infrastructure::{action::{app::action::Action, async_action::async_action::AsyncAction}, channel::channel::{clean_recv_abort_chat, try_recv_abort_chat}, controller::graph_controller::{capture_graph_output, send_prompt_to_graph}};


pub fn async_controller(
    app_sender : Sender<Action>,
    async_recv : Receiver<AsyncAction>
)
{
    capture_graph_output( app_sender.clone() );
    
    thread::spawn( || thread_async( app_sender, async_recv ) );
}

fn thread_async(
    app_sender : Sender<Action>,
    async_recv : Receiver<AsyncAction>
)
{
    let body = async
    {
        loop
        {
            match async_recv.recv()
            {
                Ok( a ) => execute_async_action( a, app_sender.clone() ).await,
                _ => break
            }
        }
    };

    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("Failed building the Runtime")
    .block_on(body);
}

async fn execute_async_action( action : AsyncAction, sender : Sender<Action> )
{
    match action
    {
        AsyncAction::SendGraphRequest( g ) =>
        {
            send_prompt_to_graph( g, sender ).await;
        }
    }
}