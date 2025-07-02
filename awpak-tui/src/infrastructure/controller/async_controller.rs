use std::{sync::mpsc::{Receiver, Sender}, thread};

use awpak_tui_ai::{domain::chat::chat::ChatChannel, infrastructure::controllers::chat_controller::send_propmt_to_chat};

use crate::infrastructure::{action::{app::action::Action, async_action::async_action::AsyncAction}, channel::channel::{clean_recv_abort_chat, try_recv_abort_chat}, controller::graph_controller::send_prompt_to_graph};


pub fn async_controller(
    app_sender : Sender<Action>,
    async_recv : Receiver<AsyncAction>
)
{
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
        AsyncAction::SendChatRequest( s ) =>
        {
            let chat_action = ChatAction { sender };

            clean_recv_abort_chat();
            
            send_propmt_to_chat( s, chat_action ).await
        },
        AsyncAction::SendGraphRequest( g ) =>
        {
            send_prompt_to_graph( g, sender ).await;
        }
    }
}

#[derive(Clone)]
struct ChatAction
{
    sender : Sender<Action>
}

impl ChatChannel for ChatAction
{
    fn send_str( &self, s : &str )
    {
        let _ = self.sender.send( Action::AppendTextToContent( s.to_string() ) );
    }

    fn end_chat( &self )
    {
        let _ = self.sender.send( Action::EndChatResponse );
    }

    fn abort( &self ) -> bool
    {
        match try_recv_abort_chat()
        {
            Some( b ) => b,
            _ => false    
        }
    }
}