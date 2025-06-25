import type { DataToContext } from "../model/data";
import type { DataComparator } from "../model/data_comparator";
import type { Graph } from "../model/graph";
import { GraphNode, NodeDestination, NodeTypeVariant, type Node, type NodeNext, type NodeType } from "../model/node";

export function generate_json( graph : Graph ) : string
{
    let json : any = {};

    append_initial_context( json, graph );
    append_input_data_type( json, graph );

    json[ "preserve_context" ] = graph.preserve_context || false;

    json[ "first" ] = json_node( graph.first );

    append_nodes( json, graph );

    return JSON.stringify( json );
}

function append_nodes( json : any, graph : Graph )
{
    json[ "nodes" ] = [];

    graph.nodes.forEach( 
        ( n : NodeType ) => 
        {
            let new_node = json_node( n );

            if( new_node ) { json[ "nodes" ].push( new_node ); }
        }
    );
}

function append_input_data_type( json : any, graph : Graph )
{
    if( graph.input_type )
    {
        json[ "input_type" ] = graph.input_type;
    }
}

function append_initial_context( json : any, graph : Graph )
{
    if( graph.context )
    {
        json[ "context" ] = graph.context;
    }
}

function json_node( node : NodeType | undefined ) : any
{
    if( ! node ) return undefined;

    if( node._variant == NodeTypeVariant.Node )
    {
        return json_node_from_node( node );
    }
    else if( node._variant == NodeTypeVariant.GraphNode )
    {
        return json_node_from_graph_node( node );
    }

    return undefined;
}

function json_node_from_node( node : Node ) : any
{
    let json : any = { 
        "Node" : { 
            id : node.id, 
            output : json_data_to_context( node.output ),
            destination : json_node_destinations( node.destination )
        } 
    };

    return json;
}

function json_node_from_graph_node( node : GraphNode ) : any
{
    let json : any = { 
        "GraphNode" : {
            id : node.id,
            path : node.path,
            node_output : json_data_to_context( node.node_output ),
            node_destination : json_node_destinations( node.node_destination )
        } 
    };

    return json;
}

function json_data_to_context( data : DataToContext | undefined ) : any
{
    if( ! data ) { return undefined; }

    return {
        path : data.path,
        ty : data.ty,
        merge : data.merge
    }
}

function json_node_destinations( destinations : Array<NodeDestination> ) : any
{
    return destinations.map( ( d : NodeDestination ) => json_node_destination( d ) )
}

function json_node_destination( destination : NodeDestination ) : any
{
    return {
        next : json_node_next( destination.next ),
        condition : json_data_comparator( destination.condition )
    };
}

function json_node_next( node_next : NodeNext | undefined ) : any
{
    if( ! node_next ) { return undefined; }


}

function json_data_comparator( data_comparator : DataComparator | undefined ) : any
{
    if( ! data_comparator ) { return undefined; }
}