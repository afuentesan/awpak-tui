
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataMerge } from "../../model/data";
    import { change_data_merge } from "../../store";
    import Select from "../form/Select.svelte";

    let { label, value, base_path } = $props();

    let data_merge_options = select_options_from_enum(
        DataMerge,
        value,
        true
    );
</script>

<Select 
    label={label} 
    options={
        select_options_from_enum(
            DataMerge,
            value,
            true
        )
    } 
    value={value} 
    change_value={change_data_merge} 
    base_path={base_path} 
/>