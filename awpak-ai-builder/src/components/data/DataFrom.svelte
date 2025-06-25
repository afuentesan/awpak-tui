
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataFromVariant, type DataFrom } from "../../model/data";
    import { chage_data_from_variant, change_boolean, change_option_string } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Checkbox from "../form/Checkbox.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import DataOperation from "./DataOperation.svelte";

    interface InputProps
    {
        from : DataFrom,
        base_path : string,
        label : string,
        remove_from_loop? : () => void | undefined
    }
    
    let { from, base_path, label, remove_from_loop } : InputProps = $props();

    let options_from_variants = select_options_from_enum( DataFromVariant, from._variant, false );
</script>

<Box title={"DataFrom "+from._variant+". "+label}>

    <Select label="From" options={options_from_variants} value={from._variant} change_value={chage_data_from_variant} base_path={base_path} />

    {#if 
        from._variant == DataFromVariant.Context || 
        from._variant == DataFromVariant.ParsedInput
    }
    <Input label="Path" value={from.path} change_value={change_option_string} base_path={base_path+".path"} />
    {/if}

    {#if from._variant == DataFromVariant.Static}
    <Input label="Static" value={from.value} change_value={change_option_string} base_path={base_path+".value"} />
    {/if}

    {#if from._variant == DataFromVariant.Operation}
    <DataOperation operation={from.value} base_path={base_path+".value"} />
    {/if}

    {#if from._variant == DataFromVariant.Concat}
    <p>TODO: Concat</p>
    {/if}

    {#if 
        from._variant == DataFromVariant.Context || 
        from._variant == DataFromVariant.ParsedInput ||
        from._variant == DataFromVariant.Input
    }
    <Checkbox label="Required" value="true" checked={from.required} change_value={change_boolean} base_path={base_path+".required"} />
    {/if}
    
    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove DataFrom" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>