
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataOperationVariant } from "../../model/data";
    import { change_data_operation, change_option_string } from "../../store";
    import Box from "../form/Box.svelte";
    import Select from "../form/Select.svelte";
    import TextArea from "../form/TextArea.svelte";
    import DataFrom from "./DataFrom.svelte";

    let { operation, base_path } = $props();

    // let data_type_options = select_options_from_enum(
    //     DataOperationVariant,
    //     operation._variant,
    //     false
    // );
</script>

<Box title={"Operation " + operation._variant} base_path={base_path}>

    <Select 
        label="Operation" 
        options={
            select_options_from_enum(
                DataOperationVariant,
                operation._variant,
                false
            )
        } 
        value={operation._variant} 
        change_value={change_data_operation} 
        base_path={base_path} 
    />

    {#if operation._variant == DataOperationVariant.Len}
    <DataFrom from={operation.value} base_path={base_path+".value"} label={operation._variant + " value"} />
    {/if}

    {#if operation._variant == DataOperationVariant.Add || operation._variant == DataOperationVariant.Substract}
    <DataFrom from={operation.num_1} base_path={base_path+".num_1"} label={operation._variant + " num_1"} />
    <DataFrom from={operation.num_2} base_path={base_path+".num_2"} label={operation._variant + " num_2"} />
    {/if}

    {#if operation._variant == DataOperationVariant.StringSplit}
    <DataFrom from={operation.from} base_path={base_path+".from"} label={operation._variant + " from"} />
    <TextArea
        base_path={base_path+".sep"} 
        label={operation._variant + " separator"}
        change_value={change_option_string}
        value={operation.sep}
    />
    {/if}
</Box>