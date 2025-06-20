import type { Graph } from "../model/graph";
import { GraphNode, Node, type NodeType } from "../model/node";

export function node_variants() : Array<string>
{
    return [
        "Node",
        "GraphNode"
    ]
}

export function node_next_variants() : Array<string>
{
    return [
        "Node",
        "ExitOk",
        "ExitErr"
    ]
}

export function node_ids( graph : Graph ) : Array<string>
{
    let ids : Array<string> = [ graph.first?.id as string ];

    graph.nodes.forEach( ( v : NodeType ) => ids.push( v.id as string ) );

    return ids;
}

export function node_by_id( graph : Graph, id : string ) : NodeType | undefined
{
    if( ! id?.trim() ) { return undefined };

    if( graph.first?.id == id )
    {
        return graph.first;
    }

    return graph.nodes.get( id );
}

export function change_node_variant( node : NodeType, new_variant : string ) : NodeType | undefined
{
    if( ! new_variant?.trim() || new_variant == node._variant ) { return undefined; }

    if( new_variant == "Node" )
    {
        return node_variant( node, new_variant );
    }
    else if( new_variant == "GraphNode" )
    {
        return graph_node_variant( node, new_variant );
    }
}

function node_variant( from : NodeType, new_variant : string ) : NodeType
{
    let node = new Node( from.id as string );

    if( new_variant == "GraphNode" )
    {
        node.output = ( from as GraphNode ).node_output;
        node.destination = ( from as GraphNode ).node_destination;
    }

    return node;
}

function graph_node_variant( from : NodeType, new_variant : string ) : NodeType
{
    let node = new GraphNode( from.id as string );

    if( new_variant == "Node" )
    {
        node.node_output = ( from as Node ).output;
        node.node_destination = ( from as Node ).destination;
    }

    return node;
}