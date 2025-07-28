
<script lang="ts">
    import { is_empty } from "../../functions/data_functions";
    import { random_id } from "../../functions/form_utils";

    let { label, value, change_value, base_path } = $props();

    let id = random_id();

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
</script>

<div>
    <label for={id} class="block mb-2 mt-2 text-sm font-medium text-gray-900 dark:text-white">{label}</label>
    <textarea
        value={is_empty( value ) ? "" : value}
        onchange={local_change_value}
        onblur={local_change_value}
        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" 
    ></textarea>
</div>