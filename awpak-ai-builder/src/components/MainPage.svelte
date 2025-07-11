
<script lang="ts">
    import { ViewType } from "../model/view_type";
    import Button from "./form/Button.svelte";
    import Graph from "./graph/Graph.svelte";
    import GraphView from "./graph/GraphView.svelte";
    import Node from "./graph/Node.svelte";
    import { add_node, add_node_exit_text, graph, load_new_graph } from '../store';
    import { type NodeType } from "../model/node";
    import NodeDestination from "./node/NodeDestination.svelte";
    import InputFile from "./form/InputFile.svelte";
    import { save_graph_to_file } from "../functions/save_to_file";


    let view_type = $state( ViewType.GraphView );

    let partial_view_data = $state( {} as any );

    let graph_view_edit_mode = $state( false );

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

    function load_json( event : any )
    {
        if( ! event.target.files[ 0 ] ) { return; }

        let file = event.target.files[ 0 ];

        let reader = new FileReader();

        reader.readAsText( file, "UTF-8" );

        reader.onload = ( evt : any ) =>
        {
            let text = evt.target.result;

            load_new_graph( text );
        }

        reader.onerror = ( e : any ) =>
        {
            console.log( "Error read file. ", e );
        }
    }

</script>

<div class="w-full text-center h-14 border-b border-gray-200 dark:border-gray-700">
    {#if view_type != ViewType.Graph}
    <Button
        text="Graph data"
        click={ 
            () => {
               change_view( ViewType.Graph );
            }
        }
        color="blue"
    />
    {/if}
    {#if view_type != ViewType.GraphView}
    <Button
        text="Graph view"
        click={ 
            () => {
               change_view( ViewType.GraphView );
            }
        }
        color="blue"
    />
    {:else}
    <Button
        text={graph_view_edit_mode ? "End edit mode" : "Start edit mode"}
        click={ 
            () => {
               graph_view_edit_mode = ! graph_view_edit_mode;
            }
        }
        color={graph_view_edit_mode ? "red" : "light"}
    />
    {/if}

    {#if view_type == ViewType.Graph || view_type == ViewType.GraphView}
    <Button
        text="Add node"
        click={ 
            () => {
               add_node();
            }
        }
        color="green"
    />
    {/if}

    <InputFile
        id="load_json_file"
        text="Load JSON"
        change={load_json}
        color="purple"
    />
    <Button
        text="Save JSON"
        click={ 
            () => {
               save_graph_to_file( $graph );
            }
        }
        color="purple"
    />
</div>

{#key $graph}
{#if view_type == ViewType.GraphView}
    <GraphView change_view={change_view} edit_mode={graph_view_edit_mode} />
{:else if view_type == ViewType.Graph}
    <Graph />
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
{/key}