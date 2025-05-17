use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy)]
pub struct StrLine
{
    pub from : u16,
    pub len : u16
}

impl StrLine
{
    pub fn iter( str : &str, max : Option<u16> ) -> LineIterator
    {
        LineIterator 
        { 
            text : str.split( "\n" ), 
            max, 
            idx : 0
        }
    }
}

pub struct LineIterator<'a>
{
    text : std::str::Split<'a, &'a str>,
    max : Option<u16>,
    idx : u16
}

impl<'a> Iterator for LineIterator<'a> 
{
    type Item = StrLine;

    fn next( &mut self ) -> Option<Self::Item> 
    {
        if let Some( max ) = self.max
        {
            if max < self.idx
            {
                return None
            }
        }

        match self.text.next()
        {
            Some( s ) =>
            {
                let len = str_len( s ) as u16;
                
                let str_line = StrLine { from : self.idx, len };

                let next_idx = self.idx + len + 1;
                self.idx = next_idx;

                Some( str_line )
            },
            None => None
        }
    }
}

pub fn str_len( str : &str ) -> usize
{
    str.graphemes( true )
    .count()
}

pub fn divide_str( str : &str, position : usize ) -> ( &str, &str )
{
    if str_len( str ) <= position
    {
        return ( str, "" )
    }
    
    let indices = UnicodeSegmentation::grapheme_indices( str, true )
    .fold(
        ( 0, 0 ), 
        | mut a, ( i, _ ) |
        {
            if a.1 == position
            {
                a.0 = i;
            }

            a.1 += 1;

            a
        }
    );

    ( &str[ 0..indices.0 ], &str[ indices.0.. ] )
}

pub fn split_str_by_len( str : &str, len : usize ) -> Vec<String>
{
    str.split_word_bounds().fold(
        vec![ "".to_string() ], 
        | a, w | append_or_new_line( a, w, len )
    )
}

fn append_or_new_line( mut lines : Vec<String>, word : &str, len : usize ) -> Vec<String>
{
    let last = lines.last().unwrap();

    let word_len = str_len( word );
    let last_len = str_len( last );

    if word_len > len
    {
        if last.trim() == ""
        {
            lines.pop();
        }

        lines.append( &mut split_line( word, len ) );
    }
    else if ( last_len + word_len ) > len
    {
        lines.push( word.trim_start().to_string() );
    }
    else
    {
        let idx = lines.len() - 1;

        lines[ idx ].push_str( word );
    }

    lines
}

fn split_line( str : &str, len : usize ) -> Vec<String>
{
    let chars = str.graphemes( true )
    .collect::<Vec<_>>();

    chars.iter().enumerate().step_by( len )
    .map(
        | ( i, _ ) |
        {
            let last = usize::min( chars.len(), i + len );

            chars[i..last].join( "" )
        }
    )
    .collect()
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_str_len()
    {
        assert_eq!( str_len( "" ), 0 );
        assert_eq!( str_len( "ñ" ), 1 );
        assert_eq!( str_len( "Ñ" ), 1 );
        assert_eq!( str_len( "Ñá" ), 2 );
        assert_eq!( str_len( "Ñápa" ), 4 );
        assert_eq!( str_len( "Ñápa.-/" ), 7 );
    }

    #[test]
    fn test_split_str_by_len()
    {
        let str = "hola que tal";

        let out = split_str_by_len( str, 4 );

        assert_eq!( out.len(), 3 );
        assert_eq!( out[ 0 ], "hola" );
        assert_eq!( out[ 1 ], "que " );
        assert_eq!( out[ 2 ], "tal" );

        let out = split_str_by_len( str, 3 );
        assert_eq!( out.len(), 4 );
        assert_eq!( out[ 0 ], "hol" );
        assert_eq!( out[ 1 ], "a " );
        assert_eq!( out[ 2 ], "que" );
        assert_eq!( out[ 3 ], "tal" );

        let str = r#"Destination path exists. Destination: "/home/angel/tmp/pruebas_file_explorer/dir1/file1.txt""#;

        let out = split_str_by_len( str, 48 );

        assert_eq!( out.len(), 2 );

        assert_eq!( out[ 0 ], r#"Destination path exists. Destination: "/home/"# );
        assert_eq!( out[ 1 ], r#"angel/tmp/pruebas_file_explorer/dir1/file1.txt""# );
    }
}