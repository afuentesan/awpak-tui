
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataType } from "../../model/data";
    import Select from "../form/Select.svelte";

    let { label, value, change_value, base_path } = $props();

</script>

<Select 
    label={label} 
    options={
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