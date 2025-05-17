use crate::domain::app::model::app::App;


pub enum WindowAction
{
    Render( App ),
    ShowLoading,
    ConfirmShowLoading,
    HideLoading,
    MoveCursorContent( CursorDirection ),
    Exit
}

#[derive(Clone, Copy)]
pub enum CursorDirection
{
    Up,
    Down,
    End
}