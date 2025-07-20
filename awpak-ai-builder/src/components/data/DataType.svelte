
<script lang="ts">
    import { select_options_from_array, select_options_from_enum } from "../../functions/form_utils";
    import { DataType } from "../../model/data";
    import Select from "../form/Select.svelte";

    interface InputProps
    {
        label : string,
        value : any,
        change_value : any,
        base_path : string,
        allowed_types? : Array<DataType>
    }

    let { label, value, change_value, base_path, allowed_types } : InputProps = $props();

</script>

<Select 
    label={label} 
    options={
        allowed_types?.length
        ?
        select_options_from_array(
            allowed_types,
            value,
            true
        )
        :
        select_options_from_enum(
            DataType,
            value,
            true
        )
    } 
    value={value} 
    change_value={change_value} 
    base_path={base_path} 
/>