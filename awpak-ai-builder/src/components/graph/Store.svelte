
<script lang="ts">
    import { is_empty } from "../../functions/data_functions";
    import { select_options_from_enum } from "../../functions/form_utils";
    import { StoreDocumentText, StoreModelVariant, StoreProvider, type StoreConfig } from "../../model/store";
    import { change_store_model, change_store_provider, change_store_id, remove_from_array, change_option_string, append_to_array } from "../../store";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import StoreDocument from "./StoreDocument.svelte";


    interface InputProps
    {
        store : StoreConfig,
        base_path : string
    }

    let { store, base_path } : InputProps = $props();

    function store_idx() : number | undefined
    {
        if( ! base_path?.startsWith( "$.stores[" ) ) return undefined;

        let str_idx = base_path.replace( /^\$.stores\[/, "" ).replace( /\].*$/, "" );

        if( is_empty( str_idx?.trim() ) || str_idx.trim() === "" ) return;

        try
        {
            let idx = Number( str_idx.trim() );

            if( is_empty( idx ) || isNaN( idx ) ) return undefined;

            return idx;
        }
        catch( _e )
        {
            return undefined;
        }
    }

    function remove_store()
    {
        let idx = store_idx();

        if( is_empty( idx ) ) return;

        remove_from_array( "$.stores", idx as number );
    }

    function send_change_store_id( event : any )
    {
        event.preventDefault();

        change_store_id( store.id, event?.target?.value );
    }
</script>

<Box title={"Store " + store.id} base_path={base_path}>

    <div>
        <Input label="Id" value={store.id} change_value={send_change_store_id} base_path={undefined} />

        <Select 
            label="Provider" 
            options={
                select_options_from_enum(
                    StoreProvider,
                    store.provider,
                    false
                )
            } 
            value={store.provider} 
            change_value={change_store_provider} 
            base_path={base_path+".provider"} 
        />

        <Box title="Model provider" base_path={base_path+".model"}>
            <Select 
                label="Model provider" 
                options={
                    select_options_from_enum(
                        StoreModelVariant,
                        store.model._variant,
                        false
                    )
                } 
                value={store.model._variant} 
                change_value={change_store_model} 
                base_path={base_path+".model"} 
            />

            <Input 
                label="Model" 
                value={store.model.model} 
                change_value={change_option_string} 
                base_path={base_path+".model.model"} 
            />

            {#if 
                store.model._variant == StoreModelVariant.Gemini ||
                store.model._variant == StoreModelVariant.OpenAI
            }
                <Input 
                    label="Api key" 
                    value={store.model.api_key} 
                    change_value={change_option_string} 
                    base_path={base_path+".model.api_key"} 
                />
            {/if}

        </Box>

        <Box title="Documents">
            {#each store.documents as _, i}
                <StoreDocument
                    base_path={base_path+".documents["+i+"]"}
                    document={store.documents[i]}
                    label={"Document " + ( i + 1 )}
                    remove_from_loop={
                        () => remove_from_array( base_path+".documents", i )
                    }
                />
            {/each}
            <div class="text-center">
                <Button text="Add document" click={() => append_to_array( base_path+".documents", new StoreDocumentText() )} />
            </div>
        </Box>

        <div class="text-center">
            <Button text="Remove Store" click={() => remove_store()} color="red" />
        </div>

    </div>
    
</Box>