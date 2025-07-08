<script lang="ts">
    import cytoscape from 'cytoscape';
    import edgehandles from 'cytoscape-edgehandles';
    import klay from 'cytoscape-klay';
    import { onMount } from 'svelte';
    import { element_from_path, graph } from '../../store';
    import { change_graph_positions, graph_positions } from '../../positions_store';
    import { graph_to_cytoscape, ID_EXIT_ERR, ID_EXIT_OK } from '../../functions/graph_to_cytoscape';
    import { NodeTypeVariant } from '../../model/node';
    import { ViewType } from '../../model/view_type';
    import { node_and_base_path_from_id } from '../../functions/node_functions';
    import type { Graph } from '../../model/graph';


    interface InputProps
    {
        change_view : ( view : ViewType, data? : any ) => void
    }

    let { change_view } : InputProps = $props();

    const click_on_edge = ( event : cytoscape.EventObject ) =>
    {
        let id = event?.target?.id();
        let source_id = event?.target?.source()?.id();

        if( 
            typeof( id ) === "undefined" || 
            id === null || 
            id.trim() === "" ||
            ! id.trim().match( /__[0-9]+$/ ) ||
            typeof( source_id ) === "undefined" || 
            source_id === null ||
            source_id.trim() === ""
        ) return;

        event.preventDefault();
        event.stopPropagation();

        let parts = id.trim().split( "__" );

        let idx = parts[ parts.length - 1 ].trim();

        let node = node_and_base_path_from_id( $graph, source_id );

        if( ! node ) return;

        let base_path = node.base_path + "." + ( node.node._variant == NodeTypeVariant.Node ? "destination" : "node_destination" ) + "[" + idx + "]";

        let destination = element_from_path( $graph, base_path );

        if( ! destination ) return;

        let data = {
            node : node.node,
            base_path : base_path,
            idx : node.idx,
            is_first : node.is_first,
            destination_name : ( node.node._variant == NodeTypeVariant.Node ? "destination" : "node_destination" ),
            destination_idx : Number( idx ),
            source_id : source_id,
            target_id : event?.target?.target()?.id()
        };

        change_view( ViewType.DestinationView, data );
    };

    const click_on_node = ( event : cytoscape.EventObject ) =>
    {
        if( typeof( event.target.id ) !== "function" ) return;

        event.preventDefault();
        event.stopPropagation();

        let node = node_and_base_path_from_id( $graph, event.target.id() );

        if( ! node ) return;

        let data = {
            node : node.node,
            base_path : node.base_path,
            label : node.node._variant + " " + node.node.id,
            idx : node.idx,
            is_first : node.is_first
        };

        change_view( ViewType.NodeView, data );
    };

    const nodes_and_layout = ( g : Graph ) : [ Array<any>, string ] =>
    {
        const [ nodes, all_has_position ] = graph_to_cytoscape( g, $graph_positions );

        return [
            nodes,
            ( all_has_position ) ? "klay" : "klay"
        ]
    };

    let cy: cytoscape.Core;
    let container: HTMLDivElement;

    cytoscape.use( klay );
    cytoscape.use( edgehandles );

    // Actualiza el grafo si el store cambia
    const unsubscribe = graph.subscribe( ( data ) => 
        {
            if( cy )
            {
                const [ nodes_and_edges, layout ] = nodes_and_layout( data );

                console.log( "Changed graph: ", data );

                cy.elements().remove();              // Quitar los nodos y edges antiguos
                cy.add( nodes_and_edges );

                cy.layout({ name: layout }).run();  // Reaplicar layout

                cy.nodes().on( "click", click_on_node );
                cy.edges().on( "click", click_on_edge );
            }
        } 
    );

    onMount(() => 
    {        
        const [ nodes_and_edges, layout ] = nodes_and_layout( $graph );

        console.log( "onMount: ", $graph, nodes_and_edges );

        cy = cytoscape(
            {
                container,
                elements: nodes_and_edges,
                style: [
                    { 
                        selector : 'node', 
                        style : { 
                            'shape': 'roundrectangle',
                            'label': 'data(label)',
                            'text-valign': 'center',
                            'text-halign': 'center',
                            'text-wrap': 'wrap',
                            'text-max-width': "100",
                            'padding': '10px',
                            'background-color': '#111827',
                            'color': '#ffffff',
                            'font-size': 12,
                            'width': 'label',
                            'height': 'label'
                        } 
                    },
                    {
                        selector: `node[ty = "${NodeTypeVariant.Node}"]`,
                        style: {
                        'background-color': '#1D4ED8'
                        }
                    },
                    {
                        selector: `node[ty = "${NodeTypeVariant.GraphNode}"]`,
                        style: {
                        'background-color': '#6D28D9'
                        }
                    },
                    {
                        selector: `node[ty = "${ID_EXIT_OK}"]`,
                        style: {
                        'background-color': '#047857'
                        }
                    },
                    {
                        selector: `node[ty = "${ID_EXIT_ERR}"]`,
                        style: {
                        'background-color': '#B91C1C'
                        }
                    },
                    { 
                        selector : 'edge', 
                        style : { 
                            'width': 2,
                            'line-color': '#ffffff',
                            'target-arrow-shape': 'triangle',
                            'target-arrow-color': '#ffffff',
                            'curve-style': 'bezier' 
                        } 
                    },
                    {
                        selector: 'edge[source = target]',
                        style: { 'curve-style': 'bezier', 'control-point-step-size': 55}
                    }
                ],
                layout: { name: layout }
            }
        );

        // the default values of each option are outlined below:
        let defaults = {
        canConnect: function( _src : any, _target : any ){
            // whether an edge can be created between source and target
            return true;
        },
        edgeParams: function( _src : any, _target : any ){
            // for edges between the specified source and target
            // return element object to be passed to cy.add() for edge
            return {};
        },
        hoverDelay: 150, // time spent hovering over a target node before it is considered selected
        snap: true, // when enabled, the edge can be drawn by just moving close to a target node (can be confusing on compound graphs)
        snapThreshold: 50, // the target node must be less than or equal to this many pixels away from the cursor/finger
        snapFrequency: 15, // the number of times per second (Hz) that snap checks done (lower is less expensive)
        noEdgeEventsInDraw: true, // set events:no to edges during draws, prevents mouseouts on compounds
        disableBrowserGestures: true // during an edge drawing gesture, disable browser gestures such as two-finger trackpad swipe and pinch-to-zoom
        };

        // let eh = cy.edgehandles( defaults as any );

        // eh.enableDrawMode();
        
        cy.nodes().on( "click", click_on_node );
        cy.edges().on( "click", click_on_edge );

        // cy.nodes().on( 
        //     "mousedown", 
        //     ( event : any ) => 
        //     {
        //         const node = event.target;
        //         const clickPos = event.renderedPosition;

        //         const box = node.boundingBox({ rendered: true });

        //         const relativeX = clickPos.x - box.x1;
        //         const relativeY = clickPos.y - box.y1;

        //         const width = box.w;
        //         const height = box.h;

        //         console.log(
        //             `Click relativo dentro del nodo: x=${relativeX}, y=${relativeY}, width=${width}, height=${height}`
        //         );
        //     }
        // );

        return () => 
        {
            change_graph_positions( cy );

            unsubscribe();

            cy.destroy();
        };
    });
</script>

<div bind:this={container} class="w-full min-h-screen"></div>