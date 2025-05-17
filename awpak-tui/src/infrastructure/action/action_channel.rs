use std::sync::mpsc::{self, Receiver, Sender};

use super::{app::action::Action, async_action::async_action::AsyncAction, window::window_action::WindowAction};


pub struct ActionChannel
{
    pub app_sender : Sender<Action>,
    pub app_recv : Receiver<Action>,

    pub window_sender : Sender<WindowAction>,
    pub window_recv : Receiver<WindowAction>,

    pub chat_sender : Sender<AsyncAction>,
    pub chat_recv : Receiver<AsyncAction>
}

pub fn init_channels() -> ActionChannel
{
    let ( window_sender, window_recv ) = mpsc::channel::<WindowAction>();

    let ( app_sender, app_recv ) = mpsc::channel::<Action>();

    let ( chat_sender, chat_recv ) = mpsc::channel::<AsyncAction>();

    ActionChannel{ app_sender, app_recv, window_sender, window_recv, chat_sender, chat_recv }
}