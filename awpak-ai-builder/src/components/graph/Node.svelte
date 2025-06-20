
<script lang="ts">
    import { select_options_from_array } from "../../functions/form_utils";
    import { node_variants } from "../../functions/node_functions";
    import type { NodeType } from "../../model/node";
    import { change_node_id, change_node_type } from "../../store";
    import DataToContext from "../data/DataToContext.svelte";
    import DataType from "../data/DataType.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";


    let { node } : { node : NodeType } = $props();

    const node_type_options = select_options_from_array(
        node_variants(),
        node._variant,
        false
    );

    function send_change_node_type( event : any )
    {
        event.preventDefault();

        change_node_type( node.id as string, event?.target?.value );
    }

    function send_change_node_id( event : any )
    {
        event.preventDefault();

        change_node_id( node.id as string, event?.target?.value );
    }

    function send_change_node_output_type( event : any )
    {
        event.preventDefault();

        console.log( "send_change_node_output_type: ", event?.target?.value );
    }

    function send_change_node_output_path( event : any )
    {
        event.preventDefault();

        console.log( "send_change_node_output_path: ", event?.target?.value );
    }

    function send_change_node_output_merge( event : any )
    {
        event.preventDefault();

        console.log( "send_change_node_output_merge: ", event?.target?.value );
    }

    let node_output = node._variant == "Node" ? node.output : node.node_output;

</script>

<Input label="Id" value={node.id} change_value={send_change_node_id} />
<Select label="Node type" options={node_type_options} value={node._variant} change_value={send_change_node_type} />

<DataToContext 
    prefix="Output to context" 
    node_output={node_output} 
    change_node_output_path={send_change_node_output_path}
    change_node_output_type={send_change_node_output_type}
    change_node_output_merge={send_change_node_output_merge}
/>