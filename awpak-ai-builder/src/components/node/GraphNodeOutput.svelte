
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { GraphNodeOutputVariant, type GraphNodeOutput } from "../../model/node"
    import { change_graph_node_output, change_option_string } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";

    interface InputProps
    {
        label : string,
        graph_node_output : GraphNodeOutput,
        base_path : string,
        remove_from_loop? : () => void | undefined
    }

    let { label, graph_node_output, base_path, remove_from_loop } : InputProps = $props();
    
</script>

<Box title={label}>

    <Input label="Prefix" value={graph_node_output.prefix} change_value={change_option_string} base_path={base_path+".prefix"} />

    <Select 
        label="Graph node output source" 
        options={
            select_options_from_enum(
                GraphNodeOutputVariant,
                graph_node_output._variant,
                false
            )
        } 
        value={graph_node_output._variant} 
        change_value={change_graph_node_output} 
        base_path={base_path} 
    />

    <Input label="Suffix" value={graph_node_output.suffix} change_value={change_option_string} base_path={base_path+".suffix"} />

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove graph node output" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>

