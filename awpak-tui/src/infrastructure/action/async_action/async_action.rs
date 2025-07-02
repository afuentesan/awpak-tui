use awpak_tui_ai::domain::chat::chat::Chat;

use crate::domain::graph::graph::AwpakTUIGraph;


pub enum AsyncAction
{
    SendChatRequest( Chat ),
    SendGraphRequest( AwpakTUIGraph )
}