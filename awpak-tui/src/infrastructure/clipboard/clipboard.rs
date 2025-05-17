use std::io::Read;
use clipboard_rs::{Clipboard as _, ClipboardContext};
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

#[macro_export]
macro_rules! get_first_some {
    ( $( $x:expr ),* ) => 
    {
        ( ||
        {
            $(
                if let Some( a ) = $x
                {
                    return Some( a )
                }
            )*

            return None
        } )()
    }
}

pub fn text_from_clipboard() -> Option<String>
{
    get_first_some!(
        text_from_clipboard_rs(),
        text_from_wl_clipboard( ClipboardType::Regular ),
        text_from_wl_clipboard( ClipboardType::Primary )
    )
}

fn text_from_clipboard_rs() -> Option<String>
{
    let ctx = ClipboardContext::new().ok()?;

    ctx.get_text().ok()
}

fn text_from_wl_clipboard( c_type : ClipboardType ) -> Option<String>
{
    match get_contents( c_type, Seat::Unspecified, MimeType::Text )
    {
        Ok( ( mut pipe, _ ) ) =>
        {
            let mut contents = vec![];
            pipe.read_to_end(&mut contents).ok()?;
            Some( String::from_utf8_lossy(&contents).to_string() )
        }
        _ => None
    }
}