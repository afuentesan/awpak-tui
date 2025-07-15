
<script lang="ts">
    import Self from './DataFrom.svelte'
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataFromVariant, FromContext, type DataFrom } from "../../model/data";
    import { append_to_array, chage_data_from_variant, change_boolean, change_option_string, remove_from_array, graph, element_from_path, swap_array_items } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Checkbox from "../form/Checkbox.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import DataOperation from "./DataOperation.svelte";
    import TextArea from '../form/TextArea.svelte';

    interface InputProps
    {
        from : DataFrom,
        base_path : string,
        label : string,
        remove_from_loop? : () => void | undefined,
        swap_items_in_array? : ( up : boolean ) => void | undefined
    }
    
    let { from, base_path, label, remove_from_loop, swap_items_in_array } : InputProps = $props();

    // let options_from_variant = $state( select_options_from_enum( DataFromVariant, from._variant, false ) );

    function send_add_item_concat()
    {
        let path = base_path + ".value";

        let new_item_concat = new FromContext();

        append_to_array( path, new_item_concat );
    }

    // $effect(() => {
        
    //     if( ! from?._variant ) return;

    //     let new_from = element_from_path( $graph, base_path );

    //     if( ! new_from ) return;
        
    //     options_from_variant = select_options_from_enum( DataFromVariant, new_from._variant, false );

    //     from = Object.assign( {}, new_from );
    // });
</script>

{#if from?._variant}
<Box title={"DataFrom "+from._variant+". "+label} base_path={base_path}>

    <Select label="From" options={select_options_from_enum( DataFromVariant, from._variant, false )} value={from._variant} change_value={chage_data_from_variant} base_path={base_path} />

    {#if 
        from._variant == DataFromVariant.Context || 
        from._variant == DataFromVariant.ParsedInput
    }
    <Input label="Path" value={from.path} change_value={change_option_string} base_path={base_path+".path"} />
    {/if}

    {#if from._variant == DataFromVariant.Static}
    <TextArea label="Static" value={from.value} change_value={change_option_string} base_path={base_path+".value"} />
    {/if}

    {#if from._variant == DataFromVariant.Operation}
    <DataOperation operation={from.value} base_path={base_path+".value"} />
    {/if}

    {#if from._variant == DataFromVariant.Concat}
        <Box title="Concat items" base_path={base_path+".value"}>
            {#each from.value as _, i}
                <Self
                    label={"Concat "+from.value[i]._variant + " " + i}
                    from={from.value[i]}
                    base_path={base_path+".value["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value", i, ( up ? i - 1 : i + 1 ) );
                        }
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
        {#if typeof( swap_items_in_array ) == "function"}
        <Button text="Up" click={() => swap_items_in_array( true )} color="blue" />
        <Button text="Down" click={() => swap_items_in_array( false )} color="blue" />
        {/if}
    </div>
    {/if}
</Box>
{/if}