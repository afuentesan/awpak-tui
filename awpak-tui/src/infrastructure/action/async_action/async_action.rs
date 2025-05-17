use awpak_tui_ai::domain::chat::chat::Chat;


pub enum AsyncAction
{
    SendChatRequest( Chat )
}