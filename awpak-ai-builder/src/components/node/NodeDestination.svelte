
<script lang="ts">
    import type { NodeDestination } from "../../model/node";
    import DataComparator from "../data/DataComparator.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import NodeNext from "./NodeNext.svelte";

    interface InputProps
    {
        destination : NodeDestination,
        add_exit_text : ( _ : any ) => void,
        base_path : string,
        label : string,
        remove_from_loop? : () => void | undefined
    };

    let { 
        destination,  
        add_exit_text,
        base_path,
        label,
        remove_from_loop
    } : InputProps = $props();
</script>

<Box title={label}>
    <NodeNext 
        node_next={destination.next} 
        add_exit_text={add_exit_text} 
        base_path={base_path+".next"}
    />
    <DataComparator comparator={destination.condition} base_path={base_path+".condition"} />

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove NodeDestination" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>