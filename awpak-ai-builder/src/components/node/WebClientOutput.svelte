
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { WebClientOutputVariant, type WebClientOutput } from "../../model/web_client";
    import { change_option_string, change_web_client_node_output } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import TextArea from "../form/TextArea.svelte";

    interface InputProps
    {
        output : WebClientOutput,
        label : string,
        base_path : string,
        remove_from_loop? : () => void | undefined,
        swap_items_in_array? : ( up : boolean ) => void | undefined
    }

    let { output, label, base_path, remove_from_loop, swap_items_in_array } : InputProps = $props();

</script>

<Box title={label+" "+output._variant} base_path={base_path}>
    <TextArea label="Prefix" value={output.prefix} change_value={change_option_string} base_path={base_path+".prefix"} />

    <Select
        label="WebClient output source" 
        options={
            select_options_from_enum(
                WebClientOutputVariant,
                output._variant,
                false
            )
        } 
        value={output._variant} 
        change_value={change_web_client_node_output} 
        base_path={base_path} 
    />

    {#if output._variant == WebClientOutputVariant.Header}
        <Input
            label="Header name"
            base_path={base_path+".name"}
            change_value={change_option_string}
            value={output.name}
        />
    {/if}

    <TextArea label="Suffix" value={output.suffix} change_value={change_option_string} base_path={base_path+".suffix"} />

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove WebClient node output" click={remove_from_loop} color="red" />
        {#if typeof( swap_items_in_array ) == "function"}
        <Button text="Up" click={() => swap_items_in_array( true )} color="blue" />
        <Button text="Down" click={() => swap_items_in_array( false )} color="blue" />
        {/if}
    </div>
    {/if}
</Box>