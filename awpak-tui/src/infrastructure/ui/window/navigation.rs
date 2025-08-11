use ratatui::{text::Line, Frame};

use crate::{domain::{app::model::app::{App, AppFocus}, content_generator::model::content_generator::ContentGenerator}, infrastructure::ui::{areas::areas::Areas, icons::navigation::{ICON_BACK, ICON_BACK_EMPTY, ICON_BACK_FOCUS, ICON_NAVIGATION_LINE, ICON_NAVIGATION_LINE_EMPTY, ICON_NAVIGATION_LINE_FOCUS, ICON_NEXT, ICON_NEXT_EMPTY, ICON_NEXT_FOCUS, ICON_UP, ICON_UP_EMPTY, ICON_UP_FOCUS}}};


pub fn render_navigation( app : &App, areas : &Areas, frame : &mut Frame )
{
    frame.render_widget(
        up_navigation( app ), 
        areas.navigation.up
    );

    frame.render_widget(
        left_navigation( app ), 
        areas.navigation.back
    );

    frame.render_widget(
        right_navigation( app ), 
        areas.navigation.next
    );
}

fn up_navigation( app : &App ) -> Line<'_>
{
    match app.content_generator()
    {
        ContentGenerator::Directory( _ ) => up_navigation_active( app ),
        _ => up_navigation_inactive()
    }
}

fn up_navigation_inactive() -> Line<'static>
{
    navigation_empty( ICON_UP_EMPTY )
}

fn up_navigation_active( app : &App ) -> Line<'_>
{
    match app.focus()
    {
        AppFocus::Up => navigation_focus( ICON_UP_FOCUS ),
        _ => navigation_idle( ICON_UP )    
    }
}

fn left_navigation( app : &App ) -> Line<'_>
{
    match app.history_back().len()
    {
        0 => match app.content_generator()
        {
            ContentGenerator::Detail( _, _ ) |
            ContentGenerator::Graph( _, _ ) => left_navigation_active( app ),
            _ => left_navigation_inactive()
        },
        _ => left_navigation_active( app )  
    }
}

fn left_navigation_active( app : &App ) -> Line<'_>
{
    match app.focus()
    {
        AppFocus::Back => navigation_focus( ICON_BACK_FOCUS ),
        _ => navigation_idle( ICON_BACK )
    }
}

fn left_navigation_inactive() -> Line<'static>
{
    navigation_empty( ICON_BACK_EMPTY )
}

fn right_navigation( app : &App ) -> Line<'_>
{
    match app.history_next().len()
    {
        0 => right_navigation_inactive(),
        _ => match app.content_generator()
        {
            ContentGenerator::Detail( _, _ ) |
            ContentGenerator::Graph( _, _ ) => right_navigation_inactive(),
            _ => right_navigation_active( app )
        }   
    }
}

fn right_navigation_active( app : &App ) -> Line<'_>
{
    match app.focus()
    {
        AppFocus::Next => navigation_focus( ICON_NEXT_FOCUS ),
        _ => navigation_idle( ICON_NEXT )    
    }
}

fn right_navigation_inactive() -> Line<'static>
{
    navigation_empty( ICON_NEXT_EMPTY )
}

fn navigation_idle( icon : char ) -> Line<'static>
{
    Line::from( format!( "{}{}{}", ICON_NAVIGATION_LINE, icon, ICON_NAVIGATION_LINE ) )
}

fn navigation_focus( icon : char ) -> Line<'static>
{
    Line::from( format!( "{}{}{}", ICON_NAVIGATION_LINE_FOCUS, icon, ICON_NAVIGATION_LINE_FOCUS ) )
}

fn navigation_empty( icon : char ) -> Line<'static>
{
    Line::from( format!( "{}{}{}", ICON_NAVIGATION_LINE_EMPTY, icon, ICON_NAVIGATION_LINE_EMPTY ) )
}