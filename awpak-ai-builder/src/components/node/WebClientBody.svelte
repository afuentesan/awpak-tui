
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { WebClientBodyVariant, WebClientNameValue, type WebClientBody } from "../../model/web_client";
    import { append_to_array, chage_data_body_variant, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import NameValue from "../data/NameValue.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Select from "../form/Select.svelte";


    interface InputProps
    {
        body : WebClientBody | undefined,
        label : string,
        base_path : string
    }

    let { body, label, base_path } : InputProps = $props();

</script>

<Box title={label} base_path={base_path}>

    <Select label="Body type" options={select_options_from_enum( WebClientBodyVariant, body?._variant, true )} value={body?._variant} change_value={chage_data_body_variant} base_path={base_path} />

    {#if body}
        {#if body._variant == WebClientBodyVariant.Json}
            <DataFrom
                from={body.value}
                label="Json body"
                base_path={base_path+".value"}
            />
        {:else if body._variant == WebClientBodyVariant.Form}
            <Box title="Form fields" base_path={base_path+".value"}>
                {#each body.value as _, i}
                    <NameValue
                        label={"Field "+i}
                        name_value={body.value[i]}
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
                    <Button
                        text="Add field"
                        click={
                            () => append_to_array( base_path+".value", new WebClientNameValue() )
                        }
                    />
                </div>
            </Box>
        {/if}
    {/if}
</Box>
