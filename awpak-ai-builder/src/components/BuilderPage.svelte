
<script lang="ts">
    import { ViewType } from "../model/view_type";
    import Button from "./form/Button.svelte";
    import Graph from "./graph/Graph.svelte";
    import GraphView from "./graph/GraphView.svelte";
    import Node from "./graph/Node.svelte";
    import { add_node, add_node_exit_text, clear_graph, graph, load_new_graph, remove_from_array } from '../store';
    import { type NodeType } from "../model/node";
    import NodeDestination from "./node/NodeDestination.svelte";
    import InputFile from "./form/InputFile.svelte";
    import { save_graph_to_file } from "../functions/save_to_file";
    import Dropdown from "./form/Dropdown.svelte";
    import DropwdownItem from "./form/DropwdownItem.svelte";


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

    function reset_graph()
    {
        change_view( ViewType.GraphView );

        clear_graph();
    }

</script>

<div class="h-screen grid grid-rows-[auto_1fr]">
    <div class="w-full text-center h-14 border-b border-gray-200 dark:border-gray-700">
        <h1 class="inline"><a href="/" title="AwpakAI home">AwpakAI</a></h1>
        
        {#if view_type == ViewType.GraphView}
        <Button
            text={graph_view_edit_mode ? "End edit mode" : "Start edit mode"}
            click={ 
                () => {
                graph_view_edit_mode = ! graph_view_edit_mode;
                }
            }
            color={graph_view_edit_mode ? "red" : "indigo"}
        />
        {/if}

        <Dropdown label="Options">
            <DropwdownItem
                label="New graph"
                click={ 
                    () => {
                        reset_graph();
                    }
                }
            />
            {#if view_type != ViewType.Graph}
            <DropwdownItem
                label="Graph data"
                click={ 
                    () => {
                        change_view( ViewType.Graph );
                    }
                }
            />
            {/if}
            {#if view_type != ViewType.GraphView}
            <DropwdownItem
                label="Graph view"
                click={ 
                    () => {
                        change_view( ViewType.GraphView );
                    }
                }
            />
            {/if}
            {#if view_type == ViewType.Graph || view_type == ViewType.GraphView}
            <DropwdownItem
                label="Add node"
                click={ 
                    () => {
                        add_node();
                    }
                }
            />
            {/if}
            <DropwdownItem
                label="Load JSON"
                click={load_json}
                is_file={true}
            />
            <DropwdownItem 
                label="Save JSON"
                click={ 
                    () => {
                        save_graph_to_file( $graph );
                    }
                }
            />
        </Dropdown>
    </div>

    
    {#if view_type == ViewType.GraphView}
        {#key $graph}
        <GraphView change_view={change_view} edit_mode={graph_view_edit_mode} />
        {/key}
    {:else if view_type == ViewType.Graph}
        <div class="p-2 overflow-auto">
        {#key $graph}
        <Graph />
        {/key}
        </div>
    {:else if view_type == ViewType.NodeView}
        <div class="p-2 overflow-auto">
        {#key $graph}
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
            change_view={change_view}
        />
        {/key}
        </div>
    {:else if view_type == ViewType.DestinationView}
        <div class="p-2 overflow-auto">
        {#key $graph}
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
            remove_from_loop={
                () => {
                    let bp = partial_view_data.base_path?.trim().replace( /\[[0-9]+\]$/, "" );

                    if( ! bp?.trim() ) return;

                    let d_idx = partial_view_data.destination_idx;

                    change_view( ViewType.GraphView );

                    remove_from_array( bp, d_idx );
                }
            }
        />
        {/key}
        </div>
    {/if}
</div>