
<script lang="ts">
    import { random_id } from "../../functions/form_utils";

    let { label, value, input_type = undefined, change_value } = $props();

    if( ! input_type?.trim() ) { input_type = "text"; }

    let id = random_id();

</script>

<div>
    <label for={id} class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{label}</label>
    <input 
        type={input_type} 
        id={id} 
        value={value}
        onchange={change_value}
        onblur={change_value}
        class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" 
    />
</div>