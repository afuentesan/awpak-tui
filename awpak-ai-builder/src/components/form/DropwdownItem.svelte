
<script lang="ts">
    import { random_id } from "../../functions/form_utils";


    interface InputProps
    {
        label : string,
        click : ( event : any ) => void,
        is_file? : boolean
    }

    let { label, click, is_file } : InputProps = $props();

    let id = random_id();

    let item_cls = "block cursor-pointer w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white";
</script>

<li>
    {#if is_file}
    <label 
        for={id}
        class={item_cls}
    >
        {label}
    </label>

    <input 
        type="file" 
        style="display:none;" 
        id={id}
        onchange={click}
    />
    {:else}
    <button 
        class={item_cls}
        onclick={click}
    >{label}</button>
    {/if}
</li>