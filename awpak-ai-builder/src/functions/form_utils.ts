import { v4 as uuidv4 } from 'uuid';

export interface SelectOption
{
    name : string,
    value : string,
    selected : boolean
}

export function select_options_from_array(
    options : Array<any>,
    selected_value : any,
    empty_option : boolean
) : Array<SelectOption>
{
    let ret : Array<SelectOption> = [];

    let has_selected = ! ( typeof( selected_value ) === "undefined" || selected_value === null );

    if( empty_option )
    {
        ret.push(
            {
                name : "-",
                value : "",
                selected : false
            }
        )
    }

    options.forEach( 
        ( val : string ) =>
        {
            ret.push( 
                {
                    name : val,
                    value : val,
                    selected : ( has_selected && selected_value == val ) || false
                } 
            );
        }
    );

    return ret;
}

export function select_options_from_enum( 
    enum_type : any, 
    selected_value : any,
    empty_option : boolean 
) : Array<SelectOption>
{
    let ret : Array<SelectOption> = [];

    let has_selected = ! ( typeof( selected_value ) === "undefined" || selected_value === null );

    if( empty_option )
    {
        ret.push(
            {
                name : "-",
                value : "",
                selected : false
            }
        )
    }

    (
        Object.keys( enum_type )
        .filter( value => typeof value === 'string' ) as string[]
    ).forEach( 
        ( val : string ) =>
        {
            ret.push( 
                {
                    name : val,
                    value : val,
                    selected : ( has_selected && selected_value == val ) || false
                } 
            );
        }
    );

    return ret;
}

export function random_id() : string
{
    return uuidv4();
}