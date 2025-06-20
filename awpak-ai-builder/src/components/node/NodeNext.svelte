
<script lang="ts">
    import { select_options_from_array } from "../../functions/form_utils";
    import { node_ids } from "../../functions/node_functions";
    import { graph } from "../../store";
    import Select from "../form/Select.svelte";

    let { node_next, change_node_destination } = $props();

    let posible_destinations = node_ids( graph.get() );

    let dest_node_options = ( node_next._variant == "Node" ) ? 
        select_options_from_array( posible_destinations, node_next.value, false ) 
        : 
        undefined;

</script>

{#if node_next._variant == "Node"}
<Select label="Next node" value={node_next.value} options={dest_node_options} change_value={change_node_destination} />
{:else}

{/if}