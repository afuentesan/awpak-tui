use std::{sync::mpsc::Sender, thread};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::infrastructure::action::app::action::Action;

pub fn init_handle_events( sender : Sender<Action> )
{
    thread::spawn( move || handle_events( sender ) );
}

fn handle_events( sender : Sender<Action> )
{
    let _ = sender.send( Action::Render );

    loop
    {
        let _ = crossterm::event::read().map( | ev | handle_event( ev, sender.clone() ) );
    }
}

fn handle_event( ev : crossterm::event::Event, sender : Sender<Action> )
{
    match ev
    {
        crossterm::event::Event::Key( k ) =>
        {
            if let Some( a ) = app_action_from_key_event( k )
            {
                let _ = sender.send( a );
            }
        },
        _ => return    
    }
}

fn app_action_from_key_event( key : KeyEvent ) -> Option<Action>
{
    match ( key.kind, key.modifiers, key.code )
    {
        ( KeyEventKind::Press, KeyModifiers::CONTROL, KeyCode::Char('q') ) => Some( Action::Exit ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Char('i') ) => Some( Action::AltI ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Char('a') ) => Some( Action::AltA ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Char('s') ) => Some( Action::AltS ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Char( 'x' ) ) => Some( Action::AltX ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Up ) => Some( Action::Up ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Up ) => Some( Action::AltUp ),
        ( KeyEventKind::Press, KeyModifiers::SHIFT, KeyCode::Up ) => Some( Action::ShiftUp ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Down ) => Some( Action::Down ),
        ( KeyEventKind::Press, KeyModifiers::SHIFT, KeyCode::Down ) => Some( Action::ShiftDown ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Left ) => Some( Action::Left ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Left ) => Some( Action::AltLeft ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Right ) => Some( Action::Right ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Right ) => Some( Action::AltRight ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Enter ) => Some( Action::AltEnter ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Enter ) => Some( Action::Enter ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Tab ) => Some( Action::Tab ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Tab ) => Some( Action::AltTab ),
        ( KeyEventKind::Press, KeyModifiers::ALT, KeyCode::Char( c ) ) 
        if c.is_digit( 10 ) => Some( Action::AltNumber( c.to_string().parse().unwrap() ) ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Backspace ) => Some( Action::Backspace ),
        ( KeyEventKind::Press, KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char( c ) ) => Some( Action::Char( c ) ),
        ( KeyEventKind::Press, KeyModifiers::NONE, KeyCode::Esc ) => Some( Action::Esc ),
        ( KeyEventKind::Press, KeyModifiers::CONTROL, KeyCode::Char('c') ) => Some( Action::CtrlC ),
        ( KeyEventKind::Press, KeyModifiers::CONTROL, KeyCode::Char('v') ) => Some( Action::CtrlV ),
        ( KeyEventKind::Press, KeyModifiers::CONTROL, KeyCode::Char('x') ) => Some( Action::CtrlX ),
        ( KeyEventKind::Press, KeyModifiers::CONTROL, KeyCode::Char('d') ) => Some( Action::CtrlD ),
        ( KeyEventKind::Press, KeyModifiers::CONTROL, KeyCode::Char('s') ) => Some( Action::CtrlS ),
        _ => None
    }
}