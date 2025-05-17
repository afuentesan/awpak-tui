
pub trait Selectable
{
    fn current_selected( &self ) -> bool;
    fn selected( &self ) -> bool;
    fn idle( &self ) -> bool;
    fn hidden( &self ) -> bool;

    fn to_current_selected( self ) -> Self;
    fn to_selected( self ) -> Self;
    fn to_idle( self ) -> Self;
    fn to_hidden( self ) -> Self;

    fn can_be_selected( &self ) -> bool
    {
        true
    }
}