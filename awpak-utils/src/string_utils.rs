use unicode_segmentation::UnicodeSegmentation;


pub fn str_len( str : &str ) -> usize
{
    str.graphemes( true )
    .count()
}