
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { NodeTypeVariant, type NodeType } from "../../model/node";
    import { add_node_destination, change_node_id, change_node_type } from "../../store";
    import DataToContext from "../data/DataToContext.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import NodeDestination from "../node/NodeDestination.svelte";


    let { node } : { node : NodeType } = $props();

    const node_type_options = select_options_from_enum(
        NodeTypeVariant,
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

    let node_output = node._variant == NodeTypeVariant.Node ? node.output : node.node_output;

    function send_add_node_destination()
    {
        add_node_destination( node.id as string );
    }

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

{#if node._variant == NodeTypeVariant.Node}
    {#each node.destination as dest, i}
    <NodeDestination  destination={dest} />
    {/each}
{/if}

{#if node._variant == NodeTypeVariant.GraphNode}
    {#each node.node_destination as dest, i}
    <NodeDestination  destination={dest} />
    {/each}
{/if}


<Button text="Add node destination" click={send_add_node_destination} />