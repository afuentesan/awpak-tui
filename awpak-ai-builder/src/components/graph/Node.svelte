
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataToString as OutputDataToString } from "../../model/data";
    import { GraphNodeOutputOut, NodeTypeVariant, type NodeType } from "../../model/node";
    import { add_node_destination, add_node_exit_text, change_node_type, change_node_id, change_option_string, append_to_array, remove_from_array, element_from_path } from "../../store";
    import DataToContext from "../data/DataToContext.svelte";
    import DataToString from "../data/DataToString.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import GraphNodeOutput from "../node/GraphNodeOutput.svelte";
    import NodeDestination from "../node/NodeDestination.svelte";
    import NodeExecutor from "../node/NodeExecutor.svelte";
    import { graph } from "../../store";
    import { NodeExecutorVariant } from "../../model/node_executor";

    interface InputProps
    {
        node : NodeType, 
        base_path : string, 
        remove_from_loop? : () => void | undefined,
        is_grid? : boolean
    }

    let { node, base_path, remove_from_loop, is_grid } : InputProps = $props();

    const node_type_options = select_options_from_enum(
        NodeTypeVariant,
        node._variant,
        false
    );

    function send_change_node_type( base_path : string, value : any )
    {
        change_node_type( base_path, value );
    }

    function send_change_node_id( event : any )
    {
        event.preventDefault();

        change_node_id( node.id as string, event?.target?.value );
    }

    function send_add_node_destination()
    {
        add_node_destination( node.id as string );
    }

    function send_add_node_exit_text( idx : number )
    {
        return ( _ : any ) =>
        {
            add_node_exit_text( idx, node.id as string );
        }
    }

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

    $effect(() => {
        
        if( ! node?.id ) return;

        let new_node = element_from_path( $graph, base_path );

        if( ! new_node ) return;
        // let new_node = node_by_id( $graph, node.id );

        node = Object.assign( {}, new_node );
    });

</script>

<Box title={node._variant + " " + node.id} is_grid={is_grid}>

    <div>
        <Input label="Id" value={node.id} change_value={send_change_node_id} base_path={undefined} />
        <Select label="Node type" options={node_type_options} value={node._variant} change_value={send_change_node_type} base_path={base_path} />

        {#if node._variant == NodeTypeVariant.GraphNode}
            <Input label="Path" value={node.path} change_value={change_option_string} base_path={base_path+".path"} />
        {/if}
    </div>

    {#if node._variant == NodeTypeVariant.Node && node.executor}
        <NodeExecutor
            base_path={base_path+".executor"}
            node_executor={node.executor}
        />
    {/if}
    {#if node._variant == NodeTypeVariant.GraphNode}

        <Box title="Graph input">
            {#each node.input as _, i}
                <DataToString 
                    label={"Graph input "+i} 
                    data={node.input[i]} 
                    base_path={base_path+".input["+i+"]"} 
                    remove_from_loop={
                        () => remove_from_array( base_path+".input", i )
                    } 
                />
            {/each}
            <div class="text-center">
                <Button text="New graph input" click={send_add_graph_input} />
            </div>
        </Box>

        <Box title="Graph output">
            {#each node.output as _, i}
                <GraphNodeOutput 
                    label={"Graph output "+i} 
                    graph_node_output={node.output[i]} 
                    base_path={base_path+".output["+i+"]"}
                    remove_from_loop={
                        () => remove_from_array( base_path+".output", i )
                    } 
                />
            {/each}
            <div class="text-center">
                <Button text="New graph output" click={send_add_graph_output} />
            </div>
        </Box>
        
    {/if}

    {#if 
        node._variant != NodeTypeVariant.Node ||
        ( node._variant == NodeTypeVariant.Node && node.executor?._variant != NodeExecutorVariant.ContextMut)
    }
    <DataToContext 
        label="Output to context" 
        node_output={node._variant == NodeTypeVariant.Node ? node.output : node.node_output} 
        base_path={base_path+"."+(node._variant == NodeTypeVariant.Node ? "output" : "node_output")}
    />
    {/if}
    
    {#each ( node._variant == NodeTypeVariant.Node ? node.destination : node.node_destination ) as _, i}
        <NodeDestination  
            destination={( node._variant == NodeTypeVariant.Node ? node.destination : node.node_destination )[i]}
            add_exit_text={send_add_node_exit_text( i )}
            base_path={base_path+"."+( node._variant == NodeTypeVariant.Node ? "destination" : "node_destination" )+"["+i+"]"}
            label={"NodeDestination "+i}
            remove_from_loop={
                () => remove_from_array( base_path+"."+( node._variant == NodeTypeVariant.Node ? "destination" : "node_destination" ), i )
            }
        />
    {/each}

    <div class="text-center">
        <Button text="Add node destination" click={send_add_node_destination} />
        
        {#if typeof( remove_from_loop ) == "function"}
        <hr class="h-px my-2 bg-gray-200 border-0 dark:bg-gray-700">
        <Button text="Remove Node" click={remove_from_loop} color="red" />
        {/if}
    </div>
</Box>