
pub fn parse_params( params : Option<&String> ) -> Vec<String>
{
    params.iter()
    .flat_map( | p | shellwords::split( p.as_str() ) )
    .next()
    .unwrap_or( vec![] )
}