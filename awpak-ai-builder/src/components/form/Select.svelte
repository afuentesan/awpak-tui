
<script lang="ts">
    import { random_id } from "../../functions/form_utils";

    let { label, value, options, change_value, base_path } = $props();

    let id = random_id();

    function is_value_selected( value : any, selected_value : any ) : boolean
    {
        if( typeof( selected_value ) === "undefined" || selected_value === null )
        {
            return false;
        }

        return value == selected_value
    }

    function local_change_value( event : any )
    {
        if( typeof( change_value ) == "function" )
        {
            if( ! base_path?.trim() )
            {
                change_value( event );   
            }
            else
            {
                change_value( base_path, event?.target?.value );
            }
        }
    }

    // $effect(() => {
        
        
    //     if( value || ! value || base_path )
    //     {
    //         console.log( "Options: ", options );

    //         options = options.map( ( o : any ) => Object.assign( {}, o ) );
    //     }
        
    // });

</script>

<div>
    <label for={id} class="block mb-2 mt-2 text-sm font-medium text-gray-900 dark:text-white">{label}</label>
    <select id={id} onchange={local_change_value} class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500">
        {#each options as option}
            <option value={option.value} selected={is_value_selected( option.value, value )}>{option.name}</option>
        {/each}
    </select>
</div>