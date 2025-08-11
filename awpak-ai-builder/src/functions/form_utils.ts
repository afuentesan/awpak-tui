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

export function is_type_in_enum(
    enum_type : any,
    val : string
) : boolean
{
    return array_from_enum( enum_type ).includes( val )
}

export function array_from_enum(
    enum_type : any
) : Array<string>
{
    return Object.keys( enum_type )
                 .filter( value => typeof value === 'string' ) as string[]
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

export function btn_classes( color? : string ) : string
{
    if( ! color ) color = "green";

    let btn_cls = "focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 mt-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800";

    if( color == "red" )
    {
        btn_cls = "focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900";
    }
    else if( color == "blue" )
    {
        btn_cls = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800";
    }
    else if( color == "purple" )
    {
        btn_cls = "focus:outline-none text-white bg-purple-700 hover:bg-purple-800 focus:ring-4 focus:ring-purple-300 font-medium rounded-lg text-sm px-5 py-2.5 mb-2 dark:bg-purple-600 dark:hover:bg-purple-700 dark:focus:ring-purple-900";
    }
    else if( color == "indigo" )
    {
        btn_cls = "focus:outline-none text-white bg-indigo-700 hover:bg-indigo-800 focus:ring-4 focus:ring-indigo-300 font-medium rounded-lg text-sm px-5 py-2.5 mb-2 dark:bg-indigo-600 dark:hover:bg-indigo-700 dark:focus:ring-indigo-900";
    }
    else if( color == "yellow" )
    {
        btn_cls = "focus:outline-none text-white bg-yellow-400 hover:bg-yellow-500 focus:ring-4 focus:ring-yellow-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:focus:ring-yellow-900";
    }
    else if( color == "light" )
    {
        btn_cls = "text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700";
    }

    return btn_cls;
}