
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataType } from "../../model/data";
    import Select from "../form/Select.svelte";

    let { label, value, change_value } = $props();

    let data_type_options = select_options_from_enum(
        DataType,
        value,
        true
    );
</script>

<Select label={label} options={data_type_options} value={value} change_value={change_value} />