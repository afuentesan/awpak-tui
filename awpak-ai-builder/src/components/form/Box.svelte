
<script lang="ts">
    import Checkbox from "./Checkbox.svelte";
    import Header from "./Header.svelte";

    interface InputProps
    {
        title : string,
        children : any,
        is_grid? : boolean
    }

    let { title, children, is_grid } : InputProps = $props();

    let show_content = $state( true );
</script>

<div class="p-4 bg-white border border-gray-200 rounded-lg shadow-sm dark:bg-gray-800 dark:border-gray-700">
    <Header text={title} />
    {#if ! is_grid}
        <Checkbox checked={show_content} label="Show/hide" value="true" change_value={() => { show_content = ! show_content; } } base_path={undefined} />
    {/if}
    {#if show_content}
        {#if is_grid}
            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8 p-2">
                {@render children?.()}
            </div>
        {:else}
            {@render children?.()}
        {/if}
    {/if}
</div>