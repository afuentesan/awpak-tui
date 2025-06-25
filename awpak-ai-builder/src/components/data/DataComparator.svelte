
<script lang="ts">
    import { DataComparatorVariant } from "../../model/data_comparator";
    import DataFrom from "./DataFrom.svelte";
    import Self from './DataComparator.svelte'
    import Text from "../form/Text.svelte";
    import { select_options_from_enum } from "../../functions/form_utils";
    import Select from "../form/Select.svelte";
    import { chage_data_comparator, change_option_string } from "../../store";
    import Input from "../form/Input.svelte";
    import Box from "../form/Box.svelte";

    let { comparator, base_path } = $props();

    let data_type_options = select_options_from_enum(
        DataComparatorVariant,
        comparator._variant,
        false
    );
</script>

<Box title={"Comparator "+comparator._variant}>

    <Select 
        label="Comparator type" 
        options={data_type_options} 
        value={comparator._variant} 
        change_value={chage_data_comparator} 
        base_path={base_path} 
    />

    {#if 
        comparator._variant == DataComparatorVariant.Eq ||
        comparator._variant == DataComparatorVariant.NotEq ||
        comparator._variant == DataComparatorVariant.Gt ||
        comparator._variant == DataComparatorVariant.Lt
    }
    <DataFrom from={comparator.from_1} base_path={base_path+".from_1"} label={comparator._variant + " from_1"} />
    <DataFrom from={comparator.from_2} base_path={base_path+".from_2"} label={comparator._variant + " from_2"} />
    {/if}

    {#if 
        comparator._variant == DataComparatorVariant.And ||
        comparator._variant == DataComparatorVariant.Or
    }
        <Self comparator={comparator.comp_1} base_path={base_path+".comp_1"} />
        <Self comparator={comparator.comp_2} base_path={base_path+".comp_2"} />
    {/if}

    {#if 
        comparator._variant == DataComparatorVariant.Not
    }
        <Self comparator={comparator.value} base_path={base_path+".value"} />
    {/if}

    {#if comparator._variant == DataComparatorVariant.True}
        <Text text="True" />
    {/if}

    {#if comparator._variant == DataComparatorVariant.False}
        <Text text="False" />
    {/if}

    {#if comparator._variant == DataComparatorVariant.Regex}
        <Input label="Regex" value={comparator.regex} change_value={change_option_string} base_path={base_path+".regex"} />
        <DataFrom from={comparator.from} base_path={base_path+".from"} label={comparator._variant + " from"} />
    {/if}

</Box>