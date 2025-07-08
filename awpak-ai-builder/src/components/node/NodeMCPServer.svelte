
<script lang="ts">
    import { NodeMCPServer } from "../../model/agent";
    import { FromContext } from "../../model/data";
    import { append_to_array, change_option_string, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";

    
    interface InputProps
    {
        mcp_server : NodeMCPServer,
        base_path : string,
        remove_from_loop? : () => void | undefined
    }

    let { mcp_server, base_path, remove_from_loop } : InputProps = $props();

</script>

<Box title={"MCP server " + mcp_server.command} base_path={base_path}>

    <Input
        label="Command"
        value={mcp_server.command}
        change_value={change_option_string}
        base_path={base_path+".command"}
    />

    <Box title="Arguments" base_path={base_path+".args"}>
        {#each mcp_server.args as _, i}
            <DataFrom
                label={"Arg "+i}
                from={mcp_server.args[i]}
                base_path={base_path+".args["+i+"]"}
                remove_from_loop={
                    () => remove_from_array( base_path+".args", i )
                }
                swap_items_in_array={
                    ( up : boolean ) =>
                    {
                        swap_array_items( base_path+".args", i, ( up ? i - 1 : i + 1 ) );
                    }
                }
            />
        {/each}
        <div class="text-center">
            <Button
                text="Add argument"
                click={
                    () => append_to_array( base_path+".args", new FromContext() )
                }
            />
        </div>
    </Box>

    {#if typeof( remove_from_loop ) == "function"}
    <div class="text-center">
        <Button text="Remove MCP server" click={remove_from_loop} color="red" />
    </div>
    {/if}
</Box>
