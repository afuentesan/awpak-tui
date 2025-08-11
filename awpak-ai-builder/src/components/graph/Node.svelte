
<script lang="ts">
    import { NodeConfig } from "../../model/node";
    import { add_node_destination, add_node_exit_text, change_node_id, remove_from_array, swap_array_items, make_node_first } from "../../store";
    import DataToContext from "../data/DataToContext.svelte";
    import Box from "../form/Box.svelte";
    import Button from "../form/Button.svelte";
    import Input from "../form/Input.svelte";
    import NodeDestination from "../node/NodeDestination.svelte";
    import NodeExecutor from "../node/NodeExecutor.svelte";
    import { NodeExecutorVariant } from "../../model/node_executor";
    import { is_empty } from "../../functions/data_functions";
    import { ViewType } from "../../model/view_type";

    interface InputProps
    {
        node : NodeConfig, 
        base_path : string, 
        is_grid? : boolean,
        change_view? : ( view : ViewType, data? : any ) => void
    }

    let { node, base_path, is_grid, change_view } : InputProps = $props();

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

<Box title={"Node " + node.id} is_grid={is_grid} base_path={base_path}>

    <div>
        <Input label="Id" value={node.id} change_value={send_change_node_id} base_path={undefined} />

        {#if base_path != "$.first"}
            <div class="text-center">
                <Button text="Execute first" click={() => execute_node_first()} color="blue" />
                <Button text="Remove Node" click={() => remove_node()} color="red" />
            </div>
        {/if}
    </div>

    {#if node.executor}
        <NodeExecutor
            base_path={base_path+".executor"}
            node_executor={node.executor}
        />
    {/if}
    
    {#if 
        node.executor?._variant != NodeExecutorVariant.ContextMut && 
        node.executor?._variant != NodeExecutorVariant.AgentHistoryMut
    }
    <DataToContext 
        label="Output to context" 
        node_output={node.output} 
        base_path={base_path+".output"}
        hide_type={
            node.executor?._variant == NodeExecutorVariant.Parallel
        }
    />
    {/if}
    
    {#each node.destination as _, i}
        <NodeDestination  
            destination={node.destination[i]}
            add_exit_text={send_add_node_exit_text( i )}
            base_path={base_path+".destination["+i+"]"}
            label={"NodeDestination "+i}
            remove_from_loop={
                () => remove_from_array( base_path+".destination", i )
            }
            swap_items_in_array={
                ( up : boolean ) =>
                {
                    swap_array_items( base_path+".destination", i, ( up ? i - 1 : i + 1 ) );
                }
            }
        />
    {/each}

    <div class="text-center">
        <Button text="Add node destination" click={send_add_node_destination} />
    </div>
</Box>