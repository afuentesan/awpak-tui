
<script lang="ts">
    import { GraphNodeOutputOut, type GraphExecutor } from "../../model/graph_executor";
    import { append_to_array, change_option_string, remove_from_array, swap_array_items } from "../../store";
    import DataToString from "../data/DataToString.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import { DataToString as OutputDataToString } from "../../model/data";
    import GraphNodeOutput from "./GraphNodeOutput.svelte";


    interface InputProps
    {
        graph_executor : GraphExecutor,
        base_path : string
    }

    let { graph_executor, base_path } : InputProps = $props();

    function send_add_graph_input()
    {
        let path = base_path + ".input";

        let new_input = new OutputDataToString();

        append_to_array( path, new_input );
    }

    function send_add_graph_output()
    {
        let path = base_path + ".output";

        let new_output = new GraphNodeOutputOut();

        append_to_array( path, new_output );
    }

</script>

<Input label="Path" value={graph_executor.path} change_value={change_option_string} base_path={base_path+".path"} />

<Box title="Graph input" base_path={base_path+".input"}>
    {#each graph_executor.input as _, i}
        <DataToString 
            label={"Graph input "+i} 
            data={graph_executor.input[i]} 
            base_path={base_path+".input["+i+"]"} 
            remove_from_loop={
                () => remove_from_array( base_path+".input", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".input", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}
    <div class="text-center">
        <Button text="New graph input" click={send_add_graph_input} />
    </div>
</Box>

<Box title="Graph output" base_path={base_path+".output"}>
    {#each graph_executor.output as _, i}
        <GraphNodeOutput 
            label={"Graph output "+i} 
            graph_node_output={graph_executor.output[i]} 
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
        <Button text="New graph output" click={send_add_graph_output} />
    </div>
</Box>