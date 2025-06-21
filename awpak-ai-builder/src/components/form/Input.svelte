
<script lang="ts">
    import { random_id } from "../../functions/form_utils";

    let { label, value, input_type = undefined, change_value } = $props();

    if( ! input_type?.trim() ) { input_type = "text"; }

    let id = random_id();

    function local_change_value( event : any )
    {
        if( typeof( change_value ) == "function" )
        {
            change_value( event );
        }
    }
</script>

<div>
    <label for={id} class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{label}</label>
    <input 
        type={input_type} 
        id={id} 
        value={value}
        onchange={local_change_value}
        onblur={local_change_value}
        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" 
    />
</div>