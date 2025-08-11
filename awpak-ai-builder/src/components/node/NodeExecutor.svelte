
<script lang="ts">
    import { FromContext, type DataFrom as ExecutorDataFrom } from "../../model/data";
    import { NodeExecutorVariant, type NodeExecutor } from "../../model/node_executor";
    import { append_to_array, change_data_type, change_node_executor_variant, change_option_string, change_parallel_executor_variant, clone_and_append_to_array, graph, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import DataToContext from "../data/DataToContext.svelte";
    import { DataToContext as ContextMutDataToContext } from "../../model/data";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import { select_options_from_array, select_options_from_enum } from "../../functions/form_utils";
    import Select from "../form/Select.svelte";
    import DataComparator from "../data/DataComparator.svelte";
    import { DataComparatorTrue } from "../../model/data_comparator";
    import { agent_ids } from "../../functions/node_functions";
    import { DataToAgentHistoryReplace } from "../../model/agent_history_mut";
    import DataToAgentHistory from "../data/DataToAgentHistory.svelte";
    import { ParallelExecutorCommand, ParallelExecutorVariant } from "../../model/parallel";
    import DataType from "../data/DataType.svelte";
    import NodeExecutorWebClient from "./NodeExecutorWebClient.svelte";
    import NodeExecutorCommand from "./NodeExecutorCommand.svelte";
    import NodeExecutorAgent from "./NodeExecutorAgent.svelte";
    import NodeExecutorGraph from "./NodeExecutorGraph.svelte";

    interface InputProps
    {
        node_executor : NodeExecutor,
        base_path : string
    }

    let { node_executor, base_path } : InputProps = $props();

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

        <NodeExecutorAgent
            agent={node_executor.value}
            base_path={base_path+".value"}
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
    
    {:else if node_executor._variant == NodeExecutorVariant.Graph}

        <NodeExecutorGraph
            graph_executor={node_executor.value}
            base_path={base_path+".value"}
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
                
                <DataComparator
                    base_path={base_path+".value.executors["+i+"].condition"}
                    comparator={node_executor.value.executors[ i ].condition}
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
                        text="Clone"
                        click={
                            () => clone_and_append_to_array( base_path+".value.executors", node_executor.value.executors[ i ] )
                        }
                        color="purple"
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