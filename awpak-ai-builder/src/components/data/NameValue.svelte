
<script lang="ts">
    import type { WebClientNameValue } from "../../model/web_client";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import DataFrom from "./DataFrom.svelte";

    interface InputProps
    {
        name_value : WebClientNameValue,
        base_path : string,
        label : string,
        remove_from_loop? : () => void | undefined,
        swap_items_in_array? : ( up : boolean ) => void | undefined
    }

    let { name_value, base_path, label, remove_from_loop, swap_items_in_array } : InputProps = $props();

</script>

<Box title={label} base_path={base_path}>

    <DataFrom
        from={name_value.name}
        label="Name"
        base_path={base_path+".name"}
    />

    <DataFrom
        from={name_value.value}
        label="Value"
        base_path={base_path+".value"}
    />

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text={"Remove "+label} click={remove_from_loop} color="red" />
        {#if typeof( swap_items_in_array ) == "function"}
        <Button text="Up" click={() => swap_items_in_array( true )} color="blue" />
        <Button text="Down" click={() => swap_items_in_array( false )} color="blue" />
        {/if}
    </div>
    {/if}
</Box>