use std::collections::HashMap;

use ratatui::{layout::{Alignment, Constraint}, style::{Color, Style, Stylize}, text::Text, widgets::{Cell, HighlightSpacing, Row}};

use crate::{domain::{selectable::model::{selectable::Selectable, selectable_item::SelectableItem}, table::model::{header::Header, rowable::Rowable, table::Table}}, infrastructure::ui::color::table::TableColors};

pub fn idx_visible_columns( table : &Table ) -> HashMap<usize, bool>
{
    table.headers().iter().enumerate()
    .filter( | h | h.1.visible() )
    .fold(
        HashMap::new(), 
        | mut a, ( i, _ ) |
        {
            a.insert( i, true );

            a
        }
    )
}

pub fn ui_from_table<'a>(
    table : &'a Table, 
    visible_columns : &'a HashMap<usize, bool>,
    colors : &'a TableColors,
    fn_render_cell : impl Fn( usize, &'a SelectableItem<crate::domain::table::model::cell::Cell>, &'a TableColors, bool ) -> Cell<'a>,
    fn_constraints : impl Fn( &HashMap<usize, bool>, &Table ) -> Vec<Constraint>
) -> ratatui::widgets::Table<'a>
{
    let header_row = headers_to_header_row( 
        table.headers(),  
        colors.header_bg, 
        colors.header_fg
    );

    let bg_color = colors.bg;

    let rows = table_to_rows( table, colors, visible_columns, fn_render_cell );

    let constraints = fn_constraints( visible_columns, table );

    build_table( header_row, rows, constraints, bg_color )
}

fn build_table<'a>(
    headers : Row<'a>,
    rows : Vec<Row<'a>>,
    constraints : Vec<Constraint>,
    bg_color : Color
) -> ratatui::widgets::Table<'a>
{
    let bar = format!( " {} ", "⏺" );

    ratatui::widgets::Table::new(
        rows,
        constraints,
    )
    .header(headers)
    .highlight_symbol(Text::from(vec![bar.into()]))
    .bg( bg_color )
    .highlight_spacing(HighlightSpacing::Always)
}

fn table_to_rows<'a>(
    table : &'a Table, 
    colors : &'a TableColors,
    visible_columns : &'a HashMap<usize, bool>,
    fn_render_cell : impl Fn( usize, &'a SelectableItem<crate::domain::table::model::cell::Cell>, &'a TableColors, bool ) -> Cell<'a>
) -> Vec<Row<'a>>
{
    table.rows().iter()
    .filter( | r | ! r.hidden() )
    .map( 
        | i |
        {
            let row_color = row_color( i, colors );

            cells_to_row( 
                i.inner().cells(),
                visible_columns,
                &fn_render_cell,
                row_color.1,
                row_color.0,
                &colors,
                i.current_selected()
            )
        }
    )
    .collect()
}

fn cells_to_row<'a>(
    cells : &'a Vec<SelectableItem<crate::domain::table::model::cell::Cell>>,
    visible_columns : &'a HashMap<usize, bool>,
    fn_render_cell : impl Fn( usize, &'a SelectableItem<crate::domain::table::model::cell::Cell>, &'a TableColors, bool ) -> Cell<'a>,
    fg_color : Color,
    bg_color : Color,
    colors : &'a TableColors,
    row_selected : bool
) -> Row<'a>
{
    cells.iter()
    .enumerate()
    .filter( | c | visible_columns.contains_key( &c.0 ) )
    .map( | ( i, c ) | fn_render_cell( i, c, colors, row_selected ) )
    .collect::<Row>()
    .style(Style::new().fg( fg_color ).bg( bg_color ) )
    .height(1)
}

fn headers_to_header_row<'a>( 
    headers : &'a Vec<Header>, 
    header_bg : Color,
    header_fg : Color
) -> Row<'a>
{
    headers.iter()
    .filter( | h | h.visible() )
    .map( | s | Cell::from( s.to_string() ) )
    .collect::<Row>()
    .bg( header_bg )
    .fg( header_fg )
    .height(1)
}

pub fn row_color<T>( 
    item : &SelectableItem<T>,
    colors : &TableColors
) -> ( Color, Color )
where T: Default
{
    match item
    {
        SelectableItem::CurrentSelected( _ ) |
        SelectableItem::Selected( _ ) => ( colors.row_bg_selected, colors.row_fg_selected ),
        SelectableItem::Idle( _ ) |
        SelectableItem::Hidden( _ ) => ( colors.row_bg_idle, colors.row_fg_idle )
    }
}

pub fn render_cell_default<'a>( 
    _ : usize, 
    cell : &'a SelectableItem<crate::domain::table::model::cell::Cell>, 
    colors : &'a TableColors,
    row_selected : bool
) -> Cell<'a>
{
    let mut table_cell = Cell::from( 
        Text::from( cell.inner().to_string() )
        .alignment( Alignment::Left ) 
    );

    if cell.current_selected() && row_selected
    {
        table_cell = table_cell.bg( colors.cell_bg_selected ).fg( colors.cell_fg_selected );
        // table_cell = table_cell.bg( WHITE_2 ).fg( BLACK_1 );
    }
    
    table_cell
}

pub fn render_cell_detail<'a>( 
    idx : usize, 
    cell : &'a SelectableItem<crate::domain::table::model::cell::Cell>, 
    colors : &'a TableColors,
    row_selected : bool
) -> Cell<'a>
{
    let ( text, align ) = if idx == 0 
    { 
        ( format!( "{}: ", cell.inner().to_string() ), Alignment::Right )
    } 
    else 
    { 
        ( cell.inner().to_string(), Alignment::Left )
    };

    let mut table_cell = Cell::from( 
        Text::from( text )
        .alignment( align ) 
    );

    if cell.current_selected() && row_selected
    {
        table_cell = table_cell.bg( colors.cell_bg_selected ).fg( colors.cell_fg_selected );
        // table_cell = table_cell.bg( WHITE_2 ).fg( BLACK_1 );
    }

    table_cell
}

pub fn constraints_table_detail( _ : &HashMap<usize, bool>, table : &Table ) -> Vec<Constraint>
{
    let len = table.rows().iter().fold(
        0,
        | a, r | 
        {
            let c = r.inner().cells()[ 0 ].inner().to_string().chars().count();

            if c > a { c } else { a }
        }
    );

    let len = usize::min( len + 2, 50 );

    vec![ Constraint::Length( len as u16 ), Constraint::Fill( 0 ) ]
}

pub fn constraints_table_default( visible_columns : &HashMap<usize, bool>, _ : &Table ) -> Vec<Constraint>
{
    vec![ Constraint::Min( 10 ); visible_columns.len() ]
}
