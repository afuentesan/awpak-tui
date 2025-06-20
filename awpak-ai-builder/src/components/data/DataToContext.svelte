
<script lang="ts">
    import Input from "../form/Input.svelte";
    import DataMerge from "./DataMerge.svelte";
    import DataType from "./DataType.svelte";

    let { 
        prefix, 
        node_output, 
        change_node_output_path, 
        change_node_output_type,
        change_node_output_merge
    } = $props();
</script>

<Input label={prefix + " path"} value={node_output?.path} change_value={change_node_output_path} />
<DataType label={prefix + " type"} value={node_output?.ty} change_value={change_node_output_type} />
<DataMerge label={prefix + " merge"} value={node_output?.merge} change_value={change_node_output_merge} />