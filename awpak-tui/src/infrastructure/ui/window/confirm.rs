use std::collections::HashMap;

use ratatui::{layout::Rect, widgets::Clear, Frame};

use crate::{domain::{app::model::app::{App, AppFocus, Confirm}, movible::model::movible::{Movible, MovibleAction}, selectable::functions::selectable_utils::idx_current_selected_item_filter_hidden, util::string_utils::str_len}, infrastructure::ui::{areas::areas::Areas, color::{palette::Palette, table::TableColors}, list::from_selectable::list_from_selectable, modal::modal::render_modal}};

use super::state::WindowState;


pub fn render_confirm( app : &App, areas : &Areas, frame : &mut Frame, window_state : &mut WindowState, palette : &Palette )
{
    match app.focus()
    {
        AppFocus::Confirm( c ) => render_confirm_type( app, areas, frame, c, window_state, palette ),
        _ => {}
    }
}

fn render_confirm_type( 
    app : &App, 
    areas : &Areas, 
    frame : &mut Frame, 
    confirm : Confirm, 
    window_state : &mut WindowState,
    palette : &Palette
)
{
    match confirm
    {
        Confirm::MovibleAction => render_confirm_movible( app, areas, frame, palette ),
        Confirm::AgentSelection => render_confirm_agent_selection( app, areas, frame, window_state, palette ),
    }
}

fn render_confirm_agent_selection( 
    app : &App, 
    areas : &Areas, 
    frame : &mut Frame, 
    window_state : &mut WindowState,
    palette : &Palette
)
{
    window_state.confirm_list.select( idx_current_selected_item_filter_hidden( app.ai_agents() ) );

    let list = list_from_selectable( app.ai_agents(), "Select AI", &TableColors::default_selected( palette ) );

    let area = area_confirm_agent_selection( app, areas );

    frame.render_widget( Clear, area );
    frame.render_stateful_widget(list, area, &mut window_state.confirm_list );
}

fn area_confirm_agent_selection( app : &App, areas : &Areas ) -> Rect
{
    let height = u16::min( app.ai_agents().len() as u16 + 1, areas.full.height - 2 );

    let width = u16::max( u16::min( 
        areas.full.width - 2,
        app.ai_agents().iter()
        .fold(
            0, 
            | a, i |
            {
                let len = str_len( i.inner().to_string().as_str() ) as u16;

                if len > a { len } else { a }
            }
        )
    ), 50 );

    let x = ( areas.full.width / 2 ) - ( width / 2 );
    let y = ( areas.full.height / 2 ) - ( height / 2 );

    let y = u16::min( y, areas.content.y + 3 );

    Rect::new( x, y, width, height )
}

fn render_confirm_movible( app : &App, areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    match app.movible_action()
    {
        MovibleAction::Delete( m ) => render_confirm_movible_delete( areas, frame, m, palette ),
        MovibleAction::Copy( _ ) |
        MovibleAction::Cut( _ ) |
        MovibleAction::None => {}
    }
}

fn render_confirm_movible_delete( areas : &Areas, frame : &mut Frame, movibles : &Vec<Movible>, palette : &Palette )
{
    let title = "Confirm delete";

    let mut msg = if movibles.len() == 1
    {
        format!( "Delete {} {}?", key_movible( &movibles[ 0 ] ), movibles[ 0 ].path().to_string_lossy() )
    }
    else
    {
        format!( "Delete {}?", parse_files_and_dirs( files_and_dirs( movibles ) ) )
    };

    msg.push_str( "\n\nESC to Cancel. ENTER to Confirm" );

    render_modal( title, msg.as_str(), areas, frame, palette );
}

fn parse_files_and_dirs( map : HashMap<&'static str, usize> ) -> String
{
    let mut ret = map.iter()
    .map( | a | format!( "{} {},", a.1, a.0 ) )
    .collect::<Vec<_>>()
    .join( " " );

    ret.pop();
    
    ret
}

fn files_and_dirs( movibles : &Vec<Movible> ) -> HashMap<&'static str, usize>
{
    movibles.iter().fold(
        HashMap::<&'static str, usize>::new(), 
        | mut a, m |
        {
            let key = key_movible( m );

            if ! a.contains_key( key )
            {
                a.insert( key, 0 );
            }

            let next = a.get( key ).unwrap() + 1;

            a.insert( key, next );

            a
        }
    )
}

fn key_movible( movible : &Movible ) -> &'static str
{
    match movible
    {
        Movible::File( _ ) => "Files",
        Movible::Directory( _ ) => "Directories"
    }
}