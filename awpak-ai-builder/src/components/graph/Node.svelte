
<script lang="ts">
    import { select_options_from_enum } from "../../functions/form_utils";
    import { DataToString as OutputDataToString } from "../../model/data";
    import { GraphNodeOutputOut, NodeTypeVariant, type NodeType } from "../../model/node";
    import { add_node_destination, add_node_exit_text, change_node_type, change_node_id, change_option_string, append_to_array, remove_from_array, element_from_path, swap_array_items, make_node_first } from "../../store";
    import DataToContext from "../data/DataToContext.svelte";
    import DataToString from "../data/DataToString.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import Select from "../form/Select.svelte";
    import GraphNodeOutput from "../node/GraphNodeOutput.svelte";
    import NodeDestination from "../node/NodeDestination.svelte";
    import NodeExecutor from "../node/NodeExecutor.svelte";
    import { NodeExecutorVariant } from "../../model/node_executor";
    import { is_empty } from "../../functions/data_functions";
    import { ViewType } from "../../model/view_type";

    interface InputProps
    {
        node : NodeType, 
        base_path : string, 
        is_grid? : boolean,
        change_view? : ( view : ViewType, data? : any ) => void
    }

    let { node, base_path, is_grid, change_view } : InputProps = $props();

    // const node_type_options = select_options_from_enum(
    //     NodeTypeVariant,
    //     node._variant,
    //     false
    // );

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

    function node_idx() : number | undefined
    {
        if( ! base_path?.startsWith( "$.nodes[" ) ) return undefined;

        let str_idx = base_path.replace( /^\$.nodes\[/, "" ).replace( /\].*$/, "" );

        if( is_empty( str_idx?.trim() ) || str_idx.trim() === "" ) return;

        try
        {
            let idx = Number( str_idx.trim() );

            if( is_empty( idx ) || isNaN( idx ) ) return undefined;

            return idx;
        }
        catch( _e )
        {
            return undefined;
        }
    }

    function remove_node()
    {
        let idx = node_idx();

        if( is_empty( idx ) ) return;

        if( change_view && typeof( change_view ) === "function" )
        {
            change_view( ViewType.GraphView );
        }

        remove_from_array( "$.nodes", idx as number );
    }

    function execute_node_first()
    {
        if( change_view && typeof( change_view ) === "function" )
        {
            change_view( ViewType.GraphView );
        }

        make_node_first( base_path );
    }

</script>

<Box title={node._variant + " " + node.id} is_grid={is_grid} base_path={base_path}>

    <div>
        <Input label="Id" value={node.id} change_value={send_change_node_id} base_path={undefined} />
        <Select 
            label="Node type" 
            options={
                select_options_from_enum(
                    NodeTypeVariant,
                    node._variant,
                    false
                )
            } 
            value={node._variant} 
            change_value={send_change_node_type} 
            base_path={base_path} 
        />

        {#if node._variant == NodeTypeVariant.GraphNode}
            <Input label="Path" value={node.path} change_value={change_option_string} base_path={base_path+".path"} />
        {/if}

        {#if base_path != "$.first"}
            <div class="text-center">
                <Button text="Execute first" click={() => execute_node_first()} color="blue" />
                <Button text="Remove Node" click={() => remove_node()} color="red" />
            </div>
        {/if}
    </div>

    {#if node._variant == NodeTypeVariant.Node && node.executor}
        <NodeExecutor
            base_path={base_path+".executor"}
            node_executor={node.executor}
        />
    {/if}
    {#if node._variant == NodeTypeVariant.GraphNode}

        <Box title="Graph input" base_path={base_path+".input"}>
            {#each node.input as _, i}
                <DataToString 
                    label={"Graph input "+i} 
                    data={node.input[i]} 
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
            {#each node.output as _, i}
                <GraphNodeOutput 
                    label={"Graph output "+i} 
                    graph_node_output={node.output[i]} 
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
        
    {/if}

    {#if 
        node._variant != NodeTypeVariant.Node ||
        ( 
            node._variant == NodeTypeVariant.Node && 
            node.executor?._variant != NodeExecutorVariant.ContextMut && 
            node.executor?._variant != NodeExecutorVariant.AgentHistoryMut
        )
    }
    <DataToContext 
        label="Output to context" 
        node_output={node._variant == NodeTypeVariant.Node ? node.output : node.node_output} 
        base_path={base_path+"."+(node._variant == NodeTypeVariant.Node ? "output" : "node_output")}
        hide_type={
            node._variant == NodeTypeVariant.Node && node.executor?._variant == NodeExecutorVariant.Parallel
        }
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
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+"."+( node._variant == NodeTypeVariant.Node ? "destination" : "node_destination" ), i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}

    <div class="text-center">
        <Button text="Add node destination" click={send_add_node_destination} />
    </div>
</Box>