use std::cmp::Ordering;


pub trait Sortable
{
    fn sort( &self, other : &Self, sort_by : SortBy ) -> Ordering;

    fn reverse( ordering : Ordering ) -> Ordering
    {
        match ordering
        {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortBy
{
    Default,
    ReverseDefault,
    Column( usize ),
    ReverseColumn( usize )
}
