use ratatui::{layout::Constraint, style::Stylize, text::Text, widgets::{Cell, HighlightSpacing, Row}};

use crate::{domain::selectable::model::selectable_item::SelectableItem, infrastructure::ui::{color::table::TableColors, table::from_table::row_color}};


pub fn list_from_selectable<T>( 
    items : &Vec<SelectableItem<T>>,
    title : &str,
    colors : &TableColors
) -> ratatui::widgets::Table<'static>
where T: Default + ToString
{
    let headers = Row::new( 
        vec![ title.to_string() ] 
    )
    .bg( colors.header_bg ).fg( colors.header_fg ).bold();

    let rows = items.iter().map(
        | i |
        {
            let color = row_color( i, colors );

            Row::new( vec![ Cell::new( i.inner().to_string() ) ] ).bg( color.0 ).fg( color.1 )
        }
    ).collect::<Vec<_>>();

    let bar = format!( " {} ", "⏺" );

    ratatui::widgets::Table::new(
        rows,
        [ Constraint::Fill( 0 ) ],
    )
    .header(headers)
    .highlight_symbol(Text::from(vec![bar.into()]))
    .bg( colors.bg )
    .highlight_spacing(HighlightSpacing::Always)
}