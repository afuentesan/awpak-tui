
<script lang="ts">
    import { NodeNextVariant, type NodeDestination } from "../../model/node";
    import DataComparator from "../data/DataComparator.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import NodeNext from "./NodeNext.svelte";

    interface InputProps
    {
        destination : NodeDestination,
        add_exit_text : ( _ : any ) => void,
        base_path : string,
        from? : string,
        label? : string,
        remove_from_loop? : () => void | undefined,
        is_grid? : boolean
    };

    let { 
        destination,  
        add_exit_text,
        base_path,
        from,
        label,
        remove_from_loop,
        is_grid
    } : InputProps = $props();

    const label_str = () =>
    {
        if( label?.trim() ) return label;

        let ret = from?.trim() ? ( "From " + from + " " ) : "";

        ret += "To ";

        if( 
            destination.next?._variant == NodeNextVariant.ExitOk || 
            destination.next?._variant == NodeNextVariant.ExitErr )
        {
            ret += destination.next._variant;
        }
        else
        {
            ret += ( destination.next?.value || "" );
        }

        return ret;
    };

</script>

<Box title={label_str()}>

    {#if destination.next}
    <NodeNext 
        node_next={destination.next} 
        add_exit_text={add_exit_text} 
        base_path={base_path+".next"}
        is_grid={is_grid}
    />
    {/if}

    {#if destination.condition}
    <DataComparator comparator={destination.condition} base_path={base_path+".condition"} is_grid={is_grid} />
    {/if}

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove NodeDestination" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>