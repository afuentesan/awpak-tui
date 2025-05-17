use ratatui::{layout::Rect, text::Line, Frame};

use crate::{domain::app::model::app::{App, AppFocus}, infrastructure::ui::{areas::areas::Areas, color::{line::LineColors, palette::Palette}, icons::line::LineIcons, util::ui_utils::{line_with_chars, merge_areas, relative_area, render_widgets, BorderPosition}}};


pub fn render_lines( app : &App, areas : &Areas, frame : &mut Frame, palette : &Palette )
{
    let vertical = line_with_chars( LineIcons::vertical_icons() )( LineColors::colors( palette ) );
    let horizontal = line_with_chars( LineIcons::horizontal_icons() )( LineColors::colors( palette ) );
    let horizontal_dotted = line_with_chars( LineIcons::horizontal_dotted_icons() )( LineColors::colors( palette ) );
    let horizontal_selected = line_with_chars( LineIcons::horizontal_selected_icons() )( LineColors::colors_selected( palette ) );

    render_header_sep( areas, frame, &horizontal );
    render_content_sep( areas, frame, &vertical );
    render_search_sep( app, areas, frame, &horizontal_dotted, &horizontal_selected );
    render_info_sep( app, areas, frame, &horizontal_dotted, &horizontal_selected );
}

fn render_content_sep(
    areas : &Areas, 
    frame : &mut Frame,
    vertical : &Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>
)
{
    let area = relative_area( areas.sources, 0, 0, 0, 2 );

    render_widgets( vertical( area, vec![ BorderPosition::Right ] ), frame );

    let area = relative_area( areas.sources, 1, 0, 0, 2 );

    render_widgets( vertical( area, vec![ BorderPosition::Right ] ), frame );
}

fn render_search_sep(
    app : &App,
    areas : &Areas,
    frame : &mut Frame,
    horizontal : &Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>,
    horizontal_selected : &Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>
)
{
    let h = match app.focus()
    {
        AppFocus::Search => horizontal_selected,
        _ => horizontal    
    };

    let area = relative_area( areas.search, -1, 0, 2, 0 );

    render_widgets( h( area, vec![ BorderPosition::Bottom ] ), frame );
}

fn render_info_sep(
    app : &App,
    areas : &Areas, 
    frame : &mut Frame,
    horizontal : &Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>,
    horizontal_selected : &Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>
)
{
    let ( h_sources, h_content ) = match app.focus()
    {
        AppFocus::Content => ( horizontal, horizontal_selected ),
        AppFocus::Sources => ( horizontal_selected, horizontal ),
        AppFocus::Search |
        AppFocus::Back |
        AppFocus::Up |
        AppFocus::Next |
        AppFocus::Confirm( _ ) |
        AppFocus::Field => ( horizontal, horizontal )
    };

    let area = relative_area( areas.info_sources, -1, 0, 2, 0 );

    render_widgets( h_sources( area, vec![ BorderPosition::Top ] ), frame );

    let area = relative_area( areas.info_content, -1, 0, 2, 0 );

    render_widgets( h_content( area, vec![ BorderPosition::Top ] ), frame );
}

fn render_header_sep( 
    areas : &Areas, 
    frame : &mut Frame,
    horizontal : &Box<dyn Fn( Rect, Vec<BorderPosition> ) -> Vec<(Rect, Line<'static>)>>
)
{
    let area = relative_area(
        merge_areas( areas.navigation.back, areas.content_generator ),
        0,
        0,
        0,
        0
    );

    render_widgets( horizontal( area, vec![ BorderPosition::Bottom ] ), frame );
}