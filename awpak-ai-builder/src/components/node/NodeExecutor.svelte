
<script lang="ts">
    import { FromContext, type DataFrom as ExecutorDataFrom } from "../../model/data";
    import { NodeExecutorVariant, type NodeExecutor } from "../../model/node_executor";
    import { append_to_array, change_boolean, change_node_executor_variant, change_option_string, change_request_method, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import DataToContext from "../data/DataToContext.svelte";
    import { DataToContext as ContextMutDataToContext } from "../../model/data";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import { select_options_from_enum } from "../../functions/form_utils";
    import Select from "../form/Select.svelte";
    import CommandOutput from "./CommandOutput.svelte";
    import { CommandOutputOut } from "../../model/command";
    import TextArea from "../form/TextArea.svelte";
    import Checkbox from "../form/Checkbox.svelte";
    import DataToString from "../data/DataToString.svelte";
    import { DataToString as PromptDataToString } from "../../model/data";
    import AIAgentProvider from "./AIAgentProvider.svelte";
    import NodeMcpServer from "./NodeMCPServer.svelte";
    import { NodeMCPServer } from "../../model/agent";
    import { AwpakMethod, WebClientNameValue, WebClientOutputStatus } from "../../model/web_client";
    import NameValue from "../data/NameValue.svelte";
    import WebClientBody from "./WebClientBody.svelte";
    import WebClientOutput from "./WebClientOutput.svelte";


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
        <Input
            label="Command"
            value={node_executor.value.command}
            change_value={change_option_string}
            base_path={base_path+".value.command"}
        />
        <Box title="Arguments" base_path={base_path+".value.args"}>
            {#each node_executor.value.args as _, i}
                <DataFrom
                    label={"Arg "+i}
                    from={node_executor.value.args[i]}
                    base_path={base_path+".value.args["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.args", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value.args", i, ( up ? i - 1 : i + 1 ) );
                        }
                    }
                />
            {/each}
            <div class="text-center">
                <Button
                    text="Add argument"
                    click={
                        () => append_to_array( base_path+".value.args", new FromContext() )
                    }
                />
            </div>
        </Box>
        <Box title="Command output" base_path={base_path+".value.output"}>
            {#each node_executor.value.output as _, i}
                <CommandOutput
                    label={"CommandOutput "+i}
                    output={node_executor.value.output[i]}
                    base_path={base_path+".value.output["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.output", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value.output", i, ( up ? i - 1 : i + 1 ) );
                        }
                    }
                />
            {/each}
            <div class="text-center">
                <Button
                    text="Add output"
                    click={
                        () => append_to_array( base_path+".value.output", new CommandOutputOut() )
                    }
                />
            </div>
        </Box>
    {:else if node_executor._variant == NodeExecutorVariant.WebClient}
        <DataFrom
            from={node_executor.value.url}
            label="URL"
            base_path={base_path+".value.url"}
        />
        <Select
            label="Method"
            base_path={base_path+".value.method"}
            change_value={change_request_method}
            options={
                select_options_from_enum(
                    AwpakMethod,
                    node_executor.value.method,
                    false
                )
            }
            value={node_executor.value.method}
        />

        <Box title="Headers" base_path={base_path+".value.headers"}>
            {#each node_executor.value.headers as _, i}
                <NameValue
                    label={"Header "+i}
                    name_value={node_executor.value.headers[i]}
                    base_path={base_path+".value.headers["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.headers", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value.headers", i, ( up ? i - 1 : i + 1 ) );
                        }
                    }
                />
            {/each}
            <div class="text-center">
                <Button
                    text="Add header"
                    click={
                        () => append_to_array( base_path+".value.headers", new WebClientNameValue() )
                    }
                />
            </div>
        </Box>

        <Box title="Query params" base_path={base_path+".value.query_params"}>
            {#each node_executor.value.headers as _, i}
                <NameValue
                    label={"Param "+i}
                    name_value={node_executor.value.headers[i]}
                    base_path={base_path+".value.query_params["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.query_params", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value.query_params", i, ( up ? i - 1 : i + 1 ) );
                        }
                    }
                />
            {/each}
            <div class="text-center">
                <Button
                    text="Add query param"
                    click={
                        () => append_to_array( base_path+".value.query_params", new WebClientNameValue() )
                    }
                />
            </div>
        </Box>

        <WebClientBody
            label="Body"
            base_path={base_path+".value.body"}
            body={node_executor.value.body}
        />

        <Box title="WebClient output" base_path={base_path+".value.output"}>
            {#each node_executor.value.output as _, i}
                <WebClientOutput
                    label={"WebClientOutput "+i}
                    output={node_executor.value.output[i]}
                    base_path={base_path+".value.output["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".value.output", i )
                    }
                    swap_items_in_array={
                        ( up : boolean ) =>
                        {
                            swap_array_items( base_path+".value.output", i, ( up ? i - 1 : i + 1 ) );
                        }
                    }
                />
            {/each}
            <div class="text-center">
                <Button
                    text="Add output"
                    click={
                        () => append_to_array( base_path+".value.output", new WebClientOutputStatus() )
                    }
                />
            </div>
        </Box>

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
                            to : new ContextMutDataToContext()
                        }
                    )
                }
            />
        </div>
    {/if}
</Box>