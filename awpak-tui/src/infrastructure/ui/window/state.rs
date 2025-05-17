use ratatui::widgets::TableState;

use crate::infrastructure::ui::text_area::text_area::ScrollState;


#[derive(Default)]
pub struct WindowState
{
    pub content : TableState,
    pub sources : TableState,
    pub search : ScrollState,
    pub edit_field : ScrollState,
    pub chat_response : ScrollState,
    pub confirm_list : TableState
}