
<script lang="ts">
    import type { DataToContext } from "../../model/data";
    import { change_boolean, change_data_type, change_option_string } from "../../store";
    import Box from "../form/Box.svelte";
    import Checkbox from "../form/Checkbox.svelte";
    import Input from "../form/Input.svelte";
    import DataMerge from "./DataMerge.svelte";
    import DataType from "./DataType.svelte";
    import { DataType as DT } from "../../model/data";

    interface InputProps
    {
        label : string,
        node_output : DataToContext | undefined,
        base_path : string,
        allowed_types? : Array<DT>
    }

    let { 
        label, 
        node_output, 
        base_path,
        allowed_types
    } : InputProps = $props();
</script>

<Box title={label} base_path={base_path}>

    <Input label={"Path"} value={node_output?.path} change_value={change_option_string} base_path={base_path+".path"} />
    <DataType 
        label={"Type"} 
        value={node_output?.ty} 
        change_value={change_data_type} 
        base_path={base_path+".ty"} 
        allowed_types={allowed_types}
    />
    <DataMerge label={"Merge"} value={node_output?.merge} base_path={base_path+".merge"} />
    <Checkbox 
        label="Optional" 
        value="true" 
        change_value={change_boolean} 
        base_path={base_path+".optional"}
        checked={node_output?.optional}
    />

</Box>