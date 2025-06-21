
<script lang="ts">
    import { select_options_from_array } from "../../functions/form_utils";
    import { node_ids } from "../../functions/node_functions";
    import { NodeNextVariant } from "../../model/node";
    import { graph } from "../../store";
    import DataToString from "../data/DataToString.svelte";
    import Select from "../form/Select.svelte";

    let { node_next, change_node_destination } = $props();

    let posible_destinations = node_ids( graph.get() );

    let dest_node_options = ( node_next._variant == NodeNextVariant.Node ) ? 
        select_options_from_array( posible_destinations, node_next.value, false ) 
        : 
        undefined;

</script>

{#if node_next._variant == NodeNextVariant.Node}
<Select label="Next node" value={node_next.value} options={dest_node_options} change_value={change_node_destination} />
{/if}

{#if node_next._variant == NodeNextVariant.ExitOk || node_next._variant == NodeNextVariant.ExitErr}
{#each node_next.value as data, i}
<DataToString data={data} />
{/each}
{/if}