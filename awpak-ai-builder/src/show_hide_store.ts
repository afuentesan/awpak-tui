import { atom } from "nanostores";

export const show_hide_state = atom( new Map<string, boolean>() );

export function is_box_hidden( base_path : string | undefined ) : boolean
{
    if( ! base_path?.trim() ) return false;

    return ( show_hide_state.get().get( base_path ) ? true : false );
}

export function change_box_state( base_path : string | undefined, next : boolean )
{
    if( ! base_path?.trim() ) return false;

    let new_state = new Map( show_hide_state.get() );

    new_state.set( base_path, next );

    show_hide_state.set( new_state );
}