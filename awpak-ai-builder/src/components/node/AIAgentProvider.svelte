
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { AIAgentProviderConfigVariant, type AIAgentProvider } from "../../model/agent";
    import { change_option_number, change_option_string, change_provider_variant } from "../../store";
    import Box from "../form/Box.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";

    
    interface InputProps
    {
        provider : AIAgentProvider,
        base_path : string
    }

    let { provider, base_path } : InputProps = $props();
    
</script>

<Box title={"Provider "+provider._variant} base_path={base_path}>

    <Select 
        label="Provider type" 
        options={
            select_options_from_enum(
                AIAgentProviderConfigVariant,
                provider._variant,
                false
            )
        } 
        value={provider._variant} 
        change_value={change_provider_variant} 
        base_path={base_path} 
    />

    {#if 
        provider._variant == AIAgentProviderConfigVariant.OpenAI ||
        provider._variant == AIAgentProviderConfigVariant.Anthropic ||
        provider._variant == AIAgentProviderConfigVariant.Gemini ||
        provider._variant == AIAgentProviderConfigVariant.DeepSeek
    }
        <Input
            label="Api key"
            value={provider.api_key}
            change_value={change_option_string}
            base_path={base_path+".api_key"}
        />
    {/if}

    <Input
        label="Model"
        value={provider.model}
        change_value={change_option_string}
        base_path={base_path+".model"}
    />

    {#if 
        provider._variant == AIAgentProviderConfigVariant.Anthropic ||
        provider._variant == AIAgentProviderConfigVariant.DeepSeek
    }
        <Input
            label="Max tokens"
            value={provider.max_tokens}
            change_value={change_option_number}
            base_path={base_path+".max_tokens"}
            input_type="number"
        />
    {/if}
</Box>