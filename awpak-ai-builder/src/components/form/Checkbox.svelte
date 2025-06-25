
<script lang="ts">
    import { random_id } from "../../functions/form_utils";

    let { label, value, checked, change_value, base_path } = $props();

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
                change_value( base_path, event?.target?.checked || false );
            }
        }
    }

</script>

<div class="flex items-center mt-2">
    <input checked={checked} onchange={local_change_value} id={id} type="checkbox" value={value} class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded-sm focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600" />
    <label for={id} class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">{label}</label>
</div>