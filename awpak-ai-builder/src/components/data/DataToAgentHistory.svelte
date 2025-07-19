
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataToAgentHistoryVariant, type DataToAgentHistory } from "../../model/agent_history_mut";
    import { change_data_to_agent_history, change_option_number } from "../../store";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";


    interface InputProps
    {
        data_to_agent : DataToAgentHistory,
        base_path : string
    }

    let { data_to_agent, base_path } : InputProps = $props();
</script>

<Select
    label="Target" 
    options={select_options_from_enum( DataToAgentHistoryVariant, data_to_agent._variant, false )} 
    value={data_to_agent._variant} 
    change_value={change_data_to_agent_history} 
    base_path={base_path}
/>

{#if 
    data_to_agent._variant == DataToAgentHistoryVariant.ReplaceItem ||
    data_to_agent._variant == DataToAgentHistoryVariant.StringToItem
}
    <Input 
        label="Index" 
        input_type="number" 
        value={data_to_agent.value} 
        change_value={change_option_number} 
        base_path={base_path+".value"} />
{/if}