
<script lang="ts">
    import Self from './DataFrom.svelte'
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataFromVariant, FromContext, type DataFrom } from "../../model/data";
    import { append_to_array, chage_data_from_variant, change_boolean, change_option_string, remove_from_array } from "../../store";
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

    function send_add_item_concat()
    {
        let path = base_path + ".value";

        let new_item_concat = new FromContext();

        append_to_array( path, new_item_concat );
    }
</script>

{#if from}
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
        <Box title="Concat items">
            {#each from.value as _, i}
                <Self
                    label={"Concat "+from.value[i]._variant + " " + i}
                    from={from.value[i]}
                    base_path={base_path+".value["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value", i )
                    } 
                />
            {/each}
            <div class="text-center">
                <Button text="New concat item" click={send_add_item_concat} />
            </div>
        </Box>
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
{/if}