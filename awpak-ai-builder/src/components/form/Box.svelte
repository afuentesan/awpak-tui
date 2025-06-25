
<script lang="ts">
    import Checkbox from "./Checkbox.svelte";
    import Header from "./Header.svelte";

    let { title, children } = $props();

    let show_content = $state( true );
</script>

<div class="p-4 bg-white border border-gray-200 rounded-lg shadow-sm dark:bg-gray-800 dark:border-gray-700">
    <Header text={title} />
    <Checkbox checked={show_content} label="Show/hide" value="true" change_value={() => { show_content = ! show_content; } } base_path={undefined} />
    {#if show_content}
        {@render children?.()}
    {/if}
</div>