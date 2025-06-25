
<script lang="ts">
    import { change_input_data_type, change_map_value, change_preserve_context, graph } from "../../store";
    import Checkbox from "../form/Checkbox.svelte";
    import DataType from "../data/DataType.svelte";
    import TextArea from "../form/TextArea.svelte";

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

</script>


<DataType label="Input data type" value={$graph.input_type} change_value={send_change_input_data_type} base_path={undefined} />
<TextArea label="Initial context" value={JSON.stringify( $graph.context )} change_value={change_map_value} base_path="$.context" />
<Checkbox label="Preserve context" value="true" checked={$graph.preserve_context} change_value={send_change_preserve_context} base_path={undefined} />
