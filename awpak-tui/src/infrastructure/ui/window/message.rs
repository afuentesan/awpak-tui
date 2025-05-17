use ratatui::Frame;

use crate::{domain::{app::model::app::App, message::model::message::Message}, infrastructure::ui::{areas::areas::Areas, color::palette::Palette, modal::modal::render_modal}};


pub fn render_message( app : &App, areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    match app.message()
    {
        Some( m ) => render( m, areas, frame, palette ),
        _ => {}
    }
}

fn render( message : &Message, areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    let msg = message.as_str();
    let title = title( message );

    render_modal( title, msg, areas, frame, palette );
}

fn title( message : &Message ) -> &'static str
{
    match message
    {
        Message::Error( _ ) => " Error ",
        Message::Warning( _ ) => " Warning ",
        Message::Info( _ ) => " Info "
    }
}

// fn area_message( areas : &Areas, width : u16, height : u16 ) -> Rect
// {
//     let height = u16::min( areas.full.height - 4 , height );

//     let x = ( areas.full.width / 2 ) - ( width / 2 );
//     let y = u16::min( areas.content.y - 2, ( areas.full.height / 2 ) - ( height / 2 ) );

//     Rect::new( x, y, width, height )
// }

// fn calc_max_width( areas : &Areas ) -> u16
// {
//     u16::min( areas.full.width - 2, 50 )
// }