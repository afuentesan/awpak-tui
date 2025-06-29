
<script lang="ts">
    import { ViewType } from "../model/view_type";
    import Button from "./form/Button.svelte";
    import Graph from "./graph/Graph.svelte";
    import GraphView from "./graph/GraphView.svelte";
    import Node from "./graph/Node.svelte";
    import { add_node_exit_text, graph } from '../store';
    import { type NodeType } from "../model/node";
    import NodeDestination from "./node/NodeDestination.svelte";


    let view_type = $state( ViewType.Graph );

    let partial_view_data = $state( {} as any );

    function change_view( view : ViewType, data? : any )
    {
        if( data )
        {
            partial_view_data = data;
        }

        view_type = view;
    }

    function send_add_node_exit_text( source_id : string, idx : number )
    {
        return ( _ : any ) =>
        {
            add_node_exit_text( idx, source_id );
        }
    }

</script>

<div class="w-full text-center">
    <Button
        text="Graph data"
        click={ 
            () => {
               change_view( ViewType.Graph );
            }
        }
    />
    <Button
        text="Graph view"
        click={ 
            () => {
               change_view( ViewType.GraphView );
            }
        }
        color="blue"
    />
</div>

{#if view_type == ViewType.Graph}
    <Graph />
{:else if view_type == ViewType.GraphView}
    <GraphView change_view={change_view} />
{:else if view_type == ViewType.NodeView}
    <Node
        base_path={partial_view_data.base_path}
        node={
            ( 
                ( partial_view_data.is_first ) 
                ? 
                $graph.first 
                : 
                $graph.nodes[ partial_view_data.idx as number ] 
            ) as NodeType
        }
        is_grid={true}
    />
{:else if view_type == ViewType.DestinationView}
    <NodeDestination
        base_path={partial_view_data.base_path}
        destination={
            (
                ( 
                    ( partial_view_data.is_first ) 
                    ? 
                    $graph.first 
                    : 
                    $graph.nodes[ partial_view_data.idx as number ] 
                ) as any
            )[ partial_view_data.destination_name ][ partial_view_data.destination_idx ]
        }
        from={partial_view_data.source_id}
        add_exit_text={send_add_node_exit_text( partial_view_data.source_id, partial_view_data.destination_idx )}
        is_grid={true}
    />
{/if}