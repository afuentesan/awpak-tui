<script lang="ts">
    import { AIAgentProviderConfigVariant, NodeMCPServer, type AIAgent } from "../../model/agent";
    import { append_to_array, change_boolean, remove_from_array, swap_array_items } from "../../store";
    import DataToString from "../data/DataToString.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Checkbox from "../form/Checkbox.svelte";
    import AiAgentProvider from "./AIAgentProvider.svelte";
    import NodeMcpServer from "./NodeMCPServer.svelte";
    import { DataToString as PromptDataToString } from "../../model/data";

    interface InputProps
    {
        agent : AIAgent,
        base_path : string
    }

    let { agent, base_path } : InputProps = $props();

    function send_add_prompt_part( key : string )
    {
        return () => {
            let path = base_path + "." + key;

            let new_input = new PromptDataToString();

            append_to_array( path, new_input );
        }
        
    }

</script>

<AiAgentProvider
    provider={agent.provider}
    base_path={base_path+".provider"}
/>

<Box title="System prompt" base_path={base_path+".system_prompt"}>
    {#each agent.system_prompt as _, i}
        <DataToString 
            label={"System prompt part "+i} 
            data={agent.system_prompt[i]} 
            base_path={base_path+".system_prompt["+i+"]"} 
            remove_from_loop={
                () => remove_from_array( base_path+".system_prompt", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".system_prompt", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button text="New system prompt part" click={send_add_prompt_part( "system_prompt" )} />
    </div>
</Box>

<Box title="Prompt" base_path={base_path+".prompt"}>
    {#each agent.prompt as _, i}
        <DataToString 
            label={"Prompt part "+i} 
            data={agent.prompt[i]} 
            base_path={base_path+".prompt["+i+"]"} 
            remove_from_loop={
                () => remove_from_array( base_path+".prompt", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".prompt", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button text="New prompt part" click={send_add_prompt_part( "prompt" )} />
    </div>
</Box>

<Box title="MCP Servers" base_path={base_path+".servers"}>
    {#each agent.servers as _, i}
        <NodeMcpServer
            mcp_server={agent.servers[i]}
            base_path={base_path+".servers["+i+"]"}
            remove_from_loop={
                () => remove_from_array( base_path+".servers", i )
            } 
        />
    {/each}
    <div class="text-center">
        <Button
            text="Add MCP server"
            click={
                () => append_to_array( base_path+".servers", new NodeMCPServer() )
            }
        />
    </div>
</Box>

<Checkbox
    label="Save history"
    checked={agent.save_history}
    change_value={change_boolean}
    base_path={base_path+".save_history"}
    value="true"
/>

<Checkbox
    label="Streaming response"
    checked={agent.is_stream}
    change_value={change_boolean}
    base_path={base_path+".is_stream"}
    value="true"
/>