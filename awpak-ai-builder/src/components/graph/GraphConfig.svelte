
<script lang="ts">
    import { change_input_data_type, change_preserve_context, graph } from "../../store";
    import Checkbox from "../form/Checkbox.svelte";
    import DataType from "../data/DataType.svelte";

    function send_change_preserve_context( event : any )
    {
        event.preventDefault();

        change_preserve_context( event.target.checked || false );
    }

    function send_change_input_data_type( event : any )
    {
        event.preventDefault();

        change_input_data_type( event.target.value );
    }

    console.log( "Reload graph config" );

</script>

<form class="space-y-6" action="#">
    <p>{$graph.input_type}</p>
    <DataType label="Input data type" value={$graph.input_type} change_value={send_change_input_data_type} />
    <Checkbox label="Preserve context" value="true" checked={$graph.preserve_context} change_value={send_change_preserve_context} />    
</form>