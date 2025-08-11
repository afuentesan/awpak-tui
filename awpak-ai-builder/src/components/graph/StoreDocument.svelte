
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { StoreDocumentSizerVariant, StoreDocumentVariant, type StoreDocument } from "../../model/store";
    import { change_option_number, change_option_string, change_store_document, change_store_document_sizer } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";


    interface InputProps
    {
        base_path : string,
        document : StoreDocument,
        label : string,
        remove_from_loop? : () => void | undefined
    }

    let { base_path, document, label, remove_from_loop } : InputProps = $props();

</script>

<Box title={label+". "+document._variant} base_path={base_path}>

    <Select 
        label="Document type" 
        options={
            select_options_from_enum(
                StoreDocumentVariant,
                document._variant,
                false
            )
        } 
        value={document._variant} 
        change_value={change_store_document} 
        base_path={base_path} 
    />

    <Input 
        label="Path" 
        value={document.path} 
        change_value={change_option_string} 
        base_path={base_path+".path"} 
    />

    <Select 
        label="Sizer" 
        options={
            select_options_from_enum(
                StoreDocumentSizerVariant,
                document.sizer._variant,
                false
            )
        } 
        value={document.sizer._variant} 
        change_value={change_store_document_sizer} 
        base_path={base_path+".sizer"} 
    />

    {#if 
        document.sizer._variant == StoreDocumentSizerVariant.Chars ||
        document.sizer._variant == StoreDocumentSizerVariant.Markdown
    }
        <div class="flex flex-row">
            <Input 
                label="Desired" 
                input_type="number" 
                value={document.sizer.desired} 
                change_value={change_option_number} 
                base_path={base_path+".sizer.desired"} 
            />
            <Input 
                label="Max" 
                input_type="number" 
                value={document.sizer.max} 
                change_value={change_option_number} 
                base_path={base_path+".sizer.max"} 
            />
        </div>
    {/if}

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove Document" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>