
<script lang="ts">
    import { change_box_state, is_box_hidden } from "../../show_hide_store";
    import Checkbox from "./Checkbox.svelte";
    import Header from "./Header.svelte";

    interface InputProps
    {
        title : string,
        children : any,
        is_grid? : boolean,
        base_path? : string
    }

    let { title, children, is_grid, base_path } : InputProps = $props();

    let show_content = $state( ! is_box_hidden( base_path ) );

    show_content = ! is_box_hidden( base_path );

</script>

<div class="p-4 bg-white border border-gray-200 rounded-lg shadow-sm dark:bg-gray-800 dark:border-gray-700 box">
    <Header text={title} />
    {#if ! is_grid}
        <Checkbox 
            checked={! is_box_hidden( base_path )} 
            label="Show/hide" 
            value="true" 
            change_value={
                ( event : any ) => { 
                    show_content = event?.target?.checked || false;
                    change_box_state( base_path, ! show_content );
                } 
            } 
            base_path={undefined} />
    {/if}
    {#if show_content || is_grid}
        {#if is_grid}
            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 p-2">
                {@render children?.()}
            </div>
        {:else}
            {@render children?.()}
        {/if}
    {/if}
</div>