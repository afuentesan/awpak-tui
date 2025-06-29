
<script lang="ts">
    import { select_options_from_array, select_options_from_enum } from "../../functions/form_utils";
    import { node_ids } from "../../functions/node_functions";
    import { NodeNextVariant, type NodeNext } from "../../model/node";
    import { change_next_step, change_option_string, graph, remove_from_array } from "../../store";
    import DataToString from "../data/DataToString.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Select from "../form/Select.svelte";

    interface InputProps
    {
        node_next : NodeNext,
        add_exit_text : ( _ : any ) => void,
        base_path : string,
        is_grid? : boolean
    }

    let { 
        node_next, 
        add_exit_text,
        base_path,
        is_grid
    } : InputProps = $props();

    let node_next_options = select_options_from_enum( NodeNextVariant, node_next._variant, false );

</script>

<Box title={"NodeNext "+node_next._variant} is_grid={is_grid}>

    <Select label="Next step" value={node_next._variant} options={node_next_options} change_value={change_next_step} base_path={base_path} />

    {#if node_next._variant == NodeNextVariant.Node}
    <Select 
        label="Next node id" 
        value={node_next.value} 
        options={select_options_from_array( node_ids( $graph ), node_next.value, true )} 
        change_value={change_option_string}
        base_path={base_path+".value"} 
    />
    {/if}

    {#if node_next._variant == NodeNextVariant.ExitOk || node_next._variant == NodeNextVariant.ExitErr}
    {#each node_next.value as _, i}
        <DataToString 
            data={node_next.value[i]} 
            base_path={base_path+".value[" + i + "]"} 
            label={node_next._variant + " " + i} 
            remove_from_loop={
                () => remove_from_array( base_path+".value", i )
            }
        />
    {/each}
    <div class="text-center">
        <Button text="Add exit text" click={add_exit_text} />
    </div>
    {/if}

</Box>