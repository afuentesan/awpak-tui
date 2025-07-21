
<script lang="ts">
    import { CommandOutputOut, type Command } from "../../model/command";
    import { FromContext } from "../../model/data";
    import { append_to_array, change_option_number, change_option_string, remove_from_array, swap_array_items } from "../../store";
    import DataFrom from "../data/DataFrom.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import CommandOutput from "./CommandOutput.svelte";

    interface InputProps
    {
        command : Command,
        base_path : string
    }

    let { command, base_path } : InputProps = $props();

</script>

<DataFrom
    label="Command"
    base_path={base_path+".command"}
    from={command.command}
/>

<Box title="Arguments" base_path={base_path+".args"}>
    {#each command.args as _, i}
        <DataFrom
            label={"Arg "+i}
            from={command.args[i]}
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

<Input 
    label="Timeout (secs.)" 
    input_type="number" 
    value={command.timeout} 
    change_value={change_option_number} 
    base_path={base_path+".timeout"} 
/>

<Box title="Command output" base_path={base_path+".output"}>
    {#each command.output as _, i}
        <CommandOutput
            label={"CommandOutput "+i}
            output={command.output[i]}
            base_path={base_path+".output["+i+"]"}
            remove_from_loop={
                () => remove_from_array( base_path+".output", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".output", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button
            text="Add output"
            click={
                () => append_to_array( base_path+".output", new CommandOutputOut() )
            }
        />
    </div>
</Box>