import type cytoscape from "cytoscape";
import type { Graph } from "../model/graph";
import { NodeNextVariant, NodeTypeVariant, type NodeDestination, type NodeNext, type NodeType } from "../model/node";
import { random_id } from "./form_utils";

export const ID_EXIT_OK = "__ExitOk";
export const ID_EXIT_ERR = "__ExitErr";

export function graph_to_cytoscape( graph : Graph, positions? : Map<string, cytoscape.Position> ) : [Array<any>, boolean]
{
    let [ nodes, all_has_position ] = graph_nodes( graph, positions );

    return [[ ...nodes, ...graph_edges( graph ) ], all_has_position]
}

function graph_nodes( graph : Graph, positions? : Map<string, cytoscape.Position> ) : [Array<any>, boolean]
{
    if( ! graph.first?.id ) { return [[], false] };

    let ret : Array<any> = [];

    let first_node = graph_node( graph.first, positions?.get( graph.first.id ) );

    let all_has_position = first_node.position ? true : false;

    ret.push( first_node );

    graph.nodes.forEach( 
        ( n ) => 
        {
            let node = graph_node( n, positions?.get( n.id as string ) );

            if( ! node.position )
            {
                all_has_position = false;
            }

            ret.push( node ) 
        }
    );

    ret.push( ...exit_nodes( positions ) );

    return [ ret, all_has_position ];
}

function exit_nodes( positions? : Map<string, cytoscape.Position> ) : Array<any>
{
    return [ 
        { 
            data : { 
                id : ID_EXIT_OK, 
                label : "Exit Ok",
                ty : ID_EXIT_OK
            },
            position : clone_position( positions?.get( ID_EXIT_OK ) )
        },
        { 
            data : { 
                id : ID_EXIT_ERR, 
                label : "Exit Err",
                ty : ID_EXIT_ERR
            },
            position : clone_position( positions?.get( ID_EXIT_ERR ) )
        }
    ]
}

function clone_position( position : cytoscape.Position | undefined )
{
    if( ! position?.x || ! position?.y ) return undefined;

    return {
        x : position.x,
        y : position.y
    }
}

function graph_node( node : NodeType, position? : cytoscape.Position | undefined ) : any
{
    return { 
        data : { 
            id : node.id, 
            label : label_node( node ),
            ty : graph_node_ty( node )
        },
        position : clone_position( position )
    }
}

function graph_node_ty( node : NodeType ) : string
{
    if( node._variant == NodeTypeVariant.GraphNode ) return node._variant;

    if( node.executor ) return node.executor._variant;

    return node._variant;
}

function label_node( node : NodeType ) : string
{
    return node.id || node._variant;

    // if( node._variant == NodeTypeVariant.GraphNode )
    // {
    //     return "Graph " + node.id;
    // }

    // if( ! node.executor )
    // {
    //     return node.id + "";
    // }

    // return node.executor._variant + " " + node.id;
}

function graph_edges( graph : Graph ) : Array<any>
{
    if( ! graph.first ) { return [] };

    let ret : Array<any> = [];

    ret.push( ...node_edges( graph.first ) );

    graph.nodes.forEach( ( n ) => ret.push( ...node_edges( n ) ) );

    return ret;
}

function node_edges( node : NodeType ) : Array<any>
{
    let destinations = ( node._variant == NodeTypeVariant.Node ) ? node.destination : node.node_destination;

    let ret : Array<any> = [];

    destinations.forEach(
        ( d, idx ) => ret.push( node_edge( node, d, idx ) )
    );

    return ret;
}

function node_edge( node : NodeType, destination : NodeDestination, idx : number ) : any
{
    let target = target_destination( destination );

    return { 
        data : { 
            id : node.id + "__" + target + "__" + idx, 
            source : node.id, 
            target : target
        } 
    }
}

function target_destination( destination : NodeDestination ) : string
{
    if( destination.next?._variant == NodeNextVariant.ExitOk )
    {
        return ID_EXIT_OK;
    }
    else if( destination.next?._variant == NodeNextVariant.ExitErr )
    {
        return ID_EXIT_ERR;
    }
    else if( destination.next?._variant == NodeNextVariant.Node )
    {
        return destination.next.value as string;
    }
    
    throw new Error( "Next node variant not found" );
}