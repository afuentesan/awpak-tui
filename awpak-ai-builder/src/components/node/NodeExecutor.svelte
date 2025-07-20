
<script lang="ts">
    import { FromContext, type DataFrom as ExecutorDataFrom } from "../../model/data";
    import { NodeExecutorVariant, type NodeExecutor } from "../../model/node_executor";
    import { append_to_array, change_boolean, change_data_type, change_node_executor_variant, change_option_string, change_parallel_executor_variant, change_request_method, graph, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import DataToContext from "../data/DataToContext.svelte";
    import { DataToContext as ContextMutDataToContext } from "../../model/data";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import { select_options_from_array, select_options_from_enum } from "../../functions/form_utils";
    import Select from "../form/Select.svelte";
    import TextArea from "../form/TextArea.svelte";
    import Checkbox from "../form/Checkbox.svelte";
    import DataToString from "../data/DataToString.svelte";
    import { DataToString as PromptDataToString } from "../../model/data";
    import AIAgentProvider from "./AIAgentProvider.svelte";
    import NodeMcpServer from "./NodeMCPServer.svelte";
    import { NodeMCPServer } from "../../model/agent";
    import DataComparator from "../data/DataComparator.svelte";
    import { DataComparatorTrue } from "../../model/data_comparator";
    import { agent_ids } from "../../functions/node_functions";
    import { DataToAgentHistoryReplace } from "../../model/agent_history_mut";
    import DataToAgentHistory from "../data/DataToAgentHistory.svelte";
    import { ParallelExecutorCommand, ParallelExecutorVariant } from "../../model/parallel";
    import DataType from "../data/DataType.svelte";
    import NodeExecutorWebClient from "./NodeExecutorWebClient.svelte";
    import NodeExecutorCommand from "./NodeExecutorCommand.svelte";

    interface InputProps
    {
        node_executor : NodeExecutor,
        base_path : string
    }

    let { node_executor, base_path } : InputProps = $props();

    function send_add_prompt_part()
    {
        let path = base_path + ".value.prompt";

        let new_input = new PromptDataToString();

        append_to_array( path, new_input );
    }

</script>

<Box title="Node executor" base_path={base_path}>

    <Select 
        label="Executor type" 
        options={
            select_options_from_enum(
                NodeExecutorVariant,
                node_executor._variant,
                false
            )
        } 
        value={node_executor._variant} 
        change_value={change_node_executor_variant} 
        base_path={base_path} 
    />

    {#if node_executor._variant == NodeExecutorVariant.Agent}

        <AIAgentProvider
            provider={node_executor.value.provider}
            base_path={base_path+".value.provider"}
        />

        <TextArea
            label="System prompt"
            value={node_executor.value.system_prompt}
            change_value={change_option_string}
            base_path={base_path+".value.system_prompt"}
        />

        <Box title="Prompt" base_path={base_path+".value.prompt"}>
            {#each node_executor.value.prompt as _, i}
                <DataToString 
                    label={"Prompt part "+i} 
                    data={node_executor.value.prompt[i]} 
                    base_path={base_path+".value.prompt["+i+"]"} 
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.prompt", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value.prompt", i, ( up ? i - 1 : i + 1 ) );
                        }
                    }
                />
            {/each}
            <div class="text-center">
                <Button text="New prompt part" click={send_add_prompt_part} />
            </div>
        </Box>

        <Box title="MCP Servers" base_path={base_path+".value.servers"}>
            {#each node_executor.value.servers as _, i}
                <NodeMcpServer
                    mcp_server={node_executor.value.servers[i]}
                    base_path={base_path+".value.servers["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.servers", i )
                    } 
                />
            {/each}
            <div class="text-center">
                <Button
                    text="Add MCP server"
                    click={
                        () => append_to_array( base_path+".value.servers", new NodeMCPServer() )
                    }
                />
            </div>
        </Box>

        <Checkbox
            label="Save history"
            checked={node_executor.value.save_history}
            change_value={change_boolean}
            base_path={base_path+".value.save_history"}
            value="true"
        />
    {:else if node_executor._variant == NodeExecutorVariant.Command}

        <NodeExecutorCommand
            command={node_executor.value}
            base_path={base_path+".value"}
        />
        
    {:else if node_executor._variant == NodeExecutorVariant.WebClient}

        <NodeExecutorWebClient
            base_path={base_path+".value"}
            web_client={node_executor.value}
        />

    {:else if node_executor._variant == NodeExecutorVariant.ContextMut}
        {#each node_executor.value as _, i}
            <Box title={"ContextMut "+i} base_path={base_path+".value["+i+"]"}>
                <DataFrom 
                    base_path={base_path+".value["+i+"].from"} 
                    from={node_executor.value[i].from as ExecutorDataFrom } 
                    label={"From "+i} 
                />
                <DataToContext
                    base_path={base_path+".value["+i+"].to"}
                    node_output={node_executor.value[i].to}
                    label={"To "+i}
                />
                <DataComparator
                    base_path={base_path+".value["+i+"].condition"}
                    comparator={node_executor.value[i].condition}
                />
                <div class="text-center">
                    <Button
                        text="Remove ContextMut"
                        click={
                            () => remove_from_array( base_path+".value", i )
                        }
                        color="red"
                    />
                    <Button 
                        text="Up" 
                        click={
                            () => swap_array_items( base_path+".value", i, i - 1 )
                        } 
                        color="blue" 
                    />
                    <Button 
                        text="Down" 
                        click={
                            () => swap_array_items( base_path+".value", i, i + 1 )
                        } 
                        color="blue" 
                    />
                </div>
            </Box>
        {/each}
        <div class="text-center">
            <Button
                text="Add ContextMut"
                click={
                    () => append_to_array( 
                        base_path+".value", 
                        {
                            from : new FromContext(),
                            to : new ContextMutDataToContext(),
                            condition : new DataComparatorTrue()
                        }
                    )
                }
            />
        </div>
    {:else if node_executor._variant == NodeExecutorVariant.AgentHistoryMut}
        {#each node_executor.value as _, i}
            <Box title={"AgentHistoryMut "+i} base_path={base_path+".value["+i+"]"}>
                <Select 
                    label="Agent id" 
                    value={node_executor.value[i].id} 
                    options={select_options_from_array( agent_ids( $graph ), node_executor.value[i].id, true )} 
                    change_value={change_option_string}
                    base_path={base_path+".value["+i+"].id"} 
                />

                <DataFrom 
                    base_path={base_path+".value["+i+"].from"} 
                    from={node_executor.value[i].from as ExecutorDataFrom } 
                    label={"From "+i} 
                />
                <DataToAgentHistory
                    base_path={base_path+".value["+i+"].to"}
                    data_to_agent={node_executor.value[i].to}
                />
                <DataComparator
                    base_path={base_path+".value["+i+"].condition"}
                    comparator={node_executor.value[i].condition}
                />
                <div class="text-center">
                    <Button
                        text="Remove ContextMut"
                        click={
                            () => remove_from_array( base_path+".value", i )
                        }
                        color="red"
                    />
                    <Button 
                        text="Up" 
                        click={
                            () => swap_array_items( base_path+".value", i, i - 1 )
                        } 
                        color="blue" 
                    />
                    <Button 
                        text="Down" 
                        click={
                            () => swap_array_items( base_path+".value", i, i + 1 )
                        } 
                        color="blue" 
                    />
                </div>
            </Box>
        {/each}
        <div class="text-center">
            <Button
                text="Add AgentHistoryMut"
                click={
                    () => append_to_array( 
                        base_path+".value", 
                        {
                            id : "",
                            from : new FromContext(),
                            to : new DataToAgentHistoryReplace(),
                            condition : new DataComparatorTrue()
                        }
                    )
                }
            />
        </div>
    {:else if node_executor._variant == NodeExecutorVariant.Parallel}
        {#each node_executor.value.executors as _, i}
            <Box title={"Executor "+i} base_path={base_path+".value.executors["+i+"]"}>
                
                <Select
                    label="Executor type" 
                    options={
                        select_options_from_enum(
                            ParallelExecutorVariant,
                            node_executor.value.executors[ i ]._variant,
                            false
                        )
                    } 
                    value={node_executor.value.executors[ i ]._variant} 
                    change_value={change_parallel_executor_variant} 
                    base_path={base_path+".value.executors["+i+"]"} 
                />
                
                {#if node_executor.value.executors[ i ]._variant == ParallelExecutorVariant.Command}
                    <NodeExecutorCommand
                        base_path={base_path+".value.executors["+i+"].executor"}
                        command={node_executor.value.executors[ i ].executor}
                    />
                {:else if node_executor.value.executors[ i ]._variant == ParallelExecutorVariant.WebClient}
                    <NodeExecutorWebClient
                        base_path={base_path+".value.executors["+i+"].executor"}
                        web_client={node_executor.value.executors[ i ].executor}
                    />
                {/if}

                <DataType 
                    label={"Output type"} 
                    value={node_executor.value.executors[ i ].ty} 
                    change_value={change_data_type} 
                    base_path={base_path+".value.executors["+i+"].ty"} 
                />

                <div class="text-center">
                    <Button
                        text="Remove ParallelExecutor"
                        click={
                            () => remove_from_array( base_path+".value.executors", i )
                        }
                        color="red"
                    />
                    <Button 
                        text="Up" 
                        click={
                            () => swap_array_items( base_path+".value.executors", i, i - 1 )
                        } 
                        color="blue" 
                    />
                    <Button 
                        text="Down" 
                        click={
                            () => swap_array_items( base_path+".value.executors", i, i + 1 )
                        } 
                        color="blue" 
                    />
                </div>
            </Box>
        {/each}
        <div class="text-center">
            <Button
                text="Add Executor"
                click={
                    () => append_to_array( 
                        base_path+".value.executors", 
                        new ParallelExecutorCommand()
                    )
                }
            />
        </div>
    {/if}
</Box>