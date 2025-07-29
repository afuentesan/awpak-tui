<script lang="ts">
    import cytoscape from 'cytoscape';
    import edgehandles from 'cytoscape-edgehandles';
    import klay from 'cytoscape-klay';
    import { onMount } from 'svelte';
    import { add_node_destination, element_from_path, graph } from '../../store';
    import { change_graph_positions, graph_positions } from '../../positions_store';
    import { graph_to_cytoscape, ID_EXIT_ERR, ID_EXIT_OK } from '../../functions/graph_to_cytoscape';
    import { NodeTypeVariant } from '../../model/node';
    import { ViewType } from '../../model/view_type';
    import { node_and_base_path_from_id } from '../../functions/node_functions';
    import type { Graph } from '../../model/graph';
    import { NodeExecutorVariant } from '../../model/node_executor';


    interface InputProps
    {
        change_view : ( view : ViewType, data? : any ) => void,
        edit_mode : boolean
    }

    let { change_view, edit_mode } : InputProps = $props();

    const click_on_edge = ( event : cytoscape.EventObject ) =>
    {
        clean_tooltips();

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
        clean_tooltips();

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

    const mouseover_elem = ( event : cytoscape.EventObject ) =>
    {
        if( typeof( event.target.id ) !== "function" || ! event?.target?.attr( "info" )?.trim() ) return;

        event.preventDefault();
        event.stopPropagation();

        const inner_html = event.target.attr( "info" ).trim();

        show_tooltip( event.target.id(), inner_html, event.originalEvent.clientX, event.originalEvent.clientY );
    };

    const mouseout_elem = ( event : cytoscape.EventObject ) =>
    {
        if( typeof( event.target.id ) !== "function" ) return;

        hide_tooltip( event.target.id() );
    };

    const fix_tooltip_position = ( tooltip : HTMLElement | null ) =>
    {
        if( ! tooltip ) return;

        // Obtener posición actual y tamaño del tooltip
        const rect = tooltip.getBoundingClientRect();

        // Valores ajustados (copiamos para modificar)
        let newLeft = rect.left;
        let newTop = rect.top;

        const margin = 4; // margen mínimo para no tocar el borde

        // Verificamos límites horizontales
        if (rect.right > window.innerWidth) {
            newLeft = window.innerWidth - rect.width - margin;
        } else if (rect.left < 0) {
            newLeft = margin;
        }

        // Verificamos límites verticales
        if (rect.bottom > window.innerHeight) {
            newTop = window.innerHeight - rect.height - margin;
        } else if (rect.top < 0) {
            newTop = margin;
        }

        tooltip.style.left = `${newLeft}px`;
        tooltip.style.top = `${newTop}px`;
    };

    const fix_tooltips_position = () =>
    {
        let tooltips = document.getElementsByClassName( "tooltip" );

        if( ! tooltips?.length ) return;

        for( let i = 0 ; i < tooltips.length ; i++ )
        {
            fix_tooltip_position( tooltips.item( i ) as HTMLElement | null );   
        }
    };

    const clean_tooltips = () =>
    {
        try
        {
            let tooltips = document.getElementsByClassName( "tooltip" );

            if( ! tooltips?.length ) return;

            for( let i = ( tooltips.length - 1 ) ; i >= 0 ; i-- )
            {
                tooltips.item( i )?.remove();
            }
        }
        catch( e )
        {
            console.log( e );
        }
    };

    const tooltip_id = ( id_elem : string ) =>
    {
        return "tooltip_" + id_elem;
    };

    const hide_tooltip = ( id : string ) =>
    {
        let current = document.getElementById( tooltip_id( id ) );

        if( current ) current.remove();
    };

    const show_tooltip = ( id : string, inner_html : string, x : number, y : number ) =>
    {
        if( edit_mode )
        {
            clean_tooltips();
            
            return;
        }

        let current = document.getElementById( tooltip_id( id ) );

        if( current )
        {
            current.style.top = y + "px";
            current.style.left = x + "px";

            fix_tooltips_position();

            return;
        }

        clean_tooltips();

        let div = document.createElement( 'div' );

        div.classList.add( 
            "absolute", 
            "p-2", 
            "border", 
            "rounded-md", 
            "tooltip", 
            "bg-white", 
            "dark:bg-gray-700",
            "border-gray-200",
            "shadow-sm",
            "dark:border-gray-500"
        );

        div.id = tooltip_id( id );

        div.innerHTML = inner_html;

        div.style.top = y + "px";
        div.style.left = x + "px";

        document.body.appendChild( div );

        fix_tooltips_position();
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
    let eh: edgehandles.EdgeHandlesInstance;

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

                // cy.nodes().on( "click", click_on_node );
                // cy.edges().on( "click", click_on_edge );

                cy.nodes().on( "tap", click_on_node );
                cy.edges().on( "tap", click_on_edge );

                cy.nodes().on( "mouseover", mouseover_elem );
                cy.nodes().on( "mouseout", mouseout_elem );

                cy.edges().on( "mouseover", mouseover_elem );
                cy.edges().on( "mouseout", mouseout_elem );
            }
        } 
    );

    $effect(
        () => 
        {
            edit_mode;

            if( ! eh ) return;

            if( edit_mode )
            {
                eh.enableDrawMode();
            }
            else
            {
                eh.disableDrawMode();
            }
        }
    );

    onMount( () => 
    {
        const [ nodes_and_edges, layout ] = nodes_and_layout( $graph );

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
                        selector: `node[ty = "${NodeExecutorVariant.Command}"]`,
                        style: {
                        'background-color': '#1D4ED8'
                        }
                    },
                    {
                        selector: `node[ty = "${NodeExecutorVariant.WebClient}"]`,
                        style: {
                        'background-color': '#374151'
                        }
                    },
                    {
                        selector: `node[ty = "${NodeExecutorVariant.Parallel}"]`,
                        style: {
                        'background-color': '#5145CD'
                        }
                    },
                    {
                        selector: `node[ty = "${NodeExecutorVariant.Agent}"]`,
                        style: {
                        'background-color': '#8E4B10'
                        }
                    },
                    {
                        selector: `node[ty = "${NodeExecutorVariant.ContextMut}"]`,
                        style: {
                        'background-color': '#BF125D'
                        }
                    },
                    {
                        selector: `node[ty = "${NodeExecutorVariant.AgentHistoryMut}"]`,
                        style: {
                        'background-color': '#6C2BD9'
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
                        selector: `node[first="first"]`,
                        style: {
                        'border-width' : '2px',
                        'border-color' : '#008236'
                        }
                    },
                    { 
                        selector : 'edge', 
                        style : { 
                            'width': 2,
                            'line-color': '#6875F5',
                            'target-arrow-shape': 'triangle',
                            'target-arrow-color': '#6875F5',
                            'curve-style': 'bezier' 
                        } 
                    },
                    {
                        selector : "edge:before",
                        style : {
                            'content' : "data(idx)",
                            'font-size' : "6px",
                            'font-weight' : "bolder",
                            'color' : '#a65f00',
                            'text-margin-y' : -3
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
            canConnect: function( src : any, _target : any )
            {
                if( ! src?.id() || src.id() == ID_EXIT_OK || src.id() == ID_EXIT_ERR )
                {
                    return false;
                }
                
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

        eh = cy.edgehandles( defaults as any );

        cy.on( 'ehcomplete', ( _event, src, target, _edge ) => 
            {
                if( ! src?.id() || ! target?.id() ) return;
                
                add_node_destination( src.id(), target.id() );
            }
        );

        if( edit_mode )
        {
            eh.enableDrawMode();
        }

        // cy.nodes().on( "click", click_on_node );
        cy.nodes().on( "tap", click_on_node );
        // cy.edges().on( "click", click_on_edge );
        cy.edges().on( "tap", click_on_edge );

        cy.nodes().on( "mouseover", mouseover_elem );
        cy.nodes().on( "mouseout", mouseout_elem );

        cy.edges().on( "mouseover", mouseover_elem );
        cy.edges().on( "mouseout", mouseout_elem );

        const hideTooltipsOnClick = ( _event : any ) =>
        {
            clean_tooltips();
        };

        document.addEventListener( 'click', hideTooltipsOnClick, false );

        return () => 
        {
            clean_tooltips();

            change_graph_positions( cy );

            unsubscribe();

            eh.destroy();
            cy.destroy();

            document.removeEventListener( 'click', hideTooltipsOnClick, false );
        };
    });
</script>

<div bind:this={container} class="overflow-hidden"></div>