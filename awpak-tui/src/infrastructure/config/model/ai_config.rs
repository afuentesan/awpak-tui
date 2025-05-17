use awpak_tui_ai::domain::agent::agent::AIAgent;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AIConfig
{
    pub agents : Vec<AIAgent>
}