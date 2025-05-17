use ratatui::layout::{Constraint, Layout, Rect};

use crate::{domain::app::model::app::{App, AppContent}, infrastructure::ui::util::ui_utils::relative_area};

pub struct Areas
{
    pub full : Rect,
    pub navigation : NavigationAreas,
    pub content_generator : Rect,
    pub sources : Rect,
    pub info_sources : Rect,
    pub search : Rect,
    pub content : Rect,
    pub info_content : Rect
}

pub struct NavigationAreas
{
    pub back : Rect,
    pub next : Rect,
    pub up : Rect
}

pub fn content_area( app : &App, full : Rect ) -> Rect
{
    let layout = Layout::vertical( [ Constraint::Length( 3 ), Constraint::Fill( 0 ) ] );

    let [ _, middle ] = layout.areas( full );

    let ( _, _, content ) = create_middle_areas( app, middle );

    content
}

pub fn create_areas( app : &App, full : Rect ) -> Areas
{
    let layout = Layout::vertical( [ Constraint::Length( 3 ), Constraint::Fill( 0 ) ] );

    let [ top, middle ] = layout.areas( full );

    let ( navigation, content_generator ) = create_top_areas( top );
    let ( sources, search, content ) = create_middle_areas( app, middle );
    
    Areas
    { 
        full, 
        navigation, 
        content_generator, 
        sources,
        info_sources : info_bottom( sources ),
        search, 
        content,
        info_content : info_bottom( content )
    }
}

fn info_bottom( area : Rect ) -> Rect
{
    Rect::new( area.x + 1, area.y + area.height + 1, area.width - 2, 1 )
}

fn create_middle_areas( app : &App, area : Rect ) -> ( Rect, Rect, Rect )
{
    let layout = Layout::horizontal( [ Constraint::Percentage( 20 ), Constraint::Percentage( 80 ) ] );

    let area = relative_area( area, 1, 0, -2, -1 );

    let [ mut sources, mut right ] = layout.areas( area );

    sources.width -= 1;
    right.width -= 1;
    right.x += 1;

    let search_height = match app.content()
    {
        AppContent::Chat( _ ) =>
        {
            usize::max(
                usize::min(
                    app.content_search().text.split( "\n" ).collect::<Vec<_>>().len(),
                    4
                ),
                1 
            )
        },
        _ => 1
    } as u16;

    // println!( "Seeeearch heightttt: {}", search_height );

    let layout = Layout::vertical( [ Constraint::Length( search_height + 1 ), Constraint::Fill( 0 ) ] );

    let [ search, mut content ] = layout.areas( right );

    content.height -= 2;
    sources.height -= 2;

    let search = relative_area( search, 1, 0, -2, -1 );

    ( sources, search, content )
}

fn create_top_areas( area : Rect ) -> ( NavigationAreas, Rect )
{
    let layout = Layout::horizontal( [ Constraint::Percentage( 20 ), Constraint::Percentage( 80 ) ] );

    let area = Rect::new( area.x + 1, area.y + 1, area.width - 2, area.height - 2 );

    let [ navigation, path ] = layout.areas( area );

    let layout = Layout::horizontal( [ Constraint::Length( 3 ), Constraint::Length( 3 ), Constraint::Length( 3 ), Constraint::Fill( 0 ) ] );

    let [ back, next, up, _ ] = layout.areas( navigation );

    (
        NavigationAreas { back, next, up },
        path
    )
}